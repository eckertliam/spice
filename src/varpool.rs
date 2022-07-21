use crate::mem::Address;
use crate::var::StripVar;
use crate::var::Var;

//a pool of variables generated per scope
pub struct VarPool {
    stack: Vec<Var>,
    scope: u8,             //address in the virtual machine
    mem_map: Vec<Address>, //where each var or collection of vars starts
    deref: bool,           //indicates whether the varpool can be removed on the next sweep
}

impl VarPool {
    pub fn new(scope: u8) -> Self {
        Self {
            stack: vec![],
            scope,
            mem_map: vec![],
            deref: false,
        }
    }
    //sets the pool to be derefed on the next sweep
    pub fn del(&mut self) {
        self.deref = true
    }

    //push a single variable
    pub fn push(&mut self, val: &str, id: &str) {
        //adds 1 to the last relative address in mem_map or if there are none sets it to 0
        let new_relative = match self.mem_map.len() {
            0 => 0,
            n => self.mem_map[n - 1].get_relative() + 1,
        };
        //the actual address is the len of the stack
        let new_addr = Address::new(self.stack.len(), new_relative);
        //push addr to mem_map
        self.mem_map.push(new_addr);
        //push the corresponding var with the same address to the stack
        self.stack.push(Var::new(val, id, new_addr));
    }
    //push a collection of variables
    pub fn append(&mut self, collection: Vec<StripVar>) {
        let mut new_relative = match self.mem_map.len() {
            0 => 0,
            n => self.mem_map[n - 1].get_relative() + 1,
        };
        let mut new_addr = Address::new(self.stack.len(), new_relative);
        //only adds the first address to the mem map since it is a collection
        self.mem_map.push(new_addr);
        // does push but without the mem_map push for each var since mem_map only stores the first address of a collection
        // the virtual machine recognizes gaps in the map as the length of a collection
        for v in collection {
            self.stack.push(v.to_var(new_addr));
            new_relative += 1;
            new_addr = Address::new(self.stack.len(), new_relative);
        }
    }

    pub fn shift_back_after(&mut self, offset: u8, shift: u8) {
        // checks if each var's relative address occurs after the offset
        // if it does occur after the offset the var is shifted back the amount of the shift
        for v in &mut self.stack {
            if v.get_address().get_relative() > offset {
                v.shift_address_backward(shift);
            }
        }
    }

    pub fn shift_forward_after(&mut self, offset: u8, shift: u8) {
        // checks if the var's relative address occurs after the offset
        // if it does occur after the offset the var is shifted back the amount of the shift
        for v in &mut self.stack {
            if v.get_address().get_relative() > offset {
                v.shift_address_forward(shift);
            }
        }
    }
}
