use std::fs::File;
use std::io::Read;

//check if string is a number
fn is_number(s: &str) -> bool {
    s.parse::<f64>().is_ok()
}

pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Colon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    SlashSlash,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier(String),
    String(String),
    Number(f64),

    // Keywords.
    Else,
    True,
    False,
    If,
    Elif,
    Null,
    Var,
    Print,
    Return,
    While,
    Eol,
    Eof,
}

impl TokenType {
    // convert string to token type
    pub fn from_str(s: &str) -> TokenType {
        match s {
            "(" => TokenType::LeftParen,
            ")" => TokenType::RightParen,
            "{" => TokenType::LeftBrace,
            "}" => TokenType::RightBrace,
            "," => TokenType::Comma,
            "." => TokenType::Dot,
            "-" => TokenType::Minus,
            "+" => TokenType::Plus,
            ";" => TokenType::Semicolon,
            ":" => TokenType::Colon,
            "/" => TokenType::Slash,
            "*" => TokenType::Star,
            "!" => TokenType::Bang,
            "!=" => TokenType::BangEqual,
            "=" => TokenType::Equal,
            "//" => TokenType::SlashSlash,
            "==" => TokenType::EqualEqual,
            ">" => TokenType::Greater,
            ">=" => TokenType::GreaterEqual,
            "<" => TokenType::Less,
            "<=" => TokenType::LessEqual,
            "else" => TokenType::Else,
            "true" => TokenType::True,
            "false" => TokenType::False,
            "if" => TokenType::If,
            "elif" => TokenType::Elif,
            "null" => TokenType::Null,
            "var" => TokenType::Var,
            "print" => TokenType::Print,
            "return" => TokenType::Return,
            "while" => TokenType::While,
            _ => TokenType::Identifier(s.to_string()),
        }
    }

    // converts string to Number or String TokenType
    pub fn from_str_literal(s: &str) -> TokenType {
        if is_number(s) {
            TokenType::Number(s.parse::<f64>().unwrap())
        } else {
            TokenType::String(s.to_string())
        }
    }
}

pub struct Token {
    token_type: TokenType,
    line:  usize,
    column: usize,
}

impl Token {
    pub fn new(token_type: TokenType, line: usize, column: usize) -> Token {
        Token {
            token_type,
            line,
            column,
        }
    }
    // get token type
    fn get_token_type(&self) -> &TokenType {
        &self.token_type
    }

    // get position in file
    pub fn get_position(&self) -> (usize, usize) {
        (self.line, self.column)
    }
}

//read file to string
pub fn read_file(path: &str) -> String {
    let mut file = File::open(path).expect("file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("something went wrong reading the file");
    contents
}
