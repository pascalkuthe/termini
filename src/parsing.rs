use crate::{get_str_with_offset, Error, Extended, TermInfo, TermInfoData, ValueStorage};
use std::collections::HashMap;
use std::io;
use std::io::Read;

/// magic number octal 0432 for legacy ncurses terminfo
const MAGIC_LEGACY: i16 = 0x11A;
/// magic number octal 01036 for new ncurses terminfo
const MAGIC_32BIT: i16 = 0x21E;

impl TermInfoData {
    /// Create terminfo database by parse byte-array directly
    pub fn parse<R: io::Read>(
        mut reader: R,
        numbers_32bit: bool,
        bool_cnt: u16,
        numbers_cnt: u16,
        string_cnt: u16,
        table_bytes: u16,
        aligned: bool,
    ) -> Result<TermInfoData, Error> {
        let bools = (0..bool_cnt)
            .map(|_| match read_byte(&mut reader) {
                Err(e) => Err(e),
                Ok(1) => Ok(true),
                Ok(_) => Ok(false),
            })
            .collect::<Result<_, _>>()?;

        if bool_cnt % 2 == aligned.into() {
            read_byte(&mut reader)?; // compensate for padding
        }

        let numbers = (0..numbers_cnt)
            .map(|_| {
                if numbers_32bit {
                    read_i32(&mut reader)
                } else {
                    read_i16(&mut reader).map(i32::from)
                }
            })
            .collect::<Result<_, _>>()?;

        let strings: Box<[_]> = (0..string_cnt)
            .map(|_| read_u16(&mut reader))
            .collect::<Result<_, _>>()?;

        for &off in &*strings {
            if matches!(off, 0..=0xfffd if off > table_bytes) {
                return Err(Error::OutOfBoundString {
                    off,
                    table_size: table_bytes,
                });
            }
        }

        let mut str_table = Vec::new();
        let read = reader
            .take(table_bytes.into())
            .read_to_end(&mut str_table)?;
        if read != table_bytes as usize {
            return Err(io::Error::new(io::ErrorKind::Other, "end of file").into());
        }

        Ok(TermInfoData {
            bools,
            numbers,
            strings,
            str_table: str_table.into_boxed_slice(),
        })
    }
}
impl TermInfo {
    /// Create terminfo database by parse byte-array directly
    pub fn parse<R: io::Read>(mut reader: R) -> Result<TermInfo, Error> {
        // read the magic number.
        let magic = read_i16(&mut reader)?;

        let number_32bit = match magic {
            MAGIC_LEGACY => false,
            MAGIC_32BIT => true,
            num => return Err(Error::InvalidMagicNum(num)),
        };

        let names_bytes = read_non_neg_i16(&mut reader)?;
        let bool_count = read_non_neg_i16(&mut reader)?;
        let numbers_count = read_non_neg_i16(&mut reader)?;
        let string_count = read_non_neg_i16(&mut reader)?;
        let string_table_bytes = read_non_neg_i16(&mut reader)?;

        if names_bytes == 0 {
            return Err(Error::NoNames);
        }

        let term_names = read_str(&mut reader, names_bytes - 1)?;
        let mut term_names = term_names.split('|').map(|it| it.trim().to_owned());
        let name = term_names.next().unwrap();
        let mut aliases: Vec<_> = term_names.collect();

        if read_byte(&mut reader)? != b'\0' {
            return Err(Error::NamesMissingNull);
        }

        let data = TermInfoData::parse(
            &mut reader,
            number_32bit,
            bool_count,
            numbers_count,
            string_count,
            string_table_bytes,
            names_bytes % 2 == 0,
        )?;

        let extended =
            try_parse_ext_capabilities(reader, number_32bit, string_table_bytes % 2 == 1)
                .unwrap_or_default();

        let res = TermInfo {
            name,
            description: aliases.pop().unwrap_or_default(),
            aliases,
            data,
            extended,
        };

        Ok(res)
    }
}

