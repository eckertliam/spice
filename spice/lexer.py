from enum import Enum, auto
from lib2to3.pgen2.token import NUMBER
from string import ascii_lowercase, digits

# Operators of the language
OPERATORS = ["=", "+", "-", "*", "/", "==", "=!", "=>", "=<", "&&", "||", "<", ">", "!", "|", "&"]

# Keywords of the language
KEYWORDS = ["fn", "let", "if", "elif", "else", "while", "true", "false", "enum"]

# Seperators of the language
SEPERATORS = ["(", ")", ";", ":", "[", "]", "{", "}", '"', "'"]

# Whitespace chars
WHITESPACE = [" ", "\t", "\n", "\v", "\r", "\f"]

# Alphebetical chars
ALPHABET = []
for letter in list(ascii_lowercase):
    ALPHABET.append(letter)
    ALPHABET.append(letter.capitalize())

# Number chars
DIGITS = [num for num in str(range(1, 9))]



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
def expand_seperators(chars, index=0):
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

# recursively checks that the case is an identifier
def is_identifier(case):
    if type(case) == list[str]:
        char = case.pop(0)
        if char in ["'", '"'] or char not in ALPHABET and char not in DIGITS:
            return False
    else:
        if case in KEYWORDS:
            return False
        case = list(case)
    return is_identifier(case)


# checks that front and back match, are quote marks and the are no other terminating quote marks
def is_string_literal(case):
    front, back = case.pop(0), case.pop(-1)
    if front != back or front not in ["'", '"'] or front in case:
        return False
    else:
        return True

def is_number_literal(case):
    if type(case) == list[str]:
        char = case.pop(0)
        if char not in digits or char != ".":
            return False
    else:
        case = list(case)
        if "." in case:
            periods = 0
            for char in case:
                if char == ".":
                    periods+=1
            if periods > 1:
                return False
    return is_number_literal(case)

