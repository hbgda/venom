use canny::mem::windows::{ProcessInfo, ProcessScanner};

pub unsafe fn get_module_base() -> isize {
    crate::windows::GetModuleHandleA(crate::windows::s!("MilesMorales.exe")).unwrap().0
}

pub unsafe fn get_offset(offset: isize) -> isize {
    get_module_base() + offset
}

pub unsafe fn get_offset_ptr<T>(offset: isize) -> *const T {
    (get_module_base() + offset) as *const T
}

pub unsafe fn get_offset_ptr_mut<T>(offset: isize) -> *mut T {
    (get_module_base() + offset) as *mut T
}

pub fn option_ptr<T>(ptr: *const T) -> Option<*const T> {
    if ptr.is_null() {
        return None
    }
    Some(ptr)
}

pub unsafe fn scan(pattern_str: &'static str) -> Result<*const (), Box<dyn std::error::Error>> {
    let mut scanner = create_scanner(pattern_str)?;
    match scanner.next() {
        Some(addr) => Ok(addr as *const ()),
        None => Err(format!("Failed to find address for pattern: {pattern_str}").into())
    }
}

pub unsafe fn create_scanner(pattern_str: &'static str) -> Result<ProcessScanner, Box<dyn std::error::Error>> {
    let pattern = canny::pattern::Pattern::new(pattern_str)?;
    let info = ProcessInfo::internal(crate::windows::s!("MilesMorales.exe"))?;
    Ok(ProcessScanner::scan(info, pattern))
}

pub unsafe fn scan_call(pattern: &'static str) -> Option<*const ()> {
    let mut scanner = create_scanner(pattern).unwrap();
    let found = scanner.next()?;
    let offset = i32::from_le_bytes(scanner.store[0..4].try_into().unwrap());
    Some((found as isize + 5 + offset as isize) as *const ())
}

pub unsafe fn scan_ref<T>(pattern: &'static str) -> Option<*const T> {
    let mut scanner = create_scanner(pattern).unwrap();
    let found = scanner.next()?;
    let offset = i32::from_le_bytes(scanner.store[0..4].try_into().unwrap());
    Some((found as isize + 7 + offset as isize) as *const T)
}

pub fn terminate_string(string: &mut String) {
    if !string.ends_with('\0') {
        string.push('\0');
    }
}