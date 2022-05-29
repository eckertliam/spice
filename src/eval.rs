use std::error::Error;

use crate::numbers;
use crate::custom_string;
use crate::variable_table;
use crate::parser;

use numbers::Integer as Integer;
use numbers::Double as Double;
use numbers::Float as Float;
use custom_string::CustomString as CustomString;
use variable_table::VariableTable as VariableTable;
use parser::Operation as Operation;
use parser::Expression as Expression;
use parser::ExpressionValue as ExpressionValue;

pub enum EvalResult {
    Value(String),
    Nested(Expression),
    Error(String),
}
pub struct VirtualMachine {
    table: VariableTable,
    //vec of expressions to be evaluated
    expression_tree: Vec<Expression>,
}

impl VirtualMachine {
    pub fn init() -> Self {
        Self {
            table: VariableTable::new(),
            expression_tree: vec![],
        }
    }

    pub fn push_expression(&mut self, expr: Expression) {
        self.expression_tree.push(expr)
    }

    //Pops next expression to be evaluated
    pub fn pop(&mut self) -> Result<Expression, &str> {
        if self.expression_tree.len() > 0 {
            Ok(self.expression_tree.remove(0))
        }else{
            Err("Expression tree is empty.")
        }
    }

    pub fn peak_next_expression(&self) -> Result<&Expression, &str> {
        if self.expression_tree.len() > 0 {
            Ok(&self.expression_tree[0])
        }else{
            Err("Expression tree is empty.")
        }
    }
    
}