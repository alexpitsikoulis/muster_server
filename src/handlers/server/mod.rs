mod create;
mod delete;
mod update;

pub use create::*;
pub use delete::*;
pub use update::*;

pub const BASE_PATH: &str = "/servers";
