pub struct CustomString {
    value: String,
    name: String,
    reference: bool,
    mutable: bool,
}

impl CustomString {
    pub fn new(value: String, name: String, mutable: bool) -> Self {
        Self {
            value,
            name,
            reference: true,
            mutable,
        }
    }

    pub fn get_value_immutable(&self) -> &String {
        &self.value
    }

    pub fn get_value_mutable(&mut self) -> &mut String {
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

    pub fn string_length(self) -> usize {
        self.value.len()
    }
}

