use std::fmt::{Display, Formatter, Result};


pub struct Bool(pub bool);

impl Display for Bool {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.0 as u8)
    }
}
