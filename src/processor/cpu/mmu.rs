use std::ops::{Index, IndexMut};
use std::collections::HashMap;

use crate::debug::debug;

const PAGE_MASK: usize = 0xff;

/// implement pages so we dont need to allocate all the memory from the getgo
#[derive(Clone, Copy, Debug)]
struct Page {
    data: [u8; PAGE_MASK+1]
}

////// PAGE TRAIT IMPLEMENTATIONS //////
impl Index<usize> for Page {
    type Output = u8;
    fn index(&self, s: usize) -> &u8 {
        &self.data[s]
    }
}

impl IndexMut<usize> for Page {
    fn index_mut(&mut self, s: usize) -> &mut u8 {
        &mut self.data[s]
    }
}

impl Page {
    fn new() -> Self {
        Page { data: [0u8; PAGE_MASK+1] }
    }
}





pub struct MMU {
    pages: HashMap<usize, Page>
}

////// TRAIT IMPLEMENTATIONS //////
impl Index<usize> for MMU {
    type Output = u8;
    fn index(&self, s: usize) -> &u8 {
        // translate the address into a useful address
        let page_num = (s & (0xFFFFFFFF - PAGE_MASK)) >> PAGE_MASK.count_ones();
        let page_offset = s & PAGE_MASK;
        debug(format!("MMU: Reading page number {}, offset {} (s={})", page_num, page_offset, s));

        &self.pages[&page_num][page_offset]
    }
}

impl IndexMut<usize> for MMU {
    fn index_mut(&mut self, s: usize) -> &mut u8 {
        // translate the address into a useful address
        let page_num = (s & (0xFFFFFFFF - PAGE_MASK)) >> PAGE_MASK.count_ones();
        let page_offset = s & PAGE_MASK;
        
        debug(format!("MMU: Writing page number {}, offset {} (s={})", page_num, page_offset, s));

        // check if the page exists
        match self.check_page(page_num) {
            true => (),
            false => {
                // it doesn't, so add it
                self.pages.insert(page_num, Page::new());
            }
        }

        &mut self.pages.get_mut(&page_num).unwrap()[page_offset]
    }
}



impl MMU {
    pub fn new() -> Self {
        MMU {
            pages: HashMap::new()
        }
    }

    /// checks if a page number exists or not
    fn check_page(&self, page_num: usize) -> bool{
        match self.pages.get(&page_num) {
            Some(_) => true,
            None => false
        }
    }

    /// retrieves a u32 from memory at address `offset`
    pub fn get_u32(&self, offset: usize) -> Result<u32, String>{
        // bounds check address
        if offset > u32::MAX as usize {
            return Err("Illicit memory access".to_string());
        }

        let mut ret = 0u32;
        ret += (self[offset] as u32) << 24;
        ret += (self[offset + 1] as u32) << 16;
        ret += (self[offset + 2] as u32) << 8;
        ret += self[offset + 3] as u32;
        Ok(ret)
    }

    /// retrieves a u24 from memory at address `offset` (returned as u32)
    pub fn get_u24(&self, offset: usize) -> Result<u32, String> {
        // bounds check address
        if offset > u32::MAX as usize {
            return Err("Illicit memory access".to_string());
        }
        
        let mut ret: u32 = 0;
        ret += (self[offset] as u32) << 16;
        ret += (self[offset + 1] as u32) << 8;
        ret += self[offset + 2] as u32;
        Ok(ret)
    }

    /// retrieves a u16 from memory at address `offset`
    pub fn get_u16(&self, offset: usize) -> Result<u16, String> {
        // bounds check address
        if offset > u32::MAX as usize {
            return Err("Illicit memory access".to_string());
        }
        
        let mut ret: u16 = 0;
        ret += (self[offset] as u16) << 8;
        ret += self[offset + 1] as u16;
        Ok(ret)
        
    }

    pub fn write_u32(&mut self, offset: usize, data: u32) -> Result<(), String> {
        // bounds check address
        if offset > u32::MAX as usize {
            return Err("Illicit memory access".to_string());
        }

        
        self[offset] = ((data >> 24) & 0xFF).try_into().unwrap();
        self[offset + 1] = ((data >> 16) & 0xFF).try_into().unwrap();
        self[offset + 2] = ((data >> 8) & 0xFF).try_into().unwrap();
        self[offset + 3] = (data & 0xFF).try_into().unwrap();
        Ok(())
    }
}