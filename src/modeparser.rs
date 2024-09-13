use std::fmt::{Display, Formatter};

#[derive(Debug, Eq, PartialEq)]
pub struct ModeParser {
    read: char,
    write: char,
    execute: char,
}

impl ModeParser {
    fn new() -> Self {
        Self {
            read: '-',
            write: '-',
            execute: '-',
        }
    }
    fn set_read(&mut self, read: &char) {
        self.read = *read;
    }
    fn set_write(&mut self, write: &char) {
        self.write = *write;
    }
    fn set_execute(&mut self, execute: &char) {
        self.execute = *execute;
    }
    fn get_read(&self) -> &char {
        &self.read
    }
    fn get_write(&self) -> &char {
        &self.write
    }
    fn get_execute(&self) -> &char {
        &self.execute
    }
}
impl From<&str> for ModeParser {
    fn from(perm: &str) -> Self {
        Self {
            read: perm.chars().next().unwrap_or('-'),
            write: perm.chars().nth(1).unwrap_or('-'),
            execute: perm.chars().nth(2).unwrap_or('-'),
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
