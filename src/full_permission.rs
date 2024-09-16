use crate::category_util::owners::{OwnersKind, OwnersKind::*};
use crate::category_util::permssion_conver_util::*;
use crate::FullPermissionError;
use crate::ModeParser;
use regex::Regex;
use std::cell::RefCell;
use std::fmt::{Display, Formatter};
#[derive(Debug, Eq, PartialEq)]
/// a struct that represents a file permission for the file kind and the three owner kinds
///
/// allows easily access, modify and convert the permission to a string representation
///
/// see the example for more details. [new](FullPermission::new)
pub struct FullPermission {
    file_type: char,
    user: OwnersKind,
    group: OwnersKind,
    other: OwnersKind,
}

impl FullPermission {
    /// accepts valid decimal digit represents the file permission.
    ///
    /// returns either `instance` of the struct or an `FullPermissionError` Result.
    ///
    /// # Example
    ///```
    /// use fmodeparser::FullPermission;
    ///
    /// let mode_number = 33188;
    /// let full_permission = FullPermission::new(mode_number)?;
    /// ```
    ///
    pub fn new(mode: u32) -> Result<Self, FullPermissionError> {
        // accepts a decimal number represents a file mode and returns a string of the permission
        // it will automatically convert it to octal and then convert it to a string representation
        let mode_oc = format!("{:06o}", mode);
        if mode_oc.len() != 6 {
            return Err(FullPermissionError::new(format!("the mode that was provided is \
            not valid 6 digit decimal number that can be parsed correctly into octal mode :{mode_oc}")));
        }
        let file_type = file_type_number_to_symbol(&mode_oc[0..=2]);
        let user = digit_to_permission(&mode_oc[3..=3]); // `---` as an example
        let group = digit_to_permission(&mode_oc[4..=4]);
        let other = digit_to_permission(&mode_oc[5..=5]);
        Ok(Self {
            file_type,
            user: User(RefCell::new(ModeParser::from(user))),
            group: Group(RefCell::new(ModeParser::from(group))),
            other: Other(RefCell::new(ModeParser::from(other))),
        })
    }
    /// returns character representing the file type. example: `-` for a regular file
    pub fn get_file_type(&self) -> char {
        self.file_type
    }
    /// returns the user owner that gives you the full authority to modify this owner kind.
    pub fn get_user(&mut self) -> &mut OwnersKind {
        &mut self.user
    }

    /// returns the group owner that gives you the full authority to modify this owner kind.
    pub fn get_group(&mut self) -> &mut OwnersKind {
        &mut self.group
    }
    /// returns the other owner that gives you the full authority to modify this owner kind.
    pub fn get_other(&mut self) -> &mut OwnersKind {
        &mut self.other
    }
    /// returns the mode of the file converted from octal to decimal.
    ///
    /// Also it can be reused again to construct a new [`FullPermission`](FullPermission) object.
    pub fn get_mode(&mut self) -> u32 {
        // get the mode of the file as an octal number then converts it to a decimal number
        // so that ,it can re passed again to a new FullPermission object
        let num = symbol_to_file_type_number(self.get_file_type());
        let user = self.user.get_partial_mode(); // as an octal number
        let group = self.group.get_partial_mode(); // as an octal number
        let other = self.other.get_partial_mode(); // as an octal number

        u32::from_str_radix(&format!("{num}{user}{group}{other}"), 8).unwrap_or(0)
    }
    /// returns the mode as an octal number string.
    ///
    /// this is not meant to be used to be bypassed within the crate !! only for representation.
    pub fn mode_as_octal(&mut self) -> String {
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

/// allows to construct a `FullPermission` object from a string representation of the permission.
///
/// # Example
///
/// ```
/// use fmodeparser::FullPermissionBuilder;
/// use fmodeparser::FullPermissionError;
/// use std::error::Error;
///
///
/// let mut permission = FullPermissionBuilder::new()
///                          .file_type('-')
///                          .user("rw-")
///                          .group("r--")
///                          .other("r--")
///                          .build()?;
///
///  assert_eq!(permission.to_string(), "-rw-r--r--");
///  assert_eq!(permission.get_mode(), 33188);
///  assert_eq!(permission.mode_as_octal(), "100644");
///  Ok(())
/// ```

pub struct FullPermissionBuilder {
    mode: String,
}
impl FullPermissionBuilder {
    pub fn new() -> Self {
        Self {
            mode: "".to_string(),
        }
    }
    /// helper method for setting a user,group,other permissions.
    fn common_set_perm(&self, kind: &str, perm: &str) {
        let reg = Regex::new("^[r-][w-][x-]$").unwrap();
        if !reg.is_match(perm) {
            panic!("{kind} permission must be a valid type only one of the options: rwx-")
        }
    }
    /// accepts only a single character: `(-, d,l,c,b,p,s)`.
    ///
    /// # Panics
    /// if only provided an invalid character!
    pub fn file_type(&mut self, file_type: char) -> &mut Self {
        let reg = Regex::new("^[-dlcbps]$").unwrap();
        if !reg.is_match(&file_type.to_string()) {
            panic!("file type must be a valid type only one of the options: -dlcbps")
        }
        self.mode.push_str(&symbol_to_file_type_number(file_type));
        self
    }
    /// accepts 3-character string representing the permission. examples : `(---, rwx, r--)`
    ///
    /// # Panics
    /// the permission must follow the regular expression `^[r-][w-][x-]$`
    pub fn user(&mut self, user: &str) -> &mut Self {
        self.common_set_perm("user", user);
        self.mode.push_str(&permission_to_digit(user));
        self
    }
    /// accepts 3-character string representing the permission. examples : `(---, rwx, r--)`
    ///
    /// # Panics
    /// the permission must follow the regular expression `^[r-][w-][x-]$`
    pub fn group(&mut self, group: &str) -> &mut Self {
        self.common_set_perm("group", group);
        self.mode.push_str(&permission_to_digit(group));

        self
    }
    /// accepts 3-character string representing the permission. examples : `(---, rwx, r--)`
    ///
    /// # Panics
    /// the permission must follow the regular expression `^[r-][w-][x-]$`
    pub fn other(&mut self, other: &str) -> &mut Self {
        self.common_set_perm("other", other);
        self.mode.push_str(&permission_to_digit(other));

        self
    }
    fn get_mode(&self) -> u32 {
        // returns the mode from the octal parsed as integer number
        // example "100644" will be parsed as 0o100644 and then parsed as u32 to get 33188
        // this indeed will be compatible with the usage of the crate functionalities
        u32::from_str_radix(&self.mode, 8).unwrap_or(0)
    }
    /// returns either [`FullPermission`](FullPermission) object or [`FullPermissionError`](FullPermissionError) error .
    pub fn build(&self) -> Result<FullPermission, FullPermissionError> {
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
