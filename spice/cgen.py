import sys

# custom error system
def custom_error(message, terminal=False):
    print(message)
    if terminal == True:
        sys.exit()

# explicit endline for readability
def endline():
    return "\n"

# c endline 
def cendline():
    return ";\n"

# explicit tab for readability
def tab():
    return "\t"

# returns necessary \t for specified depth
def depth(n):
    return tab() * n

# generate an operation with two operands
def binary_op(n1, n2, op):
    return str(n1) + op + str(n2)

# expands into additon c code
def add(n1, n2):
    return binary_op(n1, n2, " + ")

# expands into subtraction c code
def sub(n1, n2):
    return binary_op(n1, n2, " - ")

# expands into multiplication c coode
def mult(n1, n2):
    return binary_op(n1, n2, " * ")

# expands into division c code
def div(n1, n2, d):
    return binary_op(n1, n2, " / ")

# structure for function parameters
class FuncParam:
    def __init__(self, name, type):
        self.name = name
        self.type = type

# expands into c code for a function
# parameters use the FuncParam class
def func_def(return_t, name, body, params=None):
    rval = return_t + " " + name + "("
    if params != None:
        length = len(params)
        if length == 1:
            rval += params[0].type + " " + params[0].name
        else:
            for param in params:
                rval += param.type + " " + param.name
                if length != 1:
                    rval += ", "
                length -= 1
    rval += ") " + endline() + "{" + endline() + body + endline() + "}" + endline()
    return rval

# expands into c code for a function call
def func_call(name, params=None):
    rval = name + "("
    if params != None:
        length = len(params)
        if length == 1:
            rval += str(params[0])
        else:
            for param in params:
                rval += str(param)
                if length != 1:
                    rval += ", "
                length -= 1    
    rval += ")"
    return rval

# generic expansion for conditionals
def binary_cond(n1, n2, op):
    return "(" + binary_op(str(n1), str(n2), op) + ")"

# unary with only one parameter where the parameter is either a bool or number
def unary_cond(n):
    return "(" + str(n) + ")"

# generates an if conditional statement with body
def if_cond(body, n1, n2=None, op=None):
    rval = "if "
    if n2 and op != None:
        rval += binary_cond(n1, n2, op)
    elif n2 and op == None:
        rval += unary_cond(n1)
    else:
        return custom_error("Cannot determine whether conditional is unary or binary", True)
    rval += endline() + "{" + endline() + body + endline() + "}" + endline()
    return rval

# generate an else if conditional statement with body
def else_if_cond(body, n1, n2=None, op=None):
    return "else " + if_cond(body, n1, n2, op)

# generate an else statement with a body
def else_cond(body):
    return "else" + endline() + "{" + endline() + body + endline() + "}" + endline()

# generates a while statement with a body
def while_cond(body, n1, n2=None, op=None):
    rval = "while "
    if n2 and op != None:
        rval += binary_cond(n1, n2, op)
    elif n2 and op == None:
        rval += unary_cond(n1)
    else:
        return custom_error("Cannot determine whether conditional is unary or binary", True)
    rval += endline() + "{" + endline() + body + endline() + "}" + endline()
    return rval

# generates code for return expression
def return_expr(body):
    return "return " + str(body) + cendline()

# generates c code for a variable declaration
def var_def(t, name, value=None):
    rval = t + " " + name
    if value != None:
        rval += " " + str(value)
    rval += cendline()
    return rval

# writes to file
def write_file(fname, text):
    f = open(fname, "w")
    f.write(text)
    f.close()

# expands to head import format
def header(name):
    return "#include " + name


# name of the struct and any fields in a list, fields cannot be empty
def struct(name, fields):
    rval = "struct " + name + " {" + endline()
    for field in fields:
        rval += field + cendline()
    rval += "}" + endline
    return rval

# expands into a c enum
# for the field dict if the value is none C will just assign a value
def cenum(name, field_dict):
    rval = name + " {"
    for key in field_dict:
        rval += str(key)
        if field_dict[key] != None:
            rval += " " + str(field_dict[key])
        rval += "," + endline()
    rval += "};" + endline()
    return rval