use anyhow::Result;

pub trait MemoryController {
    fn alloc(&mut self, len: u32, align: u32) -> Result<u32>;
    fn free(&mut self, ptr: u32, len: u32) -> Result<()>;
    fn mut_mem<'a>(&'a mut self, ptr: u32, len: u32) -> &'a mut [u8];
}
