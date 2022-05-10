use crate::numbers;
use crate::custom_string;

use numbers::Integer as Integer;
use numbers::Double as Double;
use numbers::Float as Float;
use custom_string::CustomString as CustomString;

pub struct VariableTable {
    IntegerTable: Vec<Integer>,
    DoubleTable: Vec<Double>,
    FloatTable: Vec<Float>,
    StringTable: Vec<CustomString>,
}

impl VariableTable {
    pub fn new() -> Self {
        Self {
            IntegerTable: vec![],
            DoubleTable: vec![],
            FloatTable: vec![],
            StringTable: vec![],
        }
    }

    pub fn remove_dereferenced_variables(&mut self) {
        self.IntegerTable.retain(|v| *v.get_reference_immutable() == true);
        self.DoubleTable.retain(|v| *v.get_reference_immutable() == true);
        self.FloatTable.retain(|v| *v.get_reference_immutable() == true);
        self.StringTable.retain(|v| *v.get_reference_immutable() == true);
    }

    pub fn get_integer_value(&self, predicate: String) -> Option<&Integer> {
        self.IntegerTable.iter().find(|v| v.get_name_immutable() == predicate)
    }

    pub fn check_integer_exists(&self, predicate: String) -> bool {
        let search = self.get_integer_value(predicate);
        match search {
            Some(_) => true,
            None => false,
        }
    }

    pub fn insert_new_int(&mut self, new_int: Integer) -> Result<(), String> {
        if self.check_integer_exists(new_int.get_name_immutable().to_string()) == true {
            let err_msg: String = String::from("Integer name ".to_string() + new_int.get_name_immutable() + " already exists.");
            Err(err_msg)
        }else{
            self.IntegerTable.push(new_int);
            Ok(())
        }
    }

    pub fn get_float_value(&self, predicate: String) -> Option<&Float> {
        self.FloatTable.iter().find(|v| v.get_name_immutable() == predicate)
    }

    pub fn check_float_exists(&self, predicate: String) -> bool {
        let search = self.get_float_value(predicate);
        match search {
            Some(_) => true,
            None => false,
        }
    }

    pub fn insert_new_float(&mut self, new_float: Float) -> Result<(), String> {
        if self.check_float_exists(new_float.get_name_immutable().to_string()) == true {
            let err_msg: String = String::from("Float name ".to_string() + new_float.get_name_immutable() + " already exists.");
            Err(err_msg)
        }else{
            self.FloatTable.push(new_float);
            Ok(())
        }
    }

    pub fn get_double_value(&self, predicate: String) -> Option<&Double> {
        self.DoubleTable.iter().find(|v| v.get_name_immutable() == predicate)
    }

    pub fn check_double_exists(&self, predicate: String) -> bool {
        let search = self.get_double_value(predicate);
        match search { 
            Some(_) => true,
            None => false,
        }
    }

    pub fn insert_new_double(&mut self, new_double: Double) -> Result<(), String> {
        if self.check_double_exists(new_double.get_name_immutable().to_string()) == true {
            let err_msg: String = String::from("Double name ".to_string() + new_double.get_name_immutable() + " already exists.");
            Err(err_msg)
        }else{
            self.DoubleTable.push(new_double);
            Ok(())
        }
    }

    pub fn get_string_value(&self, predicate: String) -> Option<&CustomString> {
        self.StringTable.iter().find(|v| v.get_name_immutable() == predicate)
    }

    pub fn check_string_exists(&self, predicate: String) -> bool {
        let search = self.get_string_value(predicate);
        match search {
            Some(_) => true,
            None => false,
        }
    }

    pub fn insert_new_string(&mut self, new_string: CustomString) -> Result<(), String> {
        if self.check_integer_exists(new_string.get_name_immutable().to_string()) == true {
            let err_msg: String = String::from("Integer name ".to_string() + new_string.get_name_immutable() + " already exists.");
            Err(err_msg)
        }else{
            self.StringTable.push(new_string);
            Ok(())
        }
    }

}