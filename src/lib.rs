#![forbid(unsafe_code)]

use std::collections::HashMap;
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::string::FromUtf8Error;
use std::{env, io};

pub use crate::capabilities::{BoolCapability, NumberCapability, StringCapability};

mod capabilities;
mod parsing;

#[cfg(test)]
mod tests;

/// Terminfo database information
#[derive(Debug, Default)]
pub struct TermInfo {
    pub name: String,
    pub description: String,
    pub aliases: Vec<String>,
    data: TermInfoData,
    extended: Extended,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Value<'a> {
    True,
    RawString(&'a [u8]),
    Utf8String(&'a str),
    Number(i32),
}

#[derive(Debug)]
pub enum Error {
    NotFound,
    InvalidMagicNum(i16),
    Io(io::Error),
    NoNames,
    NamesMissingNull,
    StringMissingNull,
    OutOfBoundString { off: u16, table_size: u16 },
    InvalidUtf8(FromUtf8Error),
    InvalidNames,
}

impl std::error::Error for Error {
    fn source(&self) -> std::option::Option<&(dyn std::error::Error + 'static)> {
        #[allow(deprecated)]
        match self {
            Error::Io(source) => Some(source as _),
            Error::InvalidUtf8(source) => Some(source as _),
            _ => None,
        }
    }
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::NotFound {} => write!(f, "Terminfo file not found"),
            Error::InvalidMagicNum(num) => write!(f, "bad magic number {num} in terminfo header",),
            Error::Io(_) => write!(f, "reading terminfo failed",),
            Error::NoNames => write!(f, "no names exposed, need at least one"),
            Error::NamesMissingNull => write!(f, "names table missing NUL terminator"),
            Error::StringMissingNull => {
                write!(f, "string table missing NUL terminator")
            }
            Error::OutOfBoundString { off, table_size } => write!(
                f,
                "string offset {off} outside data table (size: {table_size})",
            ),
            Error::InvalidUtf8(_) => write!(f, "terminfo string is invalid ASCII/UTF-8",),
            Error::InvalidNames {} => write!(f, "no names exposed, need at least one"),
        }
    }
}
impl std::convert::From<io::Error> for Error {
    #[allow(deprecated)]
    fn from(source: io::Error) -> Self {
        Error::Io(source)
    }
}
impl std::convert::From<FromUtf8Error> for Error {
    #[allow(deprecated)]
    fn from(source: FromUtf8Error) -> Self {
        Error::InvalidUtf8(source)
    }
}

#[derive(Debug, Default)]
struct Extended {
    capabilities: HashMap<String, ValueStorage>,
    table: Box<[u8]>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum ValueStorage {
    True,
    String(u16),
    Number(i32),
}

#[derive(Debug, Default)]
struct TermInfoData {
    bools: Box<[bool]>,
    numbers: Box<[i32]>,
    strings: Box<[u16]>,
    str_table: Box<[u8]>,
}

/// gets a string from the `str_table` starting at `start`.
///
/// **panics** if start is larger than the size of string table
fn get_str_with_offset(table: &[u8], start: u16, offset: u16) -> Option<&[u8]> {
    //         // non-entry
    //         // undocumented: `FFFE` indicates cap@, which means the capability
    //         // is not present
    if matches!(start, 0xffff | 0xfffe) {
        return None;
    }
    let table = &table[(start + offset) as usize..];
    let res = table
        .iter()
        .position(|&c| c == b'\0')
        .map_or(table, |end| &table[..end]);
    Some(res)
}

impl TermInfoData {
    /// gets a string from the `str_table` starting at `start`.
    ///
    /// **panics** if start is larger than the size of string table
    fn get_str_at(&self, start: u16) -> Option<&[u8]> {
        get_str_with_offset(&self.str_table, start, 0)
    }
}

impl TermInfo {
    /// Returns the string value for the capability
    ///
    /// # Arguments
    /// * `cap` - string capability
    ///
    /// # Example
    /// ```
    /// use termini::TermInfo;
    /// use termini::StringCapability;
    ///
    /// if let Ok(info) = TermInfo::from_env() {
    ///     println!("{:?}", info.raw_string_cap(StringCapability::Bell));
    /// }
    /// ```
    pub fn raw_string_cap(&self, cap: StringCapability) -> Option<&[u8]> {
        let off = *self.data.strings.get(cap as usize)?;
        self.data.get_str_at(off)
    }

