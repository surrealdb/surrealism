#[unsafe(no_mangle)]
pub extern "C" fn __sr_alloc(len: u32) -> u32 {
    let layout = std::alloc::Layout::from_size_align(len as usize, 8).expect("invalid layout");
    let ptr = unsafe { std::alloc::alloc(layout) };

    if ptr.is_null() {
        0 // signal OOM
    } else {
        ptr as usize as u32 // cast pointer to offset
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn __sr_free(ptr: u32, len: u32) {
    let layout = std::alloc::Layout::from_size_align(len as usize, 8).expect("invalid layout");
    let ptr = ptr as usize as *mut u8;
    unsafe {
        std::alloc::dealloc(ptr, layout);
    }
}