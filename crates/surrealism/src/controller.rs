use anyhow::Result;
use surrealism_types::controller::MemoryController;

use crate::memory::{__sr_alloc, __sr_free};

pub struct Controller {}

impl MemoryController for Controller {
    fn alloc(&mut self, len: u32, align: u32) -> Result<u32> {
        let result = __sr_alloc(len, align);
        if result == -1 {
            anyhow::bail!("Memory allocation failed");
        }
        Ok(result as u32)
    }

    fn free(&mut self, ptr: u32, len: u32) -> Result<()> {
        let result = __sr_free(ptr, len);
        if result == -1 {
            anyhow::bail!("Memory deallocation failed");
        }
        Ok(())
    }

    fn mut_mem(&mut self, ptr: u32, len: u32) -> &mut [u8] {
        unsafe {
            let ptr = ptr as usize as *mut u8;
            std::slice::from_raw_parts_mut(ptr, len as usize)
        }
    }
}
