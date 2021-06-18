use std::fmt;

/// Type which returns a Result with a type of T and Error an type of EmptyVecErr
pub type ErrResult<T> = std::result::Result<T, EmptyVecErr>;

/// Error which shows up when no transactions have happened in a Blockchain
#[derive(Debug, Clone)]
pub struct EmptyVecErr;

impl fmt::Display for EmptyVecErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "No transactions have happened!")
    }
}
