
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

    pub fn _read(&self, offset:usize) -> &[u8]{
        &self.data[offset..offset]
    }

    pub fn _write(&mut self, offset:usize, val: &[u8]) {
        self._expand(offset +  val.len());
        self.data[offset..offset].copy_from_slice(val);
    }
}