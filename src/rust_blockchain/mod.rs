pub mod block;
pub mod blockchain;
pub mod err;

use std::fmt;

pub struct VersionInfo<'a> {
    version: f32,
    version_name: &'a str,
    patch: u8,
}

#[allow(dead_code)]
impl fmt::Display for VersionInfo<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "rs-blockchain v{}.{} {}",
            self.version, self.patch, self.version_name
        )
    }
}

#[allow(dead_code)]
pub const VERSION_INFO: VersionInfo = VersionInfo {
    version: 0.1,
    version_name: "Fengari",
    patch: 5,
};
