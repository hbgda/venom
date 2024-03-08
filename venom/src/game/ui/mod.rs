#[repr(C)]
pub struct UISettingsMenuSystem {
    _0x0: [u8; 0x20],
    pub system_menu_cfg: *const UISystemMenuConfig
}

#[repr(C)]
pub struct UISystemMenuConfig {
    _0x0: [u8; 0x5F],
    pub item_list: *const UISystemMenuItem,
    pub item_count: u32,
}

#[repr(C)]
pub struct UISystemMenu {
    vftable: *const (),
    _0x8: u8,
    pub title: *const u8,
    _0x18: u8,
    pub header: *const u8,
    _0x28: [u8; 0x10],
    pub item_list: *const UISystemMenuItem,
    pub item_count: u32
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct UISystemMenuItem {
    vftable: *const (),
    pub id: u8,
    pub title: *const u8,
    _0x10: [u8; 0x12],
    pub description: *const u8,
    _0x38: [u8; 0x12],
    pub image: *const u8,
    _0x58: [u8; 0x10],
    pub option_type: *const (),
    _0x70: [u8; 0x38],
}