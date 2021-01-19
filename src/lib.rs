// ToDo add documentation
// ToDo add tests

#[macro_use] extern crate lazy_static;

#[allow(dead_code)] pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");

mod net;
mod errors;
mod types;

#[allow(unused_imports)]
pub use crate::net::server::*;

#[allow(unused_imports)]
pub use crate::net::client::*;

#[allow(unused_imports)]
pub use crate::net::client::filters::*;

#[allow(unused_imports)]
pub use crate::types::*;

#[allow(unused_imports)]
pub use crate::errors::*;
