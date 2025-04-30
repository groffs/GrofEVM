
pub struct _Ememory {
    data:Vec<u8>
}

impl _Ememory {
    pub fn _new() -> Self {
        Self {data: vec![0; 0]}
    }

    pub fn _expand(&mut self, new_size: usize) {
        if new_size > self.data.len() {
            self.data.resize(new_size, 0);
        }
    }

    pub fn _read(&self, offset:usize, size: usize) -> &[u8]{
        &self.data[offset..offset + size]
    }

    pub fn _write(&mut self, offset:usize, val: &[u8]) {
        self._expand(offset +  val.len());
        self.data[offset..offset + val.len()].copy_from_slice(val);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_read_write_mem() {
        let mut mem = _Ememory::_new();
        let offset = 0;
        let size = 23;
        
        let bcodes = &[0x01 as u8, 0x60];
        mem._write(offset, bcodes);
        let mreadf = mem._read(offset, size);
        assert_eq!(mreadf[0], 0x01);
    }
}