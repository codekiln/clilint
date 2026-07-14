pub mod assessment;
pub mod cli;
pub mod engine;
pub mod model;
pub mod package;
pub mod report;
pub mod runner;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
