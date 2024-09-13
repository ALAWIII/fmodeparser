use crate::full_permission::FullPermission;
use std::error::Error;
use std::fs::Metadata;
use std::os::unix::fs::MetadataExt;

pub trait ModeParseS {
    // must return a SysCategory
    fn convert_permission_to_string(&self) -> Result<String, Box<dyn Error>>;
}
impl ModeParseS for Metadata {
    fn convert_permission_to_string(&self) -> Result<String, Box<dyn Error>> {
        let mode = self.mode();
        let permission = FullPermission::new(mode);
        Ok(permission.to_string())
    }
}
