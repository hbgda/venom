pub mod macros;
pub mod game;
pub mod hash;
pub mod utils;
pub mod patterns;
pub mod scaleform;
pub mod menu;

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
    once_cell::sync::Lazy,
    paste::paste,
    retour::*,
};
use std::ffi::CStr;

pub struct ModInfo {
    pub title: &'static str,
    pub description: &'static str,
    pub version: &'static str,
    pub author: &'static str
}

#[repr(C)]
pub struct CModInfo {
    title: *const u8,
    description: *const u8,
    version: *const u8,
    author: *const u8
}

impl From<&ModInfo> for CModInfo {
    fn from(value: &ModInfo) -> Self {
        CModInfo {
            title: value.title.as_ptr(),
            description: value.description.as_ptr(),
            version: value.version.as_ptr(),
            author: value.author.as_ptr()
        }   
    }
}

impl CModInfo {
    pub fn title(&self) -> Option<&str> {
        if let Ok(title) = unsafe { CStr::from_ptr(self.title as *const i8) }.to_str() {
            return Some(title)
        }
        None
    }

    pub fn description(&self) -> Option<&str> {
        if let Ok(desc) = unsafe { CStr::from_ptr(self.description as *const i8) }.to_str() {
            return Some(desc)
        }
        None
    }

    pub fn version(&self) -> Option<&str> {
        if let Ok(version) = unsafe { CStr::from_ptr(self.version as *const i8) }.to_str() {
            return Some(version)
        }
        None
    }
    
    pub fn author(&self) -> Option<&str> {
        if let Ok(author) = unsafe { CStr::from_ptr(self.author as *const i8) }.to_str() {
            return Some(author)
        }
        None
    }
}