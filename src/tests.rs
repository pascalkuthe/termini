use std::fs;

use crate::{BoolCapability, NumberCapability, StringCapability, TermInfo, Value};

#[test]
fn name() {
    let db = TermInfo::from_path("tests/cancer-256color").unwrap();
    assert_eq!("cancer-256color", db.name)
}

#[test]
fn aliases() {
    let db = TermInfo::from_path("tests/st-256color").unwrap();
    assert_eq!(vec!["stterm-256color"], db.aliases)
}

#[test]
fn description() {
    let db = TermInfo::from_path("tests/cancer-256color").unwrap();
    assert_eq!("terminal cancer with 256 colors", db.description)
}

#[test]
fn standard() {
    let db = TermInfo::from_path("tests/st-256color").unwrap();
    assert_eq!(Some(80), db.number_cap(NumberCapability::Columns));
    assert!(db.flag_cap(BoolCapability::AutoRightMargin));
    assert!(!db.flag_cap(BoolCapability::AutoLeftMargin));
    assert_eq!(
        "\r",
        db.utf8_string_cap(StringCapability::CarriageReturn)
            .unwrap()
    );
}

#[test]
fn extended() {
    let db = TermInfo::from_path("tests/cancer-256color").unwrap();
    assert_eq!(Some(Value::True), db.extended_cap("Ts"));
    assert_eq!(Some(Value::True), db.extended_cap("AX"));
    assert_eq!(Some(Value::Utf8String("\u{1b}[2 q")), db.extended_cap("Se"));
}

#[test]
fn bigger_numbers() {
    let db = TermInfo::from_path("tests/xterm-256color").unwrap();
    assert_eq!("xterm-256color", db.name)
}

#[test]
fn alacritty_extended_underculr() {
    let db = TermInfo::from_path("tests/alacritty").unwrap();
    assert_eq!(
        Some(Value::Utf8String("\u{1b}[4:%p1%dm")),
        db.extended_cap("Smulx")
    );
}

#[test]
fn kitty_extended_underculr() {
    let db = TermInfo::from_path("tests/xterm-kitty").unwrap();
    assert_eq!(Some(Value::True), db.extended_cap("Su"));
}

#[test]
fn crash() {
    let data: &[u8] = &[
        26, 1, 29, 0, 1, 1, 0, 0, 0, 1, 43, 6, 12, 12, 244, 131, 162, 131, 124, 35, 120, 124, 0, 0,
        0, 27, 0, 0, 0, 12, 27, 12, 0, 0, 0, 0, 0, 12, 27, 12,
    ];
    let _ = TermInfo::parse(data);
}

#[test]
fn test_parse() {
    for f in fs::read_dir("tests/").unwrap() {
        let _ = TermInfo::from_path(f.unwrap().path()).unwrap();
    }
}
