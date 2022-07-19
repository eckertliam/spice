pub enum VarType {
    Int,
    Float,
    Char,
    Bool,
    Str,
    Error,
}

const NUMBERS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
const PERIOD: char = '.';
const DOUBLE_QUOTE: char = '"';
const QUOTE: char = '\'';
const ALPHABET_LOWER: &'static str = "abcdefghijklmnopqrstuvwxyz";
const ALPHABET_UPPER: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

impl VarType {
    pub fn new(case: &str) -> Result<Self, &'static str> {
        if Self::is_bool(case) {
            return Ok(Self::Bool);
        } else if Self::is_char(case) {
            return Ok(Self::Char);
        } else if Self::is_str(case) {
            return Ok(Self::Str);
        } else if Self::is_int(case) {
            return Ok(Self::Int);
        } else if Self::is_float(case) {
            return Ok(Self::Float);
        } else {
            return Err("Unknown type");
        }
    }

    fn is_float(case: &str) -> bool {
        let mut period_count = 0;
        for ch in case.chars() {
            if ch == PERIOD {
                period_count += 1;
                if period_count > 1 {
                    return false;
                }
            } else if !NUMBERS.contains(&ch) {
                return false;
            }
        }
        // if all chars were numbers and one was a period returns true
        period_count == 1
    }

    fn is_str(case: &str) -> bool {
        let access: Vec<char> = case.chars().collect();
        // if the first char and last char are both equal quotes and has a length gthan 3 it is a str literal
        case.len() != 3 && access[0] == DOUBLE_QUOTE
            || access[0] == QUOTE && &access[0] == access.last().unwrap()
    }

    fn is_char(case: &str) -> bool {
        let access: Vec<char> = case.chars().collect();
        // if the front and tail are quotes and the legnth is 3 its a char
        case.len() == 3 && access[0] == DOUBLE_QUOTE
            || access[0] == QUOTE && &access[0] == access.last().unwrap()
    }

    fn is_int(case: &str) -> bool {
        let access: Vec<char> = case.chars().collect();
        for ch in access {
            if !NUMBERS.contains(&ch) {
                return false;
            }
        }
        // if all chars were numbers return true
        true
    }

    fn is_bool(case: &str) -> bool {
        case == "true" || case == "false"
    }
}
