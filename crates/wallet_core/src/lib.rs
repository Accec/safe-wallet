pub mod application;
pub mod chains;
pub mod domains;
pub mod error;
pub mod models;
pub mod protocol;
pub use protocol::qr;
pub mod storage;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod architecture_tests;
