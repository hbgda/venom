use std::ffi::CString;

crate::load_library_func!("venom_menu", "generate_id", GENERATE_ID(*const i8) -> u32);
pub fn generate_id(label: &str) -> u32 {
    if let Some(extern_generate_id) = *GENERATE_ID {
        let label_ptr = CString::new(label).unwrap().into_raw();
        
        unsafe { 
            let id = extern_generate_id(label_ptr);
            let _ = CString::from_raw(label_ptr);
            return id;
        }
    }
    0
}

crate::load_library_func!("venom_menu", "register_menu", REGISTER_MENU(crate::CModInfo, fn() -> super::OptionsMenu));
pub fn register_menu(mod_info: crate::CModInfo, create: fn() -> super::OptionsMenu) {
    if let Some(extern_register_menu) = *REGISTER_MENU {
        extern_register_menu(mod_info, create)
    }
}

crate::load_library_func!("venom_menu", "register_value_callback", REGISTER_VALUE_CALLBACK(u32, fn(f64)));
pub fn register_value_callback(id: u32, cb: fn(f64)) {
    if let Some(extern_register_value_callback) = *REGISTER_VALUE_CALLBACK {
        extern_register_value_callback(id, cb)
    }
}

crate::load_library_func!("venom_menu", "register_button_callback", REGISTER_BUTTON_CALLBACK(u32, fn()));
pub fn register_button_callback(id: u32, cb: fn()) {
    if let Some(extern_register_button_callback) = *REGISTER_BUTTON_CALLBACK {
        extern_register_button_callback(id, cb)
    }
}