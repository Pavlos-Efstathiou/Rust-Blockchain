use std::error::Error;
use std::fmt;

/// Type which returns a Result with a type of T and Error an type of EmptyVecErr
pub type ErrResult<T> = std::result::Result<T, Box<dyn Error>>;

/// Error which shows up when no transactions have happened in a Blockchain
#[derive(Debug, Clone)]
pub struct EmptyVecErr;

impl Error for EmptyVecErr {}

impl fmt::Display for EmptyVecErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error: No transactions have happened!")
    }
}
