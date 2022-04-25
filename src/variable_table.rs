use crate::numbers;
use numbers::Integer as Integer;
use numbers::Double as Double;
use numbers::Float as Float;

pub struct VariableTable {
    IntegerTable: Vec<Integer>,
    DoubleTable: Vec<Double>,
    FloatTable: Vec<Float>,
}

impl VariableTable {
    pub fn new() -> Self {
        Self {
            IntegerTable: vec![],
            DoubleTable: vec![],
            FloatTable: vec![],
        }
    }

    pub fn remove_dereferenced_variables(&mut self) {
        self.IntegerTable.retain(|v| *v.get_reference_immutable() == true);
        self.DoubleTable.retain(|v| *v.get_reference_immutable() == true);
        self.FloatTable.retain(|v| *v.get_reference_immutable() == true);
    }

    pub fn search_integer_table(&self, predicate: String) -> Option<&Integer> {
        self.IntegerTable.iter().find(|v| v.get_name_immutable() == predicate)
    }

}