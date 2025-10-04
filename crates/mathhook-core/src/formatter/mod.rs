mod latex;
mod simple;
mod wolfram;

pub use latex::*;
pub use simple::*;
pub use wolfram::*;

pub struct Formatter {}

impl Formatter {
    pub fn new() -> Self {
        Self {}
    }
}
