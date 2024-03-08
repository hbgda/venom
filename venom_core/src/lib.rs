use std::{ffi::CStr, path::Path};
use venom;
use venom::config::Config;

venom::make_hook!(
    HOOK_ScaleformLoader_OpenFile,
    venom::utils::scan("48 89 5C 24 ?? 57 48 81 EC 40 02 00 00 48 8B FA").unwrap(),
    (this: *const (), path: *const i8) -> *const () {
        if let Ok(path) = CStr::from_ptr(path).to_str() {
            if let Some(idx) = path.find("export") {
                let path = &path[idx + 6..];
                let file_path = format!("./mods/assets/scaleform/{}", path.trim_start_matches("/"));
                if Path::new(&file_path).exists() {
                    if let Some(file) = venom::scaleform::file::load_file(&file_path) {
                        return file
                    }
                }
            }
        }
        HOOK_ScaleformLoader_OpenFile.call(this, path)
    }
);

venom::make_config!(
    MOD_INFO,
    VenomCoreConfig {
        logs_enabled: bool = false
    }
);

extern fn on_change(new: bool) {
    // TODO: Rewrite logging so this can actually do something
    let cfg = get_config();
    cfg.logs_enabled = new;
    cfg.save();
}

fn create_menu() -> venom::menu::OptionsMenu {
    let mut menu = venom::menu::OptionsMenu::new("venom_core");
    
    menu.add_toggle(
        "Show Logs".into(), 
        Some("Show logs from venom scripts.".into()), 
        get_config().logs_enabled, false, 
        on_change
    );

    menu
}

venom::init_mod!(
    "venom_core",
    "Core functionality for venom.",
    "0.0.1",
    "L_",
    {
        venom::menu::register_menu(&MOD_INFO, create_menu);
        unsafe { HOOK_ScaleformLoader_OpenFile.enable() }.unwrap();
    }
);