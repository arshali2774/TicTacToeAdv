// importing player and tabs modules into models module
mod player;
mod tabs;
// importing all items from player and tabs modules to be accessible directly from models module
pub use player::*;
pub use tabs::*;
