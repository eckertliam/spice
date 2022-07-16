enum VarType {
    Int,
    Float,
    Byte,
    Char,
    Bool,
    Str,
}

const NUMBERS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
const PERIOD: char = '.';
const DOUBLE_QUOTE: char = '"';
const QUOTE: char = '\'';
const ALPHABET_LOWER: &'static str = "abcdefghijklmnopqrstuvwxyz";
const ALPHABET_UPPER: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

impl VarType {
    fn is_float(case: String) -> bool {
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
        return period_count == 1;
    }

    fn is_str(case: String) -> bool {
        let access: Vec<char> = case.chars().collect();
        // if the first char and last char are both equal quotes the literal is a string
        if access[0] == DOUBLE_QUOTE || access[0] == QUOTE && &access[0] == access.last().unwrap() {
            return true;
        }
        return false;
    }

    fn is_char(self, case: String) -> bool {
        // if the front and tail are quotes and the legnth is 3 its a char
        case.len() == 3 && VarType::is_str(case)
    }

    fn is_int(case: String) -> bool {
        let access: Vec<char> = case.chars().collect();
        for ch in access {
            if !NUMBERS.contains(&ch) {
                return false;
            }
        }
        // if all chars were numbers return true
        return true;
    }

    fn is_bool(case: String) -> bool {
        case == "true" || case == "false"
    }

    fn is_byte(case: String) -> bool {
        let split: Vec<char> = case.chars().collect();
        let mut index: usize = 0;
        // returns true if the length of the string is 10 char 0 is 0 and char 1 is b
        // in spice bytes are written in binary 0b00000000 format
        if case.len() == 10 && split[0] == '0' && split[1] == 'b' {
            for ch in split {
                if index > 1 && ch != '0' && ch != '1' {
                    return false;
                }
                index += 1;
            }
        }
        return true;
    }
}
