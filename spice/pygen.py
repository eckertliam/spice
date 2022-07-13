def tabs(scope):
    return scope * "\t"

def var_def(id, value, scope):
    return tabs(scope) + id + " = " + value + "\n"

def var_del(id, scope):
    return tabs(scope) + "del " + id + "\n"

def param_str(params):
    index = 0
    p_str = ""
    for p in params:
        if index == len(params):
            p_str += p
        else:
            p_str += p + ", "
    return p_str

def func_def(id, body, params=None):
    if params != None:
        return "def " + id + "(" + param_str(params) + "):\n" + body
    else:
        return "def " + id + "():\n" + body

def func_call(id, scope, params=None):
    if params != None:
        return tabs(scope) + id + "(" + param_str(params) + ")\n"
    else:
        return tabs(scope) + id + "()\n"

def binary_op(lhs, rhs, operation):
    return lhs + operation + rhs

def add(lhs, rhs):
    return binary_op(lhs, rhs, " + ")

def sub(lhs, rhs):
    return binary_op(lhs, rhs, " - ")

def div(lhs, rhs):
    return binary_op(lhs, rhs, " / ")

def mult(lhs, rhs):
    return binary_op(lhs, rhs, " * ")

def equal(lhs, rhs):
    return binary_op(lhs, rhs, " == ")

def gthan(lhs, rhs):
    return binary_op(lhs, rhs, " > ")

def gthaneq(lhs, rhs):
    return binary_op(lhs, rhs, " >= ")

def lthan(lhs, rhs):
    return binary_op(lhs, rhs, " < ")

def lthaneq(lhs, rhs):
    return binary_op(lhs, rhs, " <= ")

def pyand(lhs, rhs):
    return binary_op(lhs, rhs, " and ")

def pyor(lhs, rhs):
    return binary_op(lhs, rhs, " or ")

def xor(lhs, rhs):
    return "(" + lhs +  " and not " + rhs + ") or (not " + lhs + " and " + rhs + ")"

