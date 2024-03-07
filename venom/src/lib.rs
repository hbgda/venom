pub mod macros;
pub mod game;
pub mod hash;
pub mod utils;
pub mod patterns;

pub mod windows {
    pub use windows::{
        s,
        Win32::{
            UI::{
                WindowsAndMessaging::*,
                Input::KeyboardAndMouse::*
            },
            Foundation::*,
            System::{
                SystemServices::*,
                LibraryLoader::*,
            }
        }
    };
}

pub use {
    once_cell::sync::Lazy
};

pub struct ModInfo {
    title: &'static str,
    version: &'static str,
    author: &'static str
}