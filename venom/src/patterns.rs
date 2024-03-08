pub(crate) mod actor {
    pub const GET_ACTOR: &'static str = "44 8B 01 41 8B D0 C1 EA 14 81 E2 FF 07 00 00 74 ?? 41 81 E0 FF FF 0F 00 4B 8D 0C ?? 48 C1 E1 06 48 03 0D ?? ?? ?? ?? 44 3B 05 ?? ?? ?? ?? 73 ?? 0F B7 41 ?? 3B D0 74";
    pub const SPAWN_ACTOR: &'static str = "40 53 48 83 EC 50 48 8B DA 48 8B D1 48 8D 0D";
    pub const ENABLE_ACTOR: &'static str = "48 89 5C 24 ?? 48 89 6C 24 ?? 56 57 41 56 48 81 EC 80 00 00 00 48 8B F9";
}

pub(crate) mod transform {
    pub const SET_POSITION: &'static str = "0F 10 51 ?? 4C 8B C1";
    pub const SET_SCALE: &'static str =  "40 53 48 81 EC 80 00 00 00 F2 0F 10 02";
}

pub(crate) mod assets {
    pub const GET_ASSET_MANAGER: &'static str = "48 0F BE C1 48 8D 0D ?? ?? ?? ?? 48 8B 04 ?? C3";
    pub const GET_MANAGER_BY_ASSET_TYPE: &'static str = "33 C0 81 F9 86 FE 08 72";
    pub const LOAD_ASSET: &'static str = "48 89 54 24 ?? 53 56 57 41 55 41 56 48 83 EC 50 48 8B DA";
}

pub(crate) mod hud {
    pub const CREATE_MESSAGE: &'static str = "48 8B C4 48 89 58 ?? 48 89 70 ?? 48 89 78 ?? 55 41 54 41 55 41 56 41 57 48 8D A8 ?? ?? ?? ?? 48 81 EC 30 05 00 00";
    pub const CLEAR_MESSAGE: &'static str = "48 89 5C 24 ?? 48 89 6C 24 ?? 48 89 74 24 ?? 48 89 7C 24 ?? 41 57 48 81 EC 90 00 00 00";
    pub const GET_HUD: &'static str = "83 F9 01 77 ?? 48 63 C1 48 8D 0D ?? ?? ?? ?? 48 8B 04 ?? C3 33 C0";
    // pub const HUD_HIDEHUD:                   &'static str = "48 89 5C 24 ?? 48 89 74 24 ?? 57 48 83 EC 30 0F 57 C0 41 0F B6 F0 0F 2F D8";
    // pub const HUD_ADDNEWMESSAGE:             &'static str = "48 89 5C 24 ?? 48 89 74 24 ?? 48 89 7C 24 ?? 4C 89 64 24 ?? 55 41 56 41 57 48 8D 6C 24 ?? 48 81 EC C0 00 00 00 49 8B F9";
    // pub const HUD_CREATEPLAYERHUD:           &'static str = "48 89 5C 24 ?? 48 89 74 24 ?? 48 89 4C 24 ?? 57 48 83 EC 30 48 63 DA";
}

pub(crate) mod scaleform {
    // pub const OPEN_FILE: &'static str = "48 89 5C 24 ?? 57 48 81 EC 40 02 00 00 48 8B FA";
    pub const OPEN_FILE_DISC: &'static str = "48 89 5C 24 ?? 48 89 6C 24 ?? 48 89 74 24 ?? 48 89 7C 24 ?? 41 56 48 83 EC 30 33 DB 41 8B E8";

    pub mod value {
        pub const CREATE_OBJECT: &'static str = "E8 ** ** ** ** 66 0F 6F 05 ?? ?? ?? ?? F3 0F 7F 45 ?? 33 F6 48 89 75 00";
        pub const CREATE_ARRAY: &'static str = "E8 ** ** ** ** 48 8D 0D ?? ?? ?? ?? E8 ?? ?? ?? ?? 48 85 C0 0F 84 ?? ?? ?? ?? 4C 8D 78";
    }
}

pub(crate) mod hero {
    pub const HERO_SYSTEM_OFFSET: &'static str = "48 89 05 ** ** ** ** 48 89 35";
}