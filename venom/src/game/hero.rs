use once_cell::sync::Lazy;

use super::actor::{self, Actor};

const HERO_SYSTEM_PTR: Lazy<Option<*const HeroSystem>> = Lazy::new(|| unsafe {
    crate::utils::scan_ref(crate::patterns::hero::HERO_SYSTEM_OFFSET)
});

pub fn hero_system() -> Option<&'static HeroSystem> {
    Some(
        unsafe { &*crate::utils::option_ptr((*HERO_SYSTEM_PTR)?)? }
    )
}

pub fn hero_system_mut() -> Option<&'static mut HeroSystem> {
    Some(
        unsafe { 
            &mut *(
                crate::utils::option_ptr((*HERO_SYSTEM_PTR)?)? as *mut HeroSystem
            ) 
        }
    )
}

#[repr(C)]
pub struct HeroSystem {
    _0x0: [u8; 0x1C],
    hero_handle: u32
}

impl HeroSystem {
    pub fn hero_handle(&self) -> u32 {
        self.hero_handle
    }

    pub fn get_hero(&self) -> Option<&Actor> {
        actor::get_actor(&self.hero_handle)
    }

    pub fn get_hero_mut(&mut self) -> Option<&mut Actor> {
        actor::get_actor_mut(&self.hero_handle)
    }
}