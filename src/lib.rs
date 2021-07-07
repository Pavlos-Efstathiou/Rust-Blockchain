pub mod rust_blockchain;
use std::fmt;

pub type ArgValue = Option<u32>;

pub trait Unwrap {
    fn arg_unwrap(&self) -> u32;
}

impl Unwrap for ArgValue {
    fn arg_unwrap(&self) -> u32 {
        self.unwrap_or(0)
    }
}

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
pub static VERSION_INFO: VersionInfo = VersionInfo {
    version: 0.2,
    version_name: "Alma",
    patch: 7,
};

/// Removes all characters that are not digits from an &str
pub fn remove_non_digits(arg: &str) -> ArgValue {
    let num = arg
        .chars()
        .filter(|c| c.is_digit(10))
        .collect::<String>()
        .parse::<u32>()
        .unwrap_or(0);

    match num {
        0 => None,
        _ => Some(num),
    }
}

/// Adds a transaction to the provided Blockchain
#[macro_export]
macro_rules! add_transaction {
    () => {};
    ($($blockchain:expr , $sender:expr => $receiver:expr , $amount:expr),+ $(,)?) => {{
        $(
            let string_json = serde_json::to_string_pretty(
                &serde_json::json!(
                    {
                        "sender": $sender,
                        "receiver": $receiver,
                        "amount": $amount,
                    }
                )
            ).unwrap_or_else(|_| String::from(""));
            $blockchain.unconfirmed_transactions.push(string_json);
        )+
    }};
}
