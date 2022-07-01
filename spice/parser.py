from enum import Enum, auto
from lib2to3.pgen2.token import EQUAL
from turtle import left
import lexer

# math binary Operation Type
class MathBinOpT(Enum):
    ADD = auto()
    SUB = auto()
    DIV = auto()
    MULT = auto()
    
# math binary operation node
class MathBinOp:
    def __init__(self, left, right, opt) -> None:
        # the node of the left hand side
        self.lhs = left
        # the node of the right hand side
        self.rhs = right
        # operation type
        self.opt = opt

# types of binary conditionals
class CondBinOpT(Enum):
    EQUAL = auto()
    LTHAN = auto()
    LTHANEQ = auto()
    GTHAN = auto()
    GTHANEQ = auto()
    NOTEQ = auto()
    # conditionals can contain other conditonals with and & or
    AND = auto()
    OR = auto()

class CondBinOp:
    def __init__(self, left, right, opt) -> None:
        self.lhs = left
        self.rhs = right
        self.opt = opt

# Constant literal node
class Constant:
    def __init__(self, value) -> None:
        self.value = value
        self.type = None#placeholder until typechecking is implemented
   

# Variable ast node
class Var:
    def __init__(self, value, varid: str) -> None:
        self.value = value
        self.type = None#placeholder until typechecking is implemented
        self.id = varid

# variable declaration node
class VarDec:
    def __init__(self, varid: str, value) -> None:
        # a string naming the var
        self.id = varid
        # the value could be anything from addition nodes to a constant
        self.value = value