fn try_parse_ext_capabilities(
    mut reader: impl io::Read,
    number_32bit: bool,
    unaligned: bool,
) -> Result<Extended, Error> {
    if unaligned {
        read_byte(&mut reader)?; // compensate for padding
    }

    // the term(5) manpage doesn't describe this properly
    // what is calls "the size of the string table" is
    // actually the number of strings in the table (including names),
    // and what it calls the "last offset in the string table" is
    // actually the size in bytes of the string table
    let bool_count = read_non_neg_i16(&mut reader)?;
    let num_count = read_non_neg_i16(&mut reader)?;
    let string_count = read_non_neg_i16(&mut reader)?;
    let num_strings_in_table = read_non_neg_i16(&mut reader)?;
    let table_bytes = read_non_neg_i16(&mut reader)?;
    // debug_assert!(num_strings_in_table >= bool_count + num_count + 2 * string_count);

    // TODO bounds checks
    let data = TermInfoData::parse(
        &mut reader,
        number_32bit,
        bool_count,
        num_count,
        num_strings_in_table,
        table_bytes,
        true,
    )?;

    if string_count as usize >= data.strings.len() {
        return Err(Error::InvalidNames);
    }

    let names_off = data.strings[..string_count as usize]
        .iter()
        .rev()
        .filter_map(|&off| Some(off + data.get_str_at(off)?.len() as u16))
        .max()
        .unwrap_or(0)
        + 1;

    let mut names = data.strings[string_count as usize..].iter().map(|&off| {
        if matches!(off, 0..=0xfffd if off as usize + names_off as usize >= table_bytes as usize) {
            return Some(Err(Error::OutOfBoundString {
                off: off + names_off,
                table_size: table_bytes,
            }));
        }
        let res = get_str_with_offset(&*data.str_table, off, names_off)?.to_owned();
        match String::from_utf8(res) {
            Ok(res) => Some(Ok(res)),
            Err(err) => Some(Err(err.into())),
        }
    });

    let mut capabilities = HashMap::with_capacity((bool_count + num_count + string_count) as usize);

    for (&val, name) in data.bools.iter().zip(&mut names) {
        if let Some(name) = name {
            if val {
                capabilities.insert(name?, ValueStorage::True);
            }
        }
    }

    for (&val, name) in data.numbers.iter().zip(&mut names) {
        if let Some(name) = name {
            if val != 0xffff {
                capabilities.insert(name?, ValueStorage::Number(val));
            }
        }
    }
    for (&val, name) in data.strings.iter().zip(&mut names) {
        if let Some(name) = name {
            if !matches!(val, 0xffff | 0xfffe) {
                capabilities.insert(name?, ValueStorage::String(val));
            }
        }
    }

    let mut str_table = Vec::from(data.str_table);
    str_table.truncate(names_off as usize);

    Ok(Extended {
        capabilities,
        table: str_table.into_boxed_slice(),
    })
}

fn read_i16<R: io::Read>(mut data: R) -> Result<i16, Error> {
    let mut buf = [0; 2];
    data.read_exact(&mut buf)?;
    Ok(i16::from_le_bytes(buf))
}

fn read_u16<R: io::Read>(mut data: R) -> Result<u16, Error> {
    let mut buf = [0; 2];
    data.read_exact(&mut buf)?;
    Ok(u16::from_le_bytes(buf))
}

/// According to the spec, these fields must be >= -1 where -1 means that the
/// feature is not
/// supported. Using 0 instead of -1 works because we skip sections with length
/// 0.
fn read_non_neg_i16(data: impl io::Read) -> Result<u16, Error> {
    match read_i16(data)? {
        n @ 0.. => Ok(n as u16),
        -1 => Ok(0),
        _ => Err(Error::InvalidNames),
    }
}

fn read_i32<R: io::Read>(mut data: R) -> Result<i32, Error> {
    let mut buf = [0; 4];
    data.read_exact(&mut buf)?;
    Ok(i32::from_le_bytes(buf))
}

fn read_str(data: impl io::Read, size: u16) -> Result<String, Error> {
    let mut bytes = Vec::new();
    let read = data.take(size.into()).read_to_end(&mut bytes)?;
    if read != size as usize {
        return Err(io::Error::new(io::ErrorKind::Other, "end of file").into());
    }
    let bytes = String::from_utf8(bytes)?;
    Ok(bytes)
}

fn read_byte(r: &mut impl io::Read) -> io::Result<u8> {
    match r.bytes().next() {
        Some(s) => s,
        None => Err(io::Error::new(io::ErrorKind::Other, "end of file")),
    }
}
