from enum import Enum, auto
from typer import Type, T

class SymbolTable:
    def __init__(self) -> None:
        self.vars: list[Var] = []

    def var_exists(self, varid):
        for var in self.vars:
            if var.id == varid:
                return True
        return False
    
    def get_var(self, varid):
        for var in self.vars:
            if var.id == varid:
                return var
        return None
    
    def new_var(self, val, varid):
        var = Var(val, varid)
        self.vars.append(var)
    
    def remove_var(self, varid):
        for var in self.vars:
            if var.id == varid:
                self.vars.remove(var)

    def new_array(self, val, arrid):
        array = Array(val, arrid)
        self.vars.append(array)
    

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
        # variable type of the operands
        self.type = None

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
        self.type = None

# Constant literal node
class Constant:
    def __init__(self, value) -> None:
        self.value = value
        self.type = None#placeholder until typechecking is implemented

    def check(self):
        self.type = Type.Check.atom(self.value)

   

# Variable ast node
class Var:
    def __init__(self, value, varid: str) -> None:
        self.value = value
        self.type = None
        self.id = varid

# variable declaration node
class VarDec:
    def __init__(self, varid: str, value) -> None:
        # a string naming the var
        self.id = varid
        # the value could be anything from addition nodes to a constant
        self.value = value
        # type of var
        self.type = None

class ArrayDec:
    def __init__(self, vals: list[str], arrid: str) -> None:
        self.length = len(vals)        
        self.type = None
        self.array = vals
        self.id = arrid

class Array:
    def __init__(self, vals: list[str], arrid: str) -> None:
        self.length = len(vals)
        self.type = None
        self.id = arrid
        self.array = vals

