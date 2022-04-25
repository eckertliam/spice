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
}