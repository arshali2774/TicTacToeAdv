// this is a module
// we will be nesting different modules here

// choose player module
mod choose_player;
// expose everything from choose_player module and make it public so that it can be used in main.rs
pub use choose_player::*;
