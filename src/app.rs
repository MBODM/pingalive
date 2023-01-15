// No need for some code here, to verify name and version from cargo.toml file,
// since cargo will show an error, if name or version contains an empty string.

pub const NAME: &str = env!("CARGO_PKG_NAME");
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const AUTHOR: &str = "MBODM";
pub const DATE: &str = "2023-01-15";
