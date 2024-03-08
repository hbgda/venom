use venom::{hash, Lazy, scaleform::value::{Value, ValueType}, game::ui::{UISettingsMenuSystem, UISystemMenuConfig, UISystemMenuItem}};

const TEXT_MODS_LABEL: &'static str = "MODS\0";
const TEXT_MODS_DESC: &'static str= "View and configure your mods.\0";

const UPDATE_OPTION: Lazy<u32> = Lazy::new(|| hash::gui_hash("UpdateOption"));
const UPDATE_COLOUR_OPTION: Lazy<u32> = Lazy::new(|| hash::gui_hash("UpdateColorOption"));
const OPTIONS_ACCEPT: Lazy<u32> = Lazy::new(|| hash::gui_hash("OptionsAccept"));

venom::make_hook!(
    HOOK_UISettingsMenuSystem_Init,
    venom::utils::scan("40 57 48 83 EC 30 48 83 79 ?? 00 48 8D 79").unwrap(),
    (this: *const UISettingsMenuSystem) {
        HOOK_UISettingsMenuSystem_Init.call(this);
        let sms = &*this;
        let sms_cfg = &mut *(sms.system_menu_cfg as *mut UISystemMenuConfig);
        let items = std::slice::from_raw_parts_mut(sms_cfg.item_list as *mut UISystemMenuItem, sms_cfg.item_count as usize);

        let mut prev_on_idx = 0;
        while items.get_unchecked(prev_on_idx).id != 95 {
            prev_on_idx += 1;
        }

        let mods_item = items.get_unchecked_mut(prev_on_idx);
        mods_item.title = TEXT_MODS_LABEL.as_ptr();
        mods_item.description = TEXT_MODS_DESC.as_ptr();
        mods_item.id = 80;

        let blank = items.get_unchecked_mut(prev_on_idx + 1);
        // Yeet connect to psn button cos its ugly
        if blank.id == 128 {
            blank.title = std::ptr::null();
            blank.description = std::ptr::null();
            blank.id = 126;
            blank.image = std::ptr::null();
            blank.option_type = std::ptr::null();

            let quit_actual = items.get_unchecked(prev_on_idx + 3);
            let quit_new = &mut *(items.get_unchecked(prev_on_idx + 2) as *const UISystemMenuItem as *mut UISystemMenuItem);
            quit_new.title = quit_actual.title;
            quit_new.description = quit_actual.description;
            quit_new.id = quit_actual.id;

            sms_cfg.item_count -= 1;
        }
    }
);

venom::make_hook!(
    HOOK_PopupSeamlessLobbyUI_CreateOptionData,
    venom::utils::scan("48 89 5C 24 ?? 48 89 74 24 ?? 48 89 7C 24 ?? 55 48 8D 6C 24 ?? 48 81 EC 20 01 00 00 48 8B FA").unwrap(),
    (smth: *const (), lobby_data: *mut Value) {
        HOOK_PopupSeamlessLobbyUI_CreateOptionData.call(smth, lobby_data);
        let data = &mut *lobby_data;
        if let None = data.get_member("modsData") {
            let mods_menu_val = &mut super::create_mods_list().to_value();
            if data.set_member("modsData", mods_menu_val) {
                mods_menu_val.drop();
            }
        }
    }
);

venom::make_hook!(
    HOOK_ExternalInterface_LobbyManager,
    venom::utils::scan("48 89 5C 24 ?? 48 89 74 24 ?? 48 89 7C 24 ?? 4C 89 64 24 ?? 55 41 56 41 57 48 8D 6C 24 ?? 48 81 EC 10 01 00 00 45 8B F9").unwrap(),
    (_movie: *const (), method_hash: u32, args: *mut [Value; 0], nargs: u32) {
        let mut opt_id = None;
        if !args.is_null() {
            let first = (&*args).get_unchecked(0);
            if first.get_type() == ValueType::UInt {
                opt_id = Some(first.get_uint());
            }
        }

        // Ignore native options
        if opt_id == None || opt_id < Some(1000) {
            return HOOK_ExternalInterface_LobbyManager.call(_movie, method_hash, args, nargs);
        }

        let args_arr = &mut *args;
        if method_hash == *UPDATE_OPTION || method_hash == *UPDATE_COLOUR_OPTION {
            let id_value = args_arr.get_unchecked(0);
            let changed_value = args_arr.get_unchecked(1);
            super::emit_value_callback(id_value.get_uint(), changed_value.get_number());
            args_arr.get_unchecked_mut(0).set_uint(0);
            return HOOK_ExternalInterface_LobbyManager.call(_movie, *UPDATE_OPTION, args, nargs);
            
        }
        if method_hash == *OPTIONS_ACCEPT {
            let id_value = args_arr.get_unchecked(0);
            super::emit_button_callback(id_value.get_uint());
        }

        HOOK_ExternalInterface_LobbyManager.call(_movie, method_hash, args, nargs);
    }
);

pub unsafe fn enable() {
    HOOK_UISettingsMenuSystem_Init.enable().unwrap();
    HOOK_PopupSeamlessLobbyUI_CreateOptionData.enable().unwrap();
    HOOK_ExternalInterface_LobbyManager.enable().unwrap();
}