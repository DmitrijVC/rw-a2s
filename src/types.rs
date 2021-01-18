use std::fmt::{Display, Formatter, Result};

// --- Unstable
//  pub const BOOL_B: TypeId = TypeId::of::<&bool>();  // 15985373975069005502
//  pub const STR: TypeId = TypeId::of::<&str>();  // 9147559743429524724
//  pub const U32_B: TypeId = TypeId::of::<&u32>();  // 12530009511461549899


pub enum Bool {
    TRUE,
    FALSE,
} impl Display for Bool {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{}",
            match *self {
                Bool::TRUE => "1",
                Bool::FALSE => "0",
            }
        )
    }
}
