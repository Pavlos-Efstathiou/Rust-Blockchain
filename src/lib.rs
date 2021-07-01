pub mod rust_blockchain;

use std::fmt;

#[derive(Debug, Copy, Clone)]
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
            "rs_blockchain v{}.{} {}",
            self.version, self.patch, self.version_name
        )
    }
}

#[allow(dead_code)]
pub const VERSION_INFO: VersionInfo = VersionInfo {
    version: 0.2,
    version_name: "Alma",
    patch: 3,
};

pub fn remove_non_digits(string: &str) -> Result<u32, std::num::ParseIntError> {
    let re = regex::Regex::new(r"(\D)+").unwrap();
    let applied_regex = re.replace_all(string, "");

    applied_regex.parse::<u32>()
}

#[macro_export]
macro_rules! add_transaction {
    ($block:expr , $sender:expr => $receiver:expr , $amount:expr $(,)?) => {{
        let string_json = format!(
            "{{\n\t\"sender\": {:?},\n\t\"receiver\": {:?},\n\t\"amount\": {:?},\n}}",
            $sender, $receiver, $amount
        );
        $block.unconfirmed_transactions.push(string_json);
    }};
}
