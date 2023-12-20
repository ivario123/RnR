# Your EBNF for your Rust in Rust language

Try to capture the syntax for your RNR language by means of EBNF rules, see e.g. [EBNF](https://en.wikipedia.org/wiki/Extended_Backus%E2%80%93Naur_form.).

You don't need to worry about white spaces, we can assume them to be suppressed.

```typescript

// Parsing rules

// Literals

digit_excluding_zero = "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9";
digit = "0" | digit_excluding_zero;

letter_u = "A" | "B" | "C" | "D" | "E" | "F" | "G"
       | "H" | "I" | "J" | "K" | "L" | "M" | "N"
       | "O" | "P" | "Q" | "R" | "S" | "T" | "U"
       | "V" | "W" | "X" | "Y" | "Z";

letter_l = "a" | "b" | "c" | "d" | "e" | "f" | "g" 
       | "h" | "i" | "j" | "k" | "l" | "m" | "n" 
       | "o" | "p" | "q" | "r" | "s" | "t" | "u" 
       | "v" | "w" | "x" | "y" | "z" ;

letter = letter_u | letter_l;

// Types
type_i32   = "i32";
type_usize = "usize";
type_bool  = "bool";
type_unit  = "()";

type = type_i32 | type_bool | type_unit | type_usize;

uint = "0"|{digit};
int  = "0" | ["-"] , {digit};
bool = "true" | "false";

// Operations

add         = "+";
subtract    = "-";
multiply    = "*";
divide      = "/";
and         = "&&";
or          = "||";
not         = "!";
refference  = "&";
derefferece = "*";

// Parentheses
open_paren    = "(";
closing_paren = ")";

// Brackets
open_bracket  = "[";
close_bracket = "]";

// Curly bracers
open_block  = "{";
close_block = "}";

// Multi char identifiers for statements
if      = "if";
else    = "else";
let     = "let";
while   = "while";
static  = "static";
    

// Groupings

literal = int | bool;

numeric_operation = add | subtract | multiply | divide;
boolean_operation = and | or;

// Composite expressions
operation        = numeric_operation | boolean_operation;
binary_operation = (expression,operation,expression);


invert          = not, bool;
minus_op        = "-",int;
unary_op = invert | minus_op | "&" | "&mut" | "*";

unary_operation = unary_operator,expression;

parentheses     = (open_paren,{expression}, closing_paren);

identifier      = letter, {letter | digit | "_"};

if_then_else    = if, expression, block, "" | (else, block);

index           = identifier, open_bracket, [uint | identifier], close_bracket;
// For now there is not any way to differentiate between index and index_mut in ebnf form.
index_mut       = identifier, open_bracket, [uint | identifier], close_bracket;

function_call   = identifier, open_paren,( {(expressions, ",") } ),closing_paren;

// Can either be [1,1,1,1] or [1;4]
array           = open_bracket, ( (expression, {",",expression}) | (expression,";", uint) ), close_bracket;
    
expression      = identifier | literal | binary_operation | unary_operation | parentheses | if_then_else | index | index_mut | array | function_call;


// Statement

let_statement           = let,["mut"], expression, [(":", type)], [("=", expression)];
assign_statement        = expression, "=", expression;
while_statement         = while, expression, block;
expression_statement    = expression;
op_assign               = [let],ident,operation,"=",expression;

function_declaration    = "fn", ident, open_paren, {(["mut"], ident, ":", type )}, closing_paren, [("->",type)], block;

statement               = let_statement | assign_statement | while_statement | expression_statement | block | function_declaration|op_assign;

// block

block = open_block, {(statement,";")}, [statement] , close_block;


// Full programs
static_var          = static,["mut"],ident,":",type,"=",expr,";";
main_declaration    = "fn", "main", open_paren, {(["mut"], ident, ":", type )}, closing_paren, [("->",type)], block;

top_level = function_declaration | static_var;

program = {top_level},main_declaration,{top_level};




```