    /// Returns the string value for the capability.
    /// If the capability is invalid UTF-8 (ASCII) or doesn't exists `None` is returned
    ///
    /// # Arguments
    /// * `cap` - string capability
    ///
    /// # Example
    /// ```
    /// use termini::TermInfo;
    /// use termini::StringCapability;
    ///
    /// if let Ok(info) = TermInfo::from_env() {
    ///     println!("{:?}", info.utf8_string_cap(StringCapability::Bell));
    /// }
    /// ```
    pub fn utf8_string_cap(&self, cap: StringCapability) -> Option<&str> {
        let off = *self.data.strings.get(cap as usize)?;
        std::str::from_utf8(self.data.get_str_at(off)?).ok()
    }

    /// Returns the number value for the capability
    ///
    /// # Arguments
    /// * `cap` - number capability
    ///
    /// # Example
    /// ```
    /// use termini::TermInfo;
    /// use termini::NumberCapability;
    ///
    /// if let Ok(info) = TermInfo::from_env() {
    ///     println!("{:?}", info.number_cap(NumberCapability::MaxColors));
    /// }
    /// ```
    pub fn number_cap(&self, cap: NumberCapability) -> Option<i32> {
        self.data
            .numbers
            .get(cap as usize)
            .copied()
            .filter(|&val| val != 0xffff)
    }

    /// Returns the bool value for the capability, if the capability is not present,
    /// `false` is returned
    ///
    /// # Arguments
    /// * `cap` - bool capability
    ///
    /// # Example
    /// ```
    /// use termini::TermInfo;
    /// use termini::BoolCapability;
    ///
    /// if let Ok(info) = TermInfo::from_env() {
    ///     println!("{:?}", info.flag_cap(BoolCapability::AutoLeftMargin));
    /// }
    /// ```
    pub fn flag_cap(&self, cap: BoolCapability) -> bool {
        self.data.bools.get(cap as usize).copied().unwrap_or(false)
    }

    pub fn extended_cap(&self, name: &str) -> Option<Value> {
        let res = match *self.extended.capabilities.get(name)? {
            ValueStorage::True => Value::True,
            ValueStorage::String(off) => {
                let raw = get_str_with_offset(&self.extended.table, off, 0)?;
                match std::str::from_utf8(raw) {
                    Ok(res) => Value::Utf8String(res),
                    Err(_) => Value::RawString(raw),
                }
            }
            ValueStorage::Number(val) => Value::Number(val),
        };
        Some(res)
    }

    /// Create TermInfo database, using TERM environment var.
    pub fn from_env() -> Result<Self, Error> {
        if let Ok(term) = std::env::var("TERM") {
            TermInfo::from_name(term.as_str())
        } else {
            Err(Error::NotFound)
        }
    }

    /// Create TermInfo database for the given name
    pub fn from_name(name: &str) -> Result<Self, Error> {
        let first = name.chars().next().ok_or(Error::NotFound)?;

        // See https://manpages.debian.org/buster/ncurses-bin/TermInfo.5.en.html#Fetching_Compiled_Descriptions
        let mut search = Vec::<PathBuf>::new();

        if let Some(dir) = env::var_os("TERMINFO") {
            search.push(dir.into());
        } else if let Some(mut home) = home::home_dir() {
            home.push(".terminfo");
            search.push(home);
        }

        if let Ok(dirs) = env::var("TERMINFO_DIRS") {
            for dir in dirs.split(':') {
                search.push(dir.into());
            }
        }

        // handle non-FHS systems like Termux
        if let Ok(prefix) = env::var("PREFIX") {
            let path = Path::new(&prefix);
            search.push(path.join("etc/terminfo"));
            search.push(path.join("lib/terminfo"));
            search.push(path.join("share/terminfo"));
        }

        search.push("/etc/terminfo".into());
        search.push("/lib/terminfo".into());
        search.push("/usr/share/terminfo".into());
        search.push("/boot/system/data/terminfo".into());

        for path in search {
            if fs::metadata(&path).is_err() {
                continue;
            }

            // Check standard location.
            {
                let mut path = path.clone();
                path.push(first.to_string());
                path.push(name);

                if fs::metadata(&path).is_ok() {
                    return Self::from_path(&path);
                }
            }

            // Check non-standard location.
            let mut path = path.clone();
            path.push(format!("{:x}", first as usize));
            path.push(name);

            if fs::metadata(&path).is_ok() {
                return Self::from_path(&path);
            }
        }

        Err(Error::NotFound)
    }

    /// Rad a TermInfo database from a given path
    pub fn from_path(file: impl AsRef<Path>) -> Result<TermInfo, Error> {
        TermInfo::parse(File::open(file)?)
    }
}
