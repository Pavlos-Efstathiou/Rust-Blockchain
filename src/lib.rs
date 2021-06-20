pub mod rust_blockchain;

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
    version: 0.2,
    version_name: "Alma",
    patch: 1,
};

pub fn remove_non_digits(string: &str) -> u32 {
    let re = regex::Regex::new(r"(\D)+").unwrap();
    let applied_regex = re.replace_all(string, "");

    applied_regex.parse::<u32>().unwrap()
}
