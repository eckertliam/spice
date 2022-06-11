from enum import Enum, auto

# Operators of the language
OPERATORS = ["=", "+", "-", "*", "/", "==", "=!", "=>", "=<", "&&", "||", "<", ">"]

# Keywords of the language
KEYWORDS = ["func", "let"]

# Seperators of the language
SEPERATORS = ["(", ")", ";", ":", "[", "]", "{", "}"]

# Whitespace chars
WHITESPACE = [" ", "\t", "\n", "\v", "\r", "\f"]

# Alphebetical chars
ALPHABET = []
for letter in range('a', 'z'):
    ALPHABET.append(letter)
    ALPHABET.append(letter.capitalize())


def is_letter(in_str):
    # ensure in_str is a list
    if type(in_str) == list[str]:
        # makes sure there are elements in in_str
        if in_str:
            char = in_str.pop(0)
            if char not in ALPHABET:
                return False
            else:
                return is_letter(in_str)
        else:
            # return True once in_str has been consumed and hasn't raised a False
            return True
    else:
        new_str_set = [letter for letter in in_str]
        return is_letter(new_str_set)

# Number chars
DIGITS = [num for num in range('1', '9')]

# same structure as is_letter just for numbers
def is_num(in_num):
    if type(in_num) == list[str]:
        if in_num:
            char = in_num.pop(0)
            if char not in DIGITS:
                return False
            else:
                return is_num(in_num)
        else:
            return True
    else:
        new_num_set = [num for num in in_num]
        return is_letter(new_num_set)


# Token_Type enumerator
class Token_Type(Enum):
    IDENTIFIER = auto()
    KEYWORD = auto()
    SEPERATOR = auto()
    OPERATOR = auto()
    LITERAL = auto()
    COMMENT = auto()
    WHITESPACE = auto()

# Preset possible tokens array
POSSIBLE_TOKENS = [Token_Type.COMMENT, Token_Type.IDENTIFIER, Token_Type.KEYWORD, Token_Type.LITERAL, Token_Type.OPERATOR, Token_Type.SEPERATOR, Token_Type.WHITESPACE]

class Token:
    def __init__(self, token_type, value=""):
        self.token_type = token_type
        self.value = value
        return self


# converts a str into a list of chars
def list_chars(in_str):
    chars = [char for char in in_str]
    return chars

# adds spaces around each SEPERATOR
def expand_seperators(chars):
    for n in range(0, len(chars)):
        index = n
        previous = chars[n - 1]
        char = chars[n]
        future = chars[n + 1]
        if char in SEPERATORS:
            if previous != " ":
                chars.insert(index, " ")
                index += 1
            if future != " ":
                chars.insert(index + 1, " ")
    return chars

class Tokenizer:
    def __init__(self, chars) -> None:
        # chars to be tokenized
        self.chars = chars
        # current position in chars
        self.index = 0
        # current char
        self.current = self.chars[self.index]
        self.tokens = []

    # These two peaks functions might seem pointless but I find they increase readability
    def peak_last_char(self):
        return self.chars[self.index - 1]

    def peak_next_char(self):
        return self.chars[self.index + 1]
    
    # Adds the next char to current char
    def absorb_next_char(self):
        # increase index to accurately reflect the current
        self.index += 1
        if self.index < len(self.current):
            self.current += self.chars[self.index + 1]
        else:
            return None
    
    # Sets the next char as current
    def next_char(self):
        self.index += 1
        if self.index < len(self.current):
            self.current = self.chars[self.index]
        else:
            return None
    
    # Peaks the most recent token
    def peak_last_token(self):
        if len(self.tokens) > 0:
            return self.tokens[-1]
        else:
            return None

