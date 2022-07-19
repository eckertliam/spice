pub struct Address {
    actual: usize, //the actual index within the mempool stack
    relative: u8,  //the address within the vm mem
}

impl Address {
    pub fn new(actual: usize, relative: u8) -> Self {
        Self { actual, relative }
    }

    pub fn shift(&mut self, offset: u8) {
        self.relative += offset
    }
}
