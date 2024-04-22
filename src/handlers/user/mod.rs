mod confirm;
mod delete;
mod login;
mod signup;
mod update;

pub use confirm::*;
pub use delete::*;
pub use login::*;
pub use signup::*;
pub use update::*;

pub const BASE_PATH: &str = "/users";
