#[macro_export]
macro_rules! init_mod {
    ($title:literal, $version:literal, $author:literal, $init:block) => {
        #[no_mangle]
        #[allow(non_snake_case)]
        extern "system" fn DllMain(_module: $crate::windows::HMODULE, call_reason: u32, _: *mut ()) {
            match call_reason {
                $crate::windows::DLL_PROCESS_ATTACH => $init,
                _ => (),
            };
        }

        const MOD_INFO: $crate::ModInfo = $crate::ModInfo { title: $title, version: $version, author: $author };
    }
}

#[macro_export]
macro_rules! make_hook {
    ($id:ident, $addr:expr, ($($param:ident: $type:ty),*) $code:block) => {
        $crate::make_hook!($id, $addr, ($($param: $type),*) -> () $code);
    };
    ($id:ident, $addr:expr, ($($param:ident: $ty:ty),*) -> $ret:ty $code:block) => {
        $crate::paste! {
            #[allow(non_upper_case_globals)]
            pub(crate) static $id: $crate::Lazy<$crate::GenericDetour<unsafe extern "system" fn($($ty,)*) -> $ret>> = $crate::Lazy::new(|| {
                unsafe {
                    let func = $crate::make_func!($addr, ($($ty),*) -> $ret);
                    $crate::GenericDetour::new(func, [<$id _Fn>])
                        .expect(&format!("Failed to create hook: {}", stringify!($id)))
                }
            });
            #[allow(non_snake_case)]
            unsafe extern "system" fn [<$id _Fn>]($($param: $ty,)*) -> $ret {
                $code
            }
        }
    };
}

#[macro_export]
macro_rules! native_func {
    ($ptr:expr, $fn:ident ( $($param:ty),* )) => {
        $crate::native_func!( $ptr, $fn ($($param),*) -> () );
    };
    ($ptr:expr, $vis:vis $fn:ident ( $($param:ty),* ) -> $ret:ty) => {
        $vis static $fn: $crate::Lazy<unsafe extern "system" fn( $($param),* ) -> $ret> = $crate::Lazy::new(|| unsafe {
            std::mem::transmute::<*const (), unsafe extern "system" fn($($param),*) -> $ret>($ptr as _)
        });
    }
}