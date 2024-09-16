//! This crate is a library for parsing file permission in Unix-like system .\
//! It provides a way to parse and manipulate file permissions in a Unix-like system.\
//! It is useful for working with file permissions in Rust programs.\
//! It is designed to be easy to use and provides a simple API for working with file permissions.\
//! It provides a trait extension for the `std::fs::Metadata` type that allows you to directly call\
//! a single method to the file permission from the metadata.
//!# Permission formats and file kinds representation
//! the file permission its octal mode consists of 6-digit number
//! the first three digits are for the file kind:
//! <div align="center">
//!     <h1>file kinds</h1>
//! </div>
//! <div align="center">
//!     <p>
//!
//!
//!     | digits | shortcut usage |      file kind   |
//!     |--------|----------------|------------------|
//!     | 100    |       -        |   regular file   |
//!     | 120    |       l        |       symlink    |
//!     | 020    |       c        | character device |
//!     | 060    |       b        |    block device  |
//!     | 010    |       p        |       fifo       |
//!     | 140    |       s        |       socket     |
//!     | 040    |       d        |      directory   |
//!  </p>
//!</div>              
//! the next three digits are for the user permission where each digit represents
//! a permission from three characters:
//!
//! (read, write, execute, -) (r,w,x, -).
//!
//! <div align="center">
//!     <p>
//!
//!    | digits | permission |
//!    |--------|------------|
//!    |    0   |     ---    |
//!    |    1   |     --x    |
//!    |    2   |     -w-    |
//!    |    3   |     -wx    |
//!    |    4   |     r--    |
//!    |    5   |     r-x    |
//!    |    6   |     rw-    |
//!    |    7   |     rwx    |
//!   </p>
//! </div>
//!
//! if we have a file with permission `33188 decimal` = `100644 octal` = `-rw-r--r--`
//! such that :
//!
//!    1)  -  :  file type
//!    2) rw- :  user permission
//!    3) r-- :  group permission
//!    4) r-- :  other permission
//!
//! # Examples
//! Import the `ModeParseS` trait which uses FullPermission struct under the hood
//! for which its method will be used as an extension for the `fs::Metadata` type.\
//! It's the safest way to use this API!
//! ## Extension on `Metadata` type
//! In most times you will use it like this:
//!```rust
//! use std::fs::Metadata;
//! use std::path::Path;
//! use fmodeparser::FullPermissionError;
//! use fmodeparser::PermStrParser;
//!
//!fn main() -> Result<(), FullPermissionError> {
//!    
//!    let path = Path::new("out.txt");
//!    let metadata = path.metadata()?;
//!    let permission = metadata.convert_permission_to_string()?;
//!    assert_eq!(permission, "-rw-r--r--");
//!    Ok(())
//! }
//! ```
//! ## Using `FullPermission` struct
//! Or you can use the `FullPermission` struct for manipulating the permission before asserting it
//! to a file :
//!```rust
//! use std::path::Path;
//! use fmodeparser::FullPermission;
//! use fmodeparser::FullPermissionError;
//! use std::os::unix::fs::MetadataExt;
//! use std::error::Error;
//!
//! fn main() -> Result<(), Box<dyn Error>> {
//!    
//! let path = Path::new("out.txt");
//!    let mode_number = path.metadata().unwrap().mode();
//!    // only accepts decimal positive number and it will be converted to octal number internally.
//!    // it uses the `format!("{:06o}", mode_number)` to convert it to octal number.
//!    // if the length of number converted is longer than 6 digits it will fail to pass and
//!    // returns error result, which means its not a valid permission number.
//!    let mut full_permission = FullPermission::new(mode_number)?;
//!
//!    let permission = full_permission.to_string();
//!    assert_eq!(permission, "-rw-r--r--");
//!
//!    // returning the permission as Octal number string.
//!    let mode_as_octal = full_permission.mode_as_octal();
//!    assert_eq!(mode_as_octal, "100644");
//!    // returning the permission as decimal number.
//!    let mode = full_permission.get_mode();
//!    assert_eq!(mode, mode_number);
//!    // returning the file type as char.
//!    let file_type = full_permission.get_file_type();
//!    assert_eq!(file_type, '-');
//!
//!    // returns the categories of owners.
//!    let user = full_permission.get_user();
//!    let group = full_permission.get_group();
//!    let other = full_permission.get_other();
//!    
//!     Ok(())
//! }
//! ```
//! ## Using `FullPermissionBuilder` struct
//! You can also use the `FullPermissionBuilder` struct to build the permission
//!
//! This will return a `FullPermission` object !
//!
//! ```
//! use fmodeparser::FullPermissionBuilder;
//! use fmodeparser::FullPermissionError;
//! use std::error::Error;
//!
//! fn main() -> Result<(), Box<dyn Error>> {
//!
//!    let mut full_permission = FullPermissionBuilder::new()
//!                             .file_type('-')
//!                             .user("rw-")
//!                             .group("r--")
//!                             .other("r--")
//!                             .build()?;
//!     assert_eq!(full_permission.to_string(), "-rw-r--r--");
//!     assert_eq!(full_permission.get_mode(), 33188);
//!     assert_eq!(full_permission.mode_as_octal(), "100644");
//!     Ok(())
//! }
//! ```

mod category_util;
mod errors;
mod full_permission;
mod metadata_ext_mode_parser;
use category_util::modeparser::ModeParser;
pub use errors::permission_error::FullPermissionError;
pub use full_permission::{FullPermission, FullPermissionBuilder};
pub use metadata_ext_mode_parser::PermStrParser;
