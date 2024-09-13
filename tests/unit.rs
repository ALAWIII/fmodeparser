use fmodeparser::full_permission::{FullPermission, FullPermissionBuilder};
use fmodeparser::metadata_ext_mode_parser::ModeParseS;
use regex::Regex;
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};

const FILE: &str = "out.txt";

fn check_ocatal_from_decimal(decimal: u32, expected: &str) {
    let mode_oc = format!("{:03o}", decimal);
    println!("{}", mode_oc);
}

#[test]
fn feed_num1() {
    check_ocatal_from_decimal(4488644, "")
}
#[test]
fn feed_num2() {
    check_ocatal_from_decimal(88644, "")
}

#[test]
fn feed_num3() {
    check_ocatal_from_decimal(44644, "")
}
#[test]
fn feed_num4() {
    check_ocatal_from_decimal(100644, "")
}
#[test]
fn feed_num5() {
    check_ocatal_from_decimal(33188, "")
}
#[test]
fn test_file_mode_returned() {
    println!("{}", PathBuf::from(FILE).metadata().unwrap().mode())
}
//=============================================================
#[test]
fn test_file_with_full_permission() {
    let full_permission = FullPermission::new(33188);
    assert_eq!(full_permission.to_string(), "-rw-r--r--");
}

#[test]
fn full_permission_get_mode() {
    let full_permission = FullPermission::new(33188);
    assert_eq!(full_permission.get_mode(), 33188);
}
//===========================================================
#[test]
fn test_trait_on_path() {
    let path = Path::new(FILE);
    let mode = path.metadata().unwrap().mode();
    println!("{}", mode);
    let permission = path
        .metadata()
        .unwrap()
        .convert_permission_to_string()
        .unwrap();
    assert_eq!(permission, "-rw-r--r--");
}

#[test]
fn test_trait_on_pathbuf() {
    let path = PathBuf::from(FILE);
    let mode = path.metadata().unwrap().mode();
    println!("{}", mode);
    let permission = path
        .metadata()
        .unwrap()
        .convert_permission_to_string()
        .unwrap();
    assert_eq!(permission, "-rw-r--r--");
}
//===========================================================
fn valid_regex(patt: &str, case: &str) {
    let reg = Regex::new(patt).unwrap();

    assert!(reg.is_match(case));
}
#[test]
fn file_type_regex() {
    valid_regex("^[-dlcbps]$", "-");
    valid_regex("^[-dlcbps]$", "d");
    valid_regex("^[-dlcbps]$", "l");
    valid_regex("^[-dlcbps]$", "c");
    valid_regex("^[-dlcbps]$", "b");
    valid_regex("^[-dlcbps]$", "p");
    valid_regex("^[-dlcbps]$", "s");
}
#[test]
#[should_panic]
fn file_empty_patt() {
    valid_regex("^[-dlcbps]$", "");
}
#[test]
#[should_panic]

fn more_than_one_t() {
    valid_regex("^[-dlcbps]$", "-dlc");
}
//============================================================
#[test]
fn full_permission_build_nothing() {
    let full_permission = FullPermissionBuilder::new().build();
    assert_eq!(full_permission.to_string(), "----------");
}
#[test]
#[should_panic]
fn full_permission_build_other_panic() {
    FullPermissionBuilder::new().file_type('-').other("");
}
#[test]
#[should_panic]
fn full_permission_build_user_panic() {
    FullPermissionBuilder::new().file_type('-').user("");
}
#[test]
#[should_panic]
fn full_permission_build_group_panic() {
    FullPermissionBuilder::new().file_type('-').group("");
}
#[test]
fn full_permission_build_user_group_other() {
    let full_permission = FullPermissionBuilder::new()
        .user("---")
        .group("---")
        .other("---")
        .build();
    assert_eq!(full_permission.to_string(), "----------");
    assert_eq!(full_permission.get_mode(), 0);
}
#[test]
fn full_permission_build_compare_file() {
    let full_permission = FullPermissionBuilder::new()
        .file_type('-')
        .user("rw-")
        .other("r--")
        .group("r--")
        .build();
    assert_eq!(full_permission.get_mode(), 33188);
}
#[test]
fn full_permission_mode_as_octal() {
    let full_permission = FullPermissionBuilder::new()
        .file_type('-')
        .user("rw-")
        .other("r--")
        .group("r--")
        .build();
    assert_eq!(full_permission.mode_as_octal(), "100644");
}
