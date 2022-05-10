pub struct Float {
    value: f32,
    name: String,
    reference: bool,
    mutable: bool,
}

impl Float {
    pub fn new(value: f32, name: String, mutable: bool) -> Self {
        Self {
            value,
            name,
            reference: true,
            mutable,
        }
    }

    pub fn get_value_immutable(&self) -> &f32 {
        &self.value
    }

    pub fn get_value_mutable(&mut self) -> &mut f32 {
        &mut self.value
    }

    pub fn get_name_immutable(&self) -> &str {
        &self.name
    }

    pub fn get_name_mutable(&mut self) -> &mut String {
        &mut self.name
    }

    pub fn get_reference_immutable(&self) -> &bool {
        &self.reference
    }

    pub fn get_reference_mutable(&mut self) -> &mut bool {
        &mut self.reference
    }

    pub fn dereference(&mut self) {
        self.reference = false;
    }

    pub fn get_mutability_immutable(&self) -> &bool {
        &self.mutable
    }

    pub fn to_integer(self) -> Integer {
        Integer::new(self.value as i32, self.name, self.mutable)
    }

    pub fn to_double(self) -> Double {
        Double::new(self.value as f64, self.name, self.mutable)
    }

    pub fn from_integer(int_in: Integer) -> Self {
        Self::new(int_in.value as f32, 
                int_in.name, 
                int_in.mutable)
    }

    pub fn from_double(double_in: Double) -> Self {
        Self::new(double_in.value as f32,
                double_in.name, 
                double_in.mutable)
    }

    pub fn from_rust_string(string_in: String, name: String, mutable: bool) -> Result<Self, String> {
        let try_parse = string_in.parse::<f32>();
        match try_parse {
            Ok(f) => Ok(Self::new(f, name, mutable)),
            Err(_) => Err(String::from("Could not parse ".to_string() + &name + " to float."))
        }
    }
}

pub struct Double {
    value: f64,
    name: String,
    reference: bool,
    mutable: bool
}

impl Double {
    pub fn new(value: f64, name: String, mutable: bool) -> Self {
        Self {
            value,
            name,
            reference: true,
            mutable,
        }
    }

    pub fn get_value_immutable(&self) -> &f64 {
        &self.value
    }

    pub fn get_value_mutable(&mut self) -> &mut f64 {
        &mut self.value
    }

    pub fn get_name_immutable(&self) -> &str {
        &self.name
    }

    pub fn get_name_mutable(&mut self) -> &mut String {
        &mut self.name
    }

    pub fn get_reference_immutable(&self) -> &bool {
        &self.reference
    }

    pub fn get_reference_mutable(&mut self) -> &mut bool {
        &mut self.reference
    }

    pub fn dereference(&mut self) {
        self.reference = false;
    }

    pub fn get_mutability_immutable(&self) -> &bool {
        &self.mutable
    }

    pub fn to_integer(self) -> Integer {
        Integer::new(self.value as i32, self.name, self.mutable)
    }

    pub fn to_float(self) -> Float {
        Float::new(self.value as f32, self.name, self.mutable)
    }

    pub fn from_integer(int_in: Integer) -> Self {
        Self::new(int_in.value as f64, int_in.name, int_in.mutable)
    }

    pub fn from_float(float_in: Float) -> Self {
        Self::new(float_in.value as f64, float_in.name, float_in.mutable)
    }

    pub fn from_rust_string(string_in: String, name: String, mutable: bool) -> Result<Self, String> {
        let try_parse = string_in.parse::<f64>();
        match try_parse {
            Ok(d) => Ok(Self::new(d, name, mutable)),
            Err(_) => Err(String::from("Could not parse ".to_string() + &name + " to double."))
        }
    }
}

pub struct Integer {
    value: i32,
    name: String,
    reference: bool,
    mutable: bool,
}

impl Integer {
    pub fn new(value: i32, name: String, mutable: bool) -> Self {
        Self {
            value,
            name,
            reference: true,
            mutable,
        }
    }

    pub fn get_value_immutable(&self) -> &i32 {
        &self.value
    }

    pub fn get_value_mutable(&mut self) -> &mut i32 {
        &mut self.value
    }

    pub fn get_name_immutable(&self) -> &str {
        &self.name
    }

    pub fn get_name_mutable(&mut self) -> &mut String {
        &mut self.name
    }

    pub fn get_reference_immutable(&self) -> &bool {
        &self.reference
    }

    pub fn get_reference_mutable(&mut self) -> &mut bool {
        &mut self.reference
    }

    pub fn dereference(&mut self) {
        self.reference = false;
    }

    pub fn get_mutability_immutable(&self) -> &bool {
        &self.mutable
    }

    pub fn to_float(self) -> Float {
        Float::new(self.value as f32, self.name, self.mutable)
    }

    pub fn to_double(self) -> Double {
        Double::new(self.value as f64,  self.name, self.mutable)
    }

    pub fn from_float(float_in: Float) -> Self {
        Self::new(float_in.value as i32, float_in.name, float_in.mutable)
    }

    pub fn from_double(double_in: Double) -> Self {
        Self::new(double_in.value as i32, double_in.name, double_in.mutable)
    }

    pub fn from_rust_string(string_in: String, name: String, mutable: bool) -> Result<Self, String> {
        let try_parse = string_in.parse::<i32>();
        match try_parse {
            Ok(i) => Ok(Self::new(i, name, mutable)),
            Err(_) => Err(String::from("Could not parse ".to_string() + &name + " to integer."))
        }
    }
}

