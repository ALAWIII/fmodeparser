use crate::modeparser::ModeParser;
use crate::permssion_conver_util::*;
use crate::syscategory::{SysCategory, SysCategory::*};
use regex::Regex;
use std::fmt::{Display, Formatter};

#[derive(Debug, Eq, PartialEq)]
pub struct FullPermission {
    mode: u32,
    file_type: String,
    user: SysCategory,
    group: SysCategory,
    other: SysCategory,
}

impl FullPermission {
    pub fn new(mode: u32) -> Self {
        // accepts a decimal number represents a file mode and returns a string of the permission
        // it will automatically convert it to octal and then convert it to a string representation
        let mode_oc = format!("{:06o}", mode);
        if mode_oc.len() != 6 {
            panic!("mode number must be 6 digits long")
        }
        let file_type = file_type_number_to_symbol(&mode_oc[0..=2]);
        let user = digit_to_permission(&mode_oc[3..=3]);
        let group = digit_to_permission(&mode_oc[4..=4]);
        let other = digit_to_permission(&mode_oc[5..=5]);
        Self {
            mode,
            file_type,
            user: User(ModeParser::from(user)),
            group: Group(ModeParser::from(group)),
            other: Other(ModeParser::from(other)),
        }
    }
    pub fn get_file_type(&self) -> &str {
        &self.file_type
    }
    pub fn get_user(&mut self) -> &mut SysCategory {
        &mut self.user
    }
    pub fn get_group(&mut self) -> &mut SysCategory {
        &mut self.group
    }
    pub fn get_other(&mut self) -> &mut SysCategory {
        &mut self.other
    }
    pub fn get_mode(&self) -> u32 {
        self.mode
    }
    pub fn mode_as_octal(&self) -> String {
        // this is not meant to be used to be bypassed within the crate !! only for representation
        format!("{:06o}", self.get_mode())
    }
}
impl Display for FullPermission {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}{}",
            self.file_type, self.user, self.group, self.other
        )
    }
}

pub struct FullPermissionBuilder {
    mode: String,
}
impl FullPermissionBuilder {
    pub fn new() -> Self {
        Self {
            mode: "".to_string(),
        }
    }
    fn common_set_perm(&self, kind: &str, perm: &str) {
        let reg = Regex::new("^[r-][w-][x-]$").unwrap();
        if !reg.is_match(perm) {
            panic!("{kind} permission must be a valid type only one of the options: rwx-")
        }
    }
    pub fn file_type(&mut self, file_type: char) -> &mut Self {
        let reg = Regex::new("^[-dlcbps]$").unwrap();
        if !reg.is_match(&file_type.to_string()) {
            panic!("file type must be a valid type only one of the options: -dlcbps")
        }
        self.mode
            .push_str(&symbol_to_file_type_number(&file_type.to_string()));
        self
    }
    pub fn user(&mut self, user: &str) -> &mut Self {
        self.common_set_perm("user", user);
        self.mode.push_str(&permission_to_digit(user));
        self
    }
    pub fn group(&mut self, group: &str) -> &mut Self {
        self.common_set_perm("group", group);
        self.mode.push_str(&permission_to_digit(group));

        self
    }
    pub fn other(&mut self, other: &str) -> &mut Self {
        self.common_set_perm("other", other);
        self.mode.push_str(&permission_to_digit(other));

        self
    }
    fn get_mode(&self) -> u32 {
        // returns the mode from the octal parsed as integer number
        // example "100644" will be parsed as 0o100644 and then parsed as u32 to get 33188
        // this indeed will be compatible with the usage of the crate functionalities
        u32::from_str_radix(&self.mode, 8).map_or(0, |x| x)
    }
    pub fn build(&self) -> FullPermission {
        FullPermission::new(self.get_mode())
    }
}

impl Default for FullPermissionBuilder {
    fn default() -> Self {
        FullPermissionBuilder::new()
    }
}
impl Display for FullPermissionBuilder {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_mode())
    }
}
