mod confirm;
mod login;
mod patch;
mod signup;
mod update;

pub use confirm::*;
pub use login::*;
pub use patch::*;
pub use signup::*;
pub use update::*;

pub const BASE_PATH: &str = "/users";
