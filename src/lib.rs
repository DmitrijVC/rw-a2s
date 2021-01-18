// ToDo add documentation
// ToDo add tests

#[macro_use] extern crate lazy_static;

#[allow(dead_code)] pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub mod net;
pub mod errors;
pub mod types;
