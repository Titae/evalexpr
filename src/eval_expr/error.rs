
use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub struct EvalExprError;

impl fmt::Display for EvalExprError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid expression")
    }
}

impl Error for EvalExprError {
    fn description(&self) -> &str {
        "Invalid expression"
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}
