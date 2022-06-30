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
DIGITS = str([num for num in range(0, 9)])



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

def seperate_comment(chars: str):
    chars = list(chars)
    r_list = []
    index = 0
    current = ""
    while index < len(chars) - 1:
        current = chars[index] + chars[index + 1]
        if current == "//":
            r_list.append(" ")
            r_list.append(current)
            r_list.append(" ")
            index += 1
        else:
            r_list.append(chars[index])
        index += 1
    return r_list

def seperate(string):
    r = expand_seperators(string)
    r = seperate_comment(r)
    print(r)
    r = psplit(r)
    r.append("\n")
    return r
    

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


def is_number_literal(case: str):
    case = list(case)
    period_count = 0
    for char in case:
        if char == ".":
            period_count+=1
        if char not in DIGITS and char != "." :
            return False
    return period_count <= 1

def is_operator(case):
    return case in OPERATORS

def is_keyword(case):
    return case in KEYWORDS

def is_identifier(case):
    case = list(case)
    for char in case:
        if char not in ALPHABET and char not in DIGITS:
            return False
    return True

def lex(string):
    stream = seperate(string)
    index = 0
    current = ""
    tokens = []
    while index < len(stream):
        current = stream[index]
        # comment catching subloop
        if current == "//":
            tokens.append(Token(Token_Type.COMMENT, "//"))
            index += 1
            while current != "\n":
                if index < len(stream):
                    current = stream[index]
                    tokens.append(Token(Token_Type.COMMENT, current))
                    index += 1
                else:
                    raise ValueError("Index is greater than the size of the pre-token stream: a comment might not end with an endline")
            tokens.append(Token(Token_Type.WHITESPACE, "\n"))
        elif current in KEYWORDS:
            tokens.append(Token(Token_Type.KEYWORD, current))
        elif current in WHITESPACE:
            tokens.append(Token(Token_Type.WHITESPACE, current))
        elif is_bool(current):
            tokens.append(Token(Token_Type.BOOL, current))
        elif is_operator(current):
            tokens.append(Token(Token_Type.OPERATOR, current))
        elif is_string_literal(current):
            tokens.append(Token(Token_Type.STRING_LITERAL, current))
        elif is_number_literal(current):
            tokens.append(Token(Token_Type.NUMBER_LITERAL, current))
        elif is_identifier(current):
            tokens.append(Token(Token_Type.IDENTIFIER, current))
        else:
            raise ValueError("Unkown input of ", current)
        index += 1
    return tokens
