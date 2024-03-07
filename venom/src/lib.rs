pub mod macros;

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

pub struct ModInfo {
    title: &'static str,
    version: &'static str,
    author: &'static str
}