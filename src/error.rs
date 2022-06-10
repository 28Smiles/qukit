use alloc::string::String;
use core::fmt;

pub type Result<T> = core::result::Result<T, QuantumError>;

#[derive(Debug, Clone)]
pub struct QuantumError(pub(crate) String);

impl fmt::Display for QuantumError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
