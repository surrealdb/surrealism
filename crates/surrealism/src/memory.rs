#[unsafe(no_mangle)]
pub extern "C" fn __sr_alloc(len: u32, align: u32) -> i32 {
    let layout = match std::alloc::Layout::from_size_align(len as usize, align as usize) {
        Ok(layout) => layout,
        Err(_) => return -1, // invalid layout
    };

    let ptr = unsafe { std::alloc::alloc(layout) };

    if ptr.is_null() {
        -1 // signal OOM or allocation failure
    } else {
        ptr as usize as i32 // cast pointer to offset
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn __sr_free(ptr: u32, len: u32) -> i32 {
    let layout = match std::alloc::Layout::from_size_align(len as usize, 8) {
        Ok(layout) => layout,
        Err(_) => return -1, // invalid layout
    };

    let ptr = ptr as usize as *mut u8;
    unsafe {
        std::alloc::dealloc(ptr, layout);
    }
    0 // success
}
