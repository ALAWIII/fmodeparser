use crate::modeparser::ModeParser;
use std::fmt::{Display, Formatter};
use SysCategory::*;

#[derive(Debug, Eq, PartialEq)]
pub enum SysCategory {
    User(ModeParser),
    Group(ModeParser),
    Other(ModeParser),
}
impl SysCategory {
    pub fn get_mode_parser(&mut self) -> &mut ModeParser {
        match self {
            User(user) => user,
            Group(group) => group,
            Other(other) => other,
        }
    }
}
impl Display for SysCategory {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                User(user) => user,
                Group(group) => group,
                Other(other) => other,
            }
        )
    }
}
