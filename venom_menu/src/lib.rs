mod hooks;

use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::ffi::CStr;
use std::hash::{Hash, Hasher};
use std::sync::atomic::{AtomicU32, Ordering};

use venom::Lazy;

const CORE_PRIORITY: &[&str] = &[
    "venom_core",
    "venom_menu"
];

fn get_priority(title: &str) -> usize {
    CORE_PRIORITY.iter().position(|&t| t == title).unwrap_or(1000)
}

static mut CORE_MAP: Vec<(venom::CModInfo, extern fn() -> venom::menu::OptionsMenu)> = Vec::new();
static mut MODS_MAP: Vec<(venom::CModInfo, extern fn() -> venom::menu::OptionsMenu)> = Vec::new();

static mut VALUE_CALLBACKS: Lazy<HashMap<u32, venom::menu::Callback>> = Lazy::new(HashMap::new);
static mut BUTTON_CALLBACKS: Lazy<HashMap<u32, extern fn()>> = Lazy::new(HashMap::new); 

static mut OPTION_COUNT: AtomicU32 = AtomicU32::new(0);

#[no_mangle]
unsafe extern "system" fn generate_id(label_ptr: *const i8) -> u32 {
    let label = match CStr::from_ptr(label_ptr).to_str() {
        Ok(label) => label,
        Err(_) => return 0 
    };
    
    internal_generate_id(label)
}

pub fn internal_generate_id(label: &str) -> u32 {
    let id = unsafe { &OPTION_COUNT }.fetch_add(1, Ordering::SeqCst);

    let mut opt_hasher = DefaultHasher::new();
    format!("MOD_OPT_{label}_{id}").hash(&mut opt_hasher);
    let opt_hash = opt_hasher.finish();

    ((opt_hash >> 32) as u32) | (opt_hash & 0xFFFF) as u32
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
unsafe extern "system" fn register_menu(mod_info: venom::CModInfo, create: extern fn() -> venom::menu::OptionsMenu) {
    if let Some(title) = mod_info.title() {
        if title.starts_with("venom_") {
            CORE_MAP.push((mod_info, create));
            return
        }
    }
    MODS_MAP.push((mod_info, create));
}

// #[no_mangle]
// unsafe extern "system" fn register_value_callback(id: u32, cb: extern fn(f64)) {
//     VALUE_CALLBACKS_MAP.insert(id, cb);
// }
#[no_mangle]
unsafe extern "system" fn register_f64_callback(id: u32, cb: extern fn(f64)) {
    VALUE_CALLBACKS.insert(id, venom::menu::Callback::F64(cb));
}

#[no_mangle]
unsafe extern "system" fn register_u32_callback(id: u32, cb: extern fn(u32)) {
    VALUE_CALLBACKS.insert(id, venom::menu::Callback::U32(cb));
}


#[no_mangle] 
unsafe extern "system" fn register_bool_callback(id: u32, cb: extern fn(bool)) {
    VALUE_CALLBACKS.insert(id, venom::menu::Callback::Bool(cb));
}

#[no_mangle]
unsafe extern "system" fn register_button_callback(id: u32, cb: extern fn()) {
    BUTTON_CALLBACKS.insert(id, cb);
}

unsafe fn emit_value_callback(id: u32, val: f64) -> bool {
    if let Some(cb) = VALUE_CALLBACKS.get(&id) {
        cb.call(val);
        return true
    }
    false
}

unsafe fn emit_button_callback(id: u32) -> bool {
    if let Some(cb) = BUTTON_CALLBACKS.get(&id) {
        cb();
        return true
    }
    false
}

unsafe fn add_mod_entry(menu: &mut venom::menu::OptionsMenu, info: &venom::CModInfo, mod_menu: venom::menu::OptionsMenu) {
    let title = info.title().unwrap_or("INVALID TITLE");
    let desc = info.description().unwrap_or("INVALID DESCRIPTION");
    let version = info.version().unwrap_or("INVALID VERSION");
    let author = info.author().unwrap_or("INVALID AUTHOR");
    
    let desc_str = format!("{title} v{version}\nby {author}\n{desc}");
    menu.add_submenu(title.into(), Some(desc_str), Box::new(mod_menu));
}

unsafe fn create_mods_list() -> venom::menu::OptionsMenu {
    let mut mods_list = venom::menu::OptionsMenu::new("Mods");

    for (info, create) in MODS_MAP.iter() {
        add_mod_entry(&mut mods_list, info, create());
    }

    if MODS_MAP.len() != 0 { 
        mods_list.add_blank();
    }

    mods_list.add_header("Core".into());

    // Order core modules
    CORE_MAP.sort_by(|(a, _), (b, _)| {
        get_priority(a.title().unwrap_or("")).cmp(&get_priority(b.title().unwrap_or("")))
    });

    for (info, create) in CORE_MAP.iter() {
        add_mod_entry(&mut mods_list, info, create());
    }

    mods_list
}

#[allow(improper_ctypes_definitions)]
extern fn create_menu() -> venom::menu::OptionsMenu {
    let mut menu = venom::menu::OptionsMenu::new("venom_menu");
    // What do i put in here?
    menu.add_blank();
    menu
}

venom::init_mod!(
    "venom_menu",
    "Allows venom scripts to add custom option menus.",
    "0.0.1",
    "L_",
    {
        unsafe { 
            hooks::enable();
            register_menu((&MOD_INFO).into(), create_menu);
        };
    }
);