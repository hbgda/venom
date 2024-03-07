use std::ffi::CStr;

use super::{component::{Component, ComponentEntry}, transform::Transform};

crate::native_func!(
    crate::utils::scan(crate::patterns::actor::GET_ACTOR).unwrap(),
    GET_ACTOR(*const u32) -> *const Actor
);
crate::native_func!(
    crate::utils::scan(crate::patterns::actor::SPAWN_ACTOR).unwrap(),
    SPAWN_ACTOR(u64, *const ()) -> *const Actor
);
crate::native_func!(
    crate::utils::scan(crate::patterns::actor::ENABLE_ACTOR).unwrap(),
    ENABLE_ACTOR(*const Actor)
);

pub fn get_actor<'l>(handle: &u32) -> Option<&'l Actor> {
    Some(
        unsafe { 
            &*(crate::utils::option_ptr(
                GET_ACTOR(handle)
            )?) 
        }
    )
}

pub fn get_actor_mut<'l>(handle: &u32) -> Option<&'l mut Actor> {
    Some(get_actor(handle)?.as_mut())
}

#[repr(C)]
pub struct Actor {
    transform: *const Transform,
    _0x8: u32,
    handle_lower: u32,
    handle_upper: u16,
    _0x12: [u8; 0x56],
    component_list: *const ComponentEntry,
    component_count: u16,
    _0x72: [u8; 0x38],
    name: *const i8
}

impl Actor {
    pub fn as_mut(&self) -> &mut Actor {
        unsafe { &mut *(self as *const Actor as *mut Actor) }
    }

    pub fn spawn<'l>(actor_hash: u64, spatial_data: *const ()) -> Option<&'l Actor> {
        Some(
            unsafe {
                &*(crate::utils::option_ptr(
                    SPAWN_ACTOR(actor_hash, spatial_data)
                )?)
            }
        )
    }

    pub fn enable(&self) {
        unsafe { ENABLE_ACTOR(self) }
    }

    pub fn get_name(&self) -> Option<&str> {
        if let Ok(name) = unsafe { CStr::from_ptr(self.name) }.to_str() {
            return Some(name)
        }
        None
    }

    pub fn handle(&self) -> u32 {
        ((self.handle_upper as u32) << 0x14) | self.handle_lower
    }

    pub fn get_components(&self) -> &[ComponentEntry] {
        unsafe {
            std::slice::from_raw_parts(self.component_list, self.component_count as usize)
        }
    }

    pub fn get_components_mut(&mut self) -> &mut [ComponentEntry] {
        unsafe {
            std::slice::from_raw_parts_mut(self.component_list as *mut ComponentEntry, self.component_count as usize)
        }
    }

    pub fn find_component_entry(&self, name: &str) -> Option<*const ComponentEntry> {
        let component_list = self.get_components();
        for entry in component_list {
            if name == entry.info()?.get_name()? {
                return Some(entry)
            }
        }
        None 
    }

    pub fn find_component<T: Component>(&self) -> Option<&T> {
        unsafe {
            (&*(
                self.find_component_entry(T::NAME)?
            )).component()
        }
    }

    pub fn find_component_mut<T: Component>(&mut self) -> Option<&mut T> {
        unsafe {
            (&mut *(
                self.find_component_entry(T::NAME)? as *mut ComponentEntry
            )).component_mut()
        }
    }

    pub fn transform(&self) -> &Transform {
        unsafe { &*self.transform }
    }

    pub fn transform_mut(&mut self) -> &mut Transform {
        unsafe { &mut *(self.transform as *mut Transform) }
    }

    pub fn set_transform(&mut self, transform: &Transform) {
        self.transform = transform;
    }
}