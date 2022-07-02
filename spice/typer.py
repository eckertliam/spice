from lexer import DIGITS, ALPHABET
from enum import Enum, auto

class T(Enum):
    Int = auto()
    Float = auto()
    Char = auto()
    String = auto()
    Bool = auto()

class Type:
    # the atom function is subclasses verifies the type for an atom 
    class Int:
        def atom(val: str):
            for char in val:
                if char not in DIGITS:
                    return False
            return True
    
    class Float:
        def atom(val: str):
            period = False
            for char in val:
                if char not in DIGITS or char != ".":
                    return False
                if char == ".":
                    period = True
            return period

    class Char:
        def atom(val: str):
            # val must be 3 characters for the quotes and then the char
            if len(val) == 3:
                chars = list(val)
                if chars[0] == chars[-1] and chars[0] == "'" or chars[0] == '"':
                    return True
            return False
    
    class String:
        def atom(val: str):
            if len(val) > 2:
                chars = list(val)
                if chars[0] == chars[-1] and chars[0] == "'" or chars[0] == '"':
                    return True
            return False

    class Bool:
        def atom(val: str):
            return val == "true" or val == "false"

    
    class Check:
        def atom(val: str):
            if Type.Bool.atom(val):
                return T.Bool
            elif Type.Char.atom(val):
                return T.Char
            elif Type.String.atom(val):
                return T.String
            elif Type.Int.atom(val):
                return T.Int
            elif Type.Float.atom(val):
                return T.Float
            else:
                raise TypeError("Unkown type for ", val)
        
        # used to verify all types in a list are of the same type
        # can put all values from an expression into a list and run it through this function to verify an expression
        def verify(vals: list[str], listid: str):
            rt = Type.Check.atom(vals[0])
            for val in vals:
                if Type.Check.atom(val) != rt:
                    raise TypeError("Type does not match in list ", listid)
            return rt
        
        

