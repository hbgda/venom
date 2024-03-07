use std::ffi::CStr;

pub trait Component {
    const NAME: &'static str;
}

#[repr(C)]
pub struct ComponentInfo {
    _0x0: [u8; 0x60],
    name_ptr: *const i8
}

#[repr(C)]
pub struct ComponentEntry {
    pub info_ptr: *const ComponentInfo,
    pub component_ptr: *const ()
}

impl ComponentInfo {
    pub fn get_name(&self) -> Option<&str> {
        if self.name_ptr.is_null() {
            return None
        }
        if let Ok(name) = unsafe { CStr::from_ptr(self.name_ptr) }.to_str() {
            return Some(name)
        }
        None
    }
}

impl ComponentEntry {
    pub fn info(&self) -> Option<&ComponentInfo> {
        if self.info_ptr.is_null() {
            return None
        }
        Some(
            unsafe { &*self.info_ptr }
        )
    }

    pub fn component<T: Component>(&self) -> Option<&T> {
        if self.component_ptr.is_null() {
            return None
        }
        Some(
            unsafe { &*(self.component_ptr as *const T) }
        )
    }

    pub fn component_mut<T: Component>(&mut self) -> Option<&mut T> {
        if self.component_ptr.is_null() {
            return None
        }
        Some(
            unsafe { &mut *(self.component_ptr as *mut T) }
        )
    }
}