//Need this enum to have the generic var structs together in a vector
pub enum VarType {
    Float(Var<f64>),
    Int(Var<i32>),
    Char(Var<u8>),
}

pub struct Var<T> {
	value: T,
	name: String,
}

impl<T> Var<T> {
	fn new(value: T, name: String) -> Self {
		Self {
			value,
			name,
		}
	}

	pub fn new_float(value: f64, name: String) -> VarType {
		VarType::Float(Var::new(value, name))
	}

	pub fn new_int(value: i32, name: String) -> VarType {
		VarType::Int(Var::new(value, name))
	}

	pub fn new_char(value: u8, name: String) -> VarType {
		VarType::Char(Var::new(value, name))
	}

	pub fn get_value(&self) -> &T {
		&self.value
	}

	pub fn get_name(&self) -> &String {
		&self.name
	}

	pub fn int_to_float(var: Var<i32>) -> VarType {
		VarType::Float(
			Var::new(
				f64::from(*var.get_value()), 
				var.get_name().clone()
			)
		)
	}
}
