//! This module is used to store the individual characters and provide a nice
//! representation and methods for complete manipulations!.
//!

use super::permssion_conver_util::permission_to_digit;
use std::fmt::{Display, Formatter};

#[derive(Debug, Eq, PartialEq)]
/// This struct is used to store the individual characters!
///
/// the struct methods are only used by OwnerKind enum !!
///
/// see [`OwnersKind`](crate::category_util::owners::OwnersKind) enum
pub struct ModeParser {
    partial_mode: u32,
    read: char,
    write: char,
    execute: char,
}

impl ModeParser {
    /// this is used after calling setter methods to reset the partial_mode so
    /// that providing consistency when calling [`get_mode`]
    ///
    /// [`get_mode`]: crate::FullPermission::get_mode
    fn reset_partial_mode(&mut self) {
        let digit_of_permission = permission_to_digit(&self.to_string()).parse::<u32>();
        self.set_partial_mode(digit_of_permission.unwrap());
    }
    pub fn set_partial_mode(&mut self, partial_mode: u32) {
        self.partial_mode = partial_mode;
    }
    /// returns the partial mode which corresponds to either one of the three [`owners kind`]
    ///
    /// [`owners kind`]: crate::category_util::owners::OwnersKind
    ///
    pub fn get_partial_mode(&self) -> u32 {
        self.partial_mode
    }
    /// this is used to set the read permission field with either `-` or `r`
    pub fn set_read(&mut self, read: char) {
        self.read = read;
        self.reset_partial_mode();
    }
    /// this is used to set the write permission field with either `-` or `w`
    pub fn set_write(&mut self, write: char) {
        self.write = write;
        self.reset_partial_mode();
    }
    /// this is used to set the execute permission field with either `-` or `x`
    pub fn set_execute(&mut self, execute: char) {
        self.execute = execute;
        self.reset_partial_mode();
    }
    /// this is used to get the read permission field with either `-` or `r`
    pub fn get_read(&self) -> char {
        self.read
    }
    /// this is used to get the write permission field with either `-` or `w`
    pub fn get_write(&self) -> char {
        self.write
    }
    /// this is used to get the execute permission field with either `-` or `x`
    pub fn get_execute(&self) -> char {
        self.execute
    }
}

/// This trait will accept str or String of three characters such that for making
/// new modeparser object
impl From<&str> for ModeParser {
    fn from(perm: &str) -> Self {
        let read = perm.chars().next().unwrap_or('-');
        let write = perm.chars().nth(1).unwrap_or('-');
        let execute = perm.chars().nth(2).unwrap_or('-');
        let mode = permission_to_digit(&format!("{}{}{}", read, write, execute))
            .parse::<u32>()
            .unwrap_or(0);
        Self {
            partial_mode: mode,
            read,
            write,
            execute,
        }
    }
}

impl From<String> for ModeParser {
    fn from(perm: String) -> Self {
        Self::from(perm.as_str())
    }
}

impl Display for ModeParser {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{}", self.read, self.write, self.execute)
    }
}
