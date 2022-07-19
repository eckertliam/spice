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

    pub fn del(&mut self) {
        self.deref = true
    }

    //push a single variable
    pub fn push(&mut self, val: &str, id: &str) {
        let new_relative = match self.mem_map.len() {
            0 => 0,
            n => self.mem_map[n - 1].get_relative() + 1,
        };
        let new_addr = Address::new(self.stack.len(), new_relative);
        self.mem_map.push(new_addr);
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
        for v in collection {
            self.stack.push(v.to_var(new_addr));
            new_relative += 1;
            new_addr = Address::new(self.stack.len(), new_relative)
        }
    }

    pub fn remove_addr(&mut self, addr: Address) {
        self.stack
            .retain(|var| var.get_address().get_relative() != addr.get_relative());
        self.mem_map
            .retain(|member| member.get_actual() != addr.get_actual());
        for a in &mut self.mem_map {
            if a.get_relative() > addr.get_relative() {
                a.shift_back(1);
                self.stack[a.get_actual()].shift_address_backward(1);
            }
        }
    }
}
