pub struct Line {
    tokens: Vec<String>,
}

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

pub struct Expression {
    tokens: Vec<String>,
    operation: Operation,
}
