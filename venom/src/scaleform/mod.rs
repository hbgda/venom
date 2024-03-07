crate::native_func!(
    crate::utils::scan(crate::patterns::scaleform::OPEN_FILE_DISC).unwrap(),
    OPEN_FILE_DISC(*const (), *const u8, u8, u32) -> *const ()
);

#[repr(u8)]
pub enum OpenFlags {
    Read = 1,
    Write,
    ReadWrite,
    Truncate,
    Create = 8,
    CreateOnly = 24,
    Buffered = 32
}

pub fn load_file(path: &str) -> Option<*const ()> {
    let file = unsafe { OPEN_FILE_DISC(std::ptr::null(), format!("{path}\0").as_ptr(), OpenFlags::Read as u8 | OpenFlags::Buffered as u8, 444) };
    crate::utils::option_ptr(file)
}