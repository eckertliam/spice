use crate::mem::Address;
use crate::tsys::VarType;

//a variable with an address mapped to the memory of a varpool
pub struct Var {
    val: String,
    t: VarType,
    id: String,
    address: Address,
    deref: bool, //indicates whether the variable can be removed on the next sweep
}

impl Var {
    pub fn new(val: &str, id: &str, address: Address) -> Self {
        Self {
            val: val.to_string(),
            t: VarType::new(val).unwrap_or(VarType::Error),
            id: id.to_string(),
            address,
            deref: false,
        }
    }

    pub fn del(&mut self) {
        self.deref = false
    }

    pub fn get_id(&self) -> &String {
        &self.id
    }

    pub fn get_address(&self) -> &Address {
        &self.address
    }

    pub fn shift_address_forward(&mut self, offset: u8) {
        self.address.shift_forward(offset)
    }

    pub fn shift_address_backward(&mut self, offset: u8) {
        self.address.shift_forward(offset)
    }

    pub fn get_t(&self) -> &VarType {
        &self.t
    }

    pub fn get_val(&self) -> &String {
        &self.val
    }

    pub fn get_int_val(&self) -> Result<i64, &'static str> {
        match self.val.parse::<i64>() {
            Ok(s) => Ok(s),
            Err(_) => Err("Variable could not be parsed into an int."),
        }
    }

    pub fn get_float_val(&self) -> Result<f64, &'static str> {
        match self.val.parse::<f64>() {
            Ok(s) => Ok(s),
            Err(_) => Err("Variable could not be parsed into a float."),
        }
    }

    pub fn get_bool_val(&self) -> Result<bool, &'static str> {
        match self.val.parse::<bool>() {
            Ok(s) => Ok(s),
            Err(_) => Err("Variable could not be parsed into a boolean."),
        }
    }

    pub fn strip_down(self) -> StripVar {
        StripVar {
            val: self.val,
            id: self.id,
        }
    }
}

pub struct StripVar {
    val: String,
    id: String,
}

impl StripVar {
    pub fn new(val: &str, id: &str) -> Self {
        Self {
            val: val.to_string(),
            id: id.to_string(),
        }
    }

    pub fn get_val(self) -> String {
        self.val
    }

    pub fn get_id(self) -> String {
        self.id
    }

    pub fn to_var(self, address: Address) -> Var {
        Var::new(self.val.as_str(), self.id.as_str(), address)
    }
}
