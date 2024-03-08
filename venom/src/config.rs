use std::path::Path;

const CONFIG_DIR: &'static str = "./mods/scripts/configs";

#[macro_export]
macro_rules! make_config {
    ($mod_info:expr, $name:ident { $($field:ident : $type:ty = $default:literal),* }) => {
        static mut CONFIG: $crate::Lazy<$name> = $crate::Lazy::new($name::load);
        
        #[derive(serde::Deserialize, serde::Serialize)]
        pub struct $name {
            $(
                $field: $type
            ),*
        }

        impl Default for $name {
            fn default() -> Self {
                Self {
                    $(
                        $field: $default
                    ),*
                }
            }
        }

        impl $crate::config::Config for $name {
            const MOD_INFO: &'static $crate::ModInfo = &$mod_info;
        }

        pub fn get_config<'a>() -> &'a mut $name {
            unsafe { &mut *CONFIG }
        }
    };
}

pub trait Config: for<'a> serde::Deserialize<'a> + serde::Serialize {
    const MOD_INFO: &'static crate::ModInfo;

    fn get_config_path() -> String
    where
        Self: Sized
    {
        format!("{CONFIG_DIR}/{}.json", &Self::MOD_INFO.title.trim_end_matches("\0"))    
    }

    /// Loads the config for ModInfo.title, or creates it if not found.
    fn load() -> Self 
    where 
        Self: Sized + for<'a> serde::Deserialize<'a> + serde::Serialize + Default
    {
        let path = Self::get_config_path();
        
        if !Path::new(&path).exists() {
            let cfg = Self::default();
            let _ = std::fs::write(path, serde_json::to_string_pretty(&cfg).unwrap());
            return cfg
        }

        let bytes = std::fs::read(path).unwrap();
        serde_json::from_slice(bytes.as_slice()).unwrap()
    }

    fn save(&self)
    where
        Self: Sized + serde::Serialize 
    {
        let path = Self::get_config_path();
        let _ = std::fs::write(path, serde_json::to_string_pretty(self).unwrap());
    }
}