crate::native_func!(
    crate::utils::scan(crate::patterns::scaleform::OPEN_FILE_DISC).unwrap(),
    OPEN_FILE_DISC(*const (), *const u8, u32, u32) -> *const ()
);

#[repr(u32)]
pub enum OpenFlags {
    Read = 1,
    Write,
    ReadWrite,
    Truncate,
    Create = 8,
    CreateOnly = 24,
    Buffered = 32
}

#[repr(u32)]
pub enum Modes {
    Read = 444,
    Write = 222,
    Execute = 111,
    ReadWrite = 666
}

pub fn load_file(path: &str) -> Option<*const ()> {
    let file = unsafe { OPEN_FILE_DISC(std::ptr::null(), format!("{path}\0").as_ptr(), OpenFlags::Read as u32 | OpenFlags::Buffered as u32, Modes::Read as u32) };
    crate::utils::option_ptr(file)
}