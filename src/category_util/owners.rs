use crate::ModeParser;
use std::cell::RefCell;
use std::fmt::{Display, Formatter};
use OwnersKind::*;

#[derive(Debug, Eq, PartialEq)]
/// this is a wrapper that encapsulate three kinds of owners and provide a nice interface
/// to call the methods
pub enum OwnersKind {
    User(RefCell<ModeParser>),
    Group(RefCell<ModeParser>),
    Other(RefCell<ModeParser>),
}
impl OwnersKind {
    /// returns the owner kind that will be used to access the mode parser methods.
    ///
    /// this is intended for internal usage not the end user .
    fn get_mode_parser(&self) -> &RefCell<ModeParser> {
        match self {
            User(user) => user,
            Group(group) => group,
            Other(other) => other,
        }
    }
    /// returns the character that represents the read permission of the owner either **`-`** or **`r`**.
    pub fn get_read(&self) -> char {
        self.get_mode_parser().borrow().get_read()
    }
    /// returns the character that represents the write permission of the owner either **`-`** or **`w`**.
    pub fn get_write(&mut self) -> char {
        self.get_mode_parser().borrow().get_write()
    }
    /// returns the character that represents the execute permission of the owner either **`-`** or **`x`**.
    pub fn get_execute(&mut self) -> char {
        self.get_mode_parser().borrow().get_execute()
    }
    /// assigns a new read permission to the owner.
    pub fn set_read(&self, read: char) {
        self.get_mode_parser().borrow_mut().set_read(read)
    }
    /// assigns a new write permission to the owner.
    pub fn set_write(&self, write: char) {
        self.get_mode_parser().borrow_mut().set_write(write)
    }
    /// assigns a new execute permission to the owner.
    pub fn set_execute(&self, execute: char) {
        self.get_mode_parser().borrow_mut().set_execute(execute)
    }
    /// returns the partial mode of the owner.
    pub fn get_partial_mode(&self) -> u32 {
        self.get_mode_parser().borrow().get_partial_mode()
    }
}
impl Display for OwnersKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                User(user) => user,
                Group(group) => group,
                Other(other) => other,
            }
            .borrow()
        )
    }
}
