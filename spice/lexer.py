from enum import Enum, auto
from string import ascii_lowercase, digits

# Operators of the language
OPERATORS = ["=", "+", "-", "*", "/", "==", "=!", "=>", "=<", "&&", "||", "<", ">", "!", "|", "&"]

# Keywords of the language
KEYWORDS = ["fn", "let", "if", "elif", "else", "while", "true", "false", "enum"]

# Seperators of the language
SEPERATORS = ["(", ")", ";", ":", "[", "]", "{", "}"]

# Whitespace chars
WHITESPACE = [" ", "\t", "\n", "\v", "\r", "\f"]

# Alphebetical chars
ALPHABET = []
for letter in list(ascii_lowercase):
    ALPHABET.append(letter)
    ALPHABET.append(letter.capitalize())

# Number chars
DIGITS = str([num for num in range(1, 9)])



# Token_Type enumerator
class Token_Type(Enum):
    IDENTIFIER = auto()
    KEYWORD = auto()
    SEPERATOR = auto()
    OPERATOR = auto()
    STRING_LITERAL = auto()
    NUMBER_LITERAL = auto()
    BOOL = auto()
    COMMENT = auto()
    WHITESPACE = auto()


class Token:
    def __init__(self, token_type, value=""):
        self.token_type = token_type
        self.value = value

# had to make my own split since python throws out delimiters
def psplit(string):
    return_ls = []
    ls_string = list(string)
    current = ""
    for char in string:
        if char not in WHITESPACE:
            current += char
        else:
            if current != "":
                return_ls.append(current)
                current = ""
            return_ls.append(char)
    return return_ls


# adds spaces around each SEPERATOR
def expand_seperators(chars: str):
    chars = list(chars)
    r_list = []
    index = 0
    for char in chars:
        if char in SEPERATORS:
            if r_list[-1] != " ":
                r_list.append(" ")
            r_list.append(char)
            if chars[index + 1] != " ":
                r_list.append(" ")
        else:
            r_list.append(char)
        index += 1
    return r_list

def seperate(string):
    return psplit(expand_seperators(string))
    

# checks that front and back match, are quote marks and the are no other terminating quote marks
def is_string_literal(case: str):
    if len(case) >= 2:
        case = list(case)
        front, back = case.pop(0), case.pop(-1)
        if front != back or front not in ["'", '"'] or front in case:
            return False
        else:
            return True
    else:
        return False

def is_bool(case):
    return case == "true" or case == "false"

def is_comment(case: str):
    case = list(case)
    if len(case) > 3:
        front, back = case.pop(0) + case.pop(0), case.pop(-1)
        return front == "//" and back == "\n"
    else:
        return False

def is_number_literal(case: str):
    case = list(case)
    period_count = 0
    for char in case:
        if char == ".":
            period_count+=1
        if char not in DIGITS and char != "." :
            print(char)
            return False
    return period_count <= 1

def is_operator(case):
    return case in OPERATORS

def is_keyword(case):
    return case in KEYWORDS

