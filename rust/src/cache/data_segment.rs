use crate::cache::freelist::FreeList;

use super::data_block::Data;


#[derive(Debug)]
pub struct DataSegment {
    data: Data,
    free_list: FreeList,
}

impl DataSegment {
    pub fn new(ptr: *mut u8, off: u64, len: u64) -> DataSegment {
        let mut free_list = FreeList::new();
        free_list.insert(off, len);
        DataSegment {
            data: Data::new(ptr, off, len),
            free_list,
        }
    }

    pub fn allocate(&mut self) -> Option<Data> {
        let ret = self.free_list.get();
        if let Some((off, len)) = ret {
            let data = self.data.allocate(off, len);
            log::info!(
                "Allocate data {:?}: [{:?}, {})",
                data.as_ptr(),
                data.off(),
                data.off() + data.len()
            );
            return Some(data);
        }
        None
    }

    pub fn free(&mut self, off: u64, len: u64) {
        log::info!("Free data [{:}, {:})", off, off + len);
        self.free_list.insert(off, len)
    }
}

#[cfg(test)]
mod tests {
    use crate::cache::data_block::Data;

    use super::DataSegment;

    #[test]
    fn test() {
        const LEN: usize = 1024;
        let mut bytes = [0u8; LEN];
        let ptr = bytes.as_mut_ptr();
        let mut ds = DataSegment::new(ptr, 0, LEN as u64);
        assert!(ds.allocate() == Some(Data::new(ptr, 0, LEN as u64)));
        assert!(ds.allocate() == None);

        ds.free(1, 3);
        unsafe { assert!(ds.allocate() == Some(Data::new(ptr.offset(1), 1, 3))) }
    }
}
