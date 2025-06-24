use anyhow::Result;
use surrealism_types::controller::MemoryController;

use crate::memory::{alloc, free};

pub struct Controller {

}

impl MemoryController for Controller {
    fn alloc(&mut self, len: u32) -> Result<u32> {
        Ok(alloc(len))
    }

    fn free(&mut self, ptr: u32, len: u32) -> Result<()> {
        Ok(free(ptr, len))
    }

    fn mut_mem<'a>(&'a mut self, ptr: u32, len: u32) -> &'a mut [u8] {
        unsafe {
            let ptr = ptr as usize as *mut u8;
            std::slice::from_raw_parts_mut(ptr, len as usize)
        }
    }
}