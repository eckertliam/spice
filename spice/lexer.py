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


# adds spaces around each SEPERATOR
def expand_seperators(chars: str, index=0):
    if type(chars) == list[str]:
        if index != 0:
            if chars[index] in SEPERATORS:
                if chars[index - 1] != " ":
                    chars.insert(index, " ")
                    index += 1
                if chars[index + 1] != " ":
                    chars.insert(index + 1, " ")
        index += 1
        if index == len(chars) - 1:
            return chars
    else:
        chars = list(chars)
    return expand_seperators(chars, index)

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
        front1, front2, back = case.pop(0), case.pop(0), case.pop(-1)
        front = front1 + front2
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


