use crate::FullPermission;
use std::error::Error;
use std::fs::Metadata;
use std::os::unix::fs::MetadataExt;

/// this trait is used as an extension on [Metadata](Metadata) to parse the permission of a file.
pub trait PermStrParser {
    /// converts the permission number into a string representation.
    /// # Example
    /// ```
    /// use std::fs::Metadata;
    /// use std::os::unix::fs::MetadataExt;
    /// use std::path::Path;
    /// use fmodeparser::PermStrParser;
    ///
    /// let path = Path::new("out.txt");
    /// let permission = path.convert_permission_to_string()?;
    /// assert_eq!(permission, "-rw-r--r--");
    ///
    /// ```
    fn convert_permission_to_string(&self) -> Result<String, Box<dyn Error>>;
}

impl PermStrParser for Metadata {
    fn convert_permission_to_string(&self) -> Result<String, Box<dyn Error>> {
        let mode = self.mode();
        let permission = FullPermission::new(mode)?;
        Ok(permission.to_string())
    }
}
