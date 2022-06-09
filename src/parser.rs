//Possible operations assigned to an expression
pub enum Operation {
    PRINT,
    ASSIGN,
    ADD,
    SUB,
    MULT,
    DIV,
    EQ,
    NOTEQ,
    GTHAN,
    LTHAN,
    NONE,
    ERR,
}

pub enum ExpressionValue {
    //recursive type must be boxed
    Nested(Box<Expression>),
    //any variable name or value which will be replaced during evaluation
    Value(String),
    //for exceptions
    None,
}

//A line that has been read and broken down into the proper operation
//Expression has the assigned operation and a lefthand and righthand side
pub struct Expression {
    operation: Operation,
    lhs: ExpressionValue,
    rhs: ExpressionValue,
    evaluated: bool,
}

impl Expression {
    pub fn new(operation: Operation, lhs: ExpressionValue, rhs: ExpressionValue, evaluated: bool) -> Self {
        Self { 
            operation, 
            lhs,
            rhs,
            evaluated,
        }
    }
}