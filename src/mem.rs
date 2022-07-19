#[derive(PartialEq, Clone, Copy)]
pub struct Address {
    actual: usize, //the actual index within the mempool stack
    relative: u8,  //the address within the vm mem
}

impl Address {
    pub fn new(actual: usize, relative: u8) -> Self {
        Self { actual, relative }
    }

    pub fn get_actual(self) -> usize {
        self.actual
    }

    pub fn get_relative(self) -> u8 {
        self.relative
    }

    pub fn shift_forward(&mut self, offset: u8) {
        self.relative += offset;
        self.actual += offset as usize;
    }

    pub fn shift_back(&mut self, offset: u8) {
        self.relative += offset;
        self.actual += offset as usize;
    }
}
