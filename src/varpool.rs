use crate::mem::Address;
use crate::var::StripVar;
use crate::var::Var;

//a pool of variables generated per scope
pub struct VarPool {
    stack: Vec<Var>,
    scope: u8,        //address in the virtual machine
    mem_map: Vec<u8>, //where each var or collection of vars starts
    deref: bool,      //indicates whether the varpool can be removed on the next sweep
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

    pub fn del(&mut self) {
        self.deref = true
    }

    //push a single variable
    pub fn push(&mut self, val: &str, id: &str) {
        let new_addr = match self.mem_map.len() {
            0 => 0,
            n => self.mem_map[n - 1] + 1,
        };
        self.mem_map.push(new_addr);
        self.stack
            .push(Var::new(val, id, Address::new(self.stack.len(), new_addr)));
    }
    //push a collection of variables
    pub fn append(&mut self, collection: Vec<StripVar>) {
        let mut new_addr = match self.mem_map.len() {
            0 => 0,
            n => self.mem_map[n - 1] + 1,
        };
        //only adds the first address to the mem map since it is a collection
        self.mem_map.push(new_addr);
        for v in collection {
            self.stack
                .push(v.to_var(Address::new(self.stack.len(), new_addr)));
            new_addr += 1;
        }
    }
}
