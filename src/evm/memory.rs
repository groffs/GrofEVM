pub struct Emem {
    mem : Vec<u8>
}

impl Emem {
    pub fn new() -> Self {
        Self { mem:vec![0;0]}
    }

    pub fn realloc(&mut self, size:usize) {
        if self.mem.len() < size {
            self.mem.resize(size, 0);
        }
    }

    pub fn read(&self, offset : usize, size: usize) -> &[u8] {
        &self.mem[offset..offset + size]
    }

    pub fn write(&mut self, offset : usize, ebyte: &[u8]) {
        self.realloc(ebyte.len());
        self.mem[offset..offset + ebyte.len()].copy_from_slice(ebyte);
    }

    pub fn dump(&self) -> &[u8] {
       &self.mem
    } 
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_mem_allocations() {
        let mut mem = Emem::new();
        let eoffset = 0x00;
        let esizeet = 0x0a;
        let ebytes : [u8; 4] = [0x01, 0x02, 0x03, 0x04]; 
        mem.write(eoffset, &ebytes);
        mem.read(eoffset, esizeet);
        assert_eq!(mem.dump(), &[0x01, 0x02, 0x03, 0x04]);
    }
}