use crate::tokens::{Token, Value};

#[derive(Debug)]
pub enum Node {
    IfStatement(Box<IfStatement>),
    ForLoop(Box<ForLoop>),
    BinOp(Box<BinOp>),
    Num(Num),
    Str(Str),
    UnaryOp(Box<UnaryOp>),
    Program(Program),
    Assign(Box<Assign>),
    VarDecl(Box<VarDecl>),
    BoolExpr(Box<BoolExpr>),
    DeclAssign(Box<DeclAssign>),
    Var(Var),
    PrintVar(Box<PrintVar>),
    PrintStr(Box<PrintStr>),
    Read(Box<Read>),
    NoOp,
}

#[derive(Clone, Debug)]
pub struct Var {
    pub token: Token,
    pub value: Value,
}

impl Var {
    pub fn new(token: Token) -> Self {
        Var {
            value: token.value.clone(),
            token,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Type {
    pub token: Token,
    pub value: Value,
}

impl Type {
    pub fn new(token: Token) -> Self {
        Type {
            value: token.value.clone(),
            token,
        }
    }
}

#[derive(Clone, Debug)]
pub struct VarDecl {
    pub var_node: Var,
    pub type_node: Type,
}

impl VarDecl {
    pub fn new(var_node: Var, type_node: Type) -> Self {
        VarDecl {
            var_node,
            type_node,
        }
    }
}

#[derive(Debug)]
pub struct Read {
    pub var_node: Var,
}

impl Read {
    pub fn new(var_node: Var) -> Self {
        Read { var_node }
    }
}

#[derive(Debug)]
pub struct PrintStr {
    pub value: Value,
}

impl PrintStr {
    pub fn new(value: Value) -> Self {
        PrintStr { value }
    }
}

#[derive(Debug)]
pub struct PrintVar {
    pub var_node: Var,
}

impl PrintVar {
    pub fn new(var_node: Var) -> Self {
        PrintVar { var_node }
    }
}

#[derive(Debug)]
pub struct BoolExpr {
    pub left: Node,
    pub op: Token,
    pub right: Node,
}

impl BoolExpr {
    pub fn new(left: Node, op: Token, right: Node) -> Self {
        BoolExpr { left, op, right }
    }
}

#[derive(Debug)]
pub struct Assign {
    pub left: Var,
    pub token: Token,
    pub op: Token,
    pub right: Node,
}

impl Assign {
    pub fn new(left: Var, op: Token, right: Node) -> Self {
        Assign {
            left,
            token: op.clone(),
            op,
            right,
        }
    }
}

#[derive(Debug)]
pub struct DeclAssign {
    pub left: Var,
    pub token: Token,
    pub type_node: Type,
    pub op: Token,
    pub right: Node,
}

impl DeclAssign {
    pub fn new(left: Var, type_node: Type, op: Token, right: Node) -> Self {
        DeclAssign {
            left,
            type_node,
            token: op.clone(),
            op,
            right,
        }
    }
}

#[derive(Debug)]
pub struct Program {
    pub children: Vec<Node>,
}

impl Program {
    pub fn new() -> Self {
        Program {
            children: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct IfStatement {
    pub bool_expr: Node,
    pub statements: Vec<Node>,
    pub else_statements: Vec<Node>,
}

impl IfStatement {
    pub fn new(bool_expr: Node, statements: Vec<Node>, else_statements: Vec<Node>) -> Self {
        IfStatement {
            bool_expr,
            statements,
            else_statements,
        }
    }
}

#[derive(Debug)]
pub struct ForLoop {
    pub var_node: Var,
    pub start: Node,
    pub end: Node,
    pub statements: Vec<Node>,
}

impl ForLoop {
    pub fn new(var_node: Var, start: Node, end: Node, statements: Vec<Node>) -> Self {
        ForLoop {
            var_node,
            start,
            end,
            statements,
        }
    }
}

#[derive(Debug)]
pub struct BinOp {
    pub left: Node,
    pub token: Token,
    pub op: Token,
    pub right: Node,
}

impl BinOp {
    pub fn new(left: Node, op: Token, right: Node) -> Self {
        BinOp {
            left,
            token: op.clone(),
            op,
            right,
        }
    }
}

#[derive(Debug)]
pub struct Str {
    pub token: Token,
    pub value: Value,
}

impl Str {
    pub fn new(token: Token) -> Self {
        Str {
            value: token.value.clone(),
            token,
        }
    }
}

#[derive(Debug)]
pub struct Num {
    pub token: Token,
    pub value: Value,
}

impl Num {
    pub fn new(token: Token) -> Self {
        Num {
            value: token.value.clone(),
            token,
        }
    }
}

#[derive(Debug)]
pub struct UnaryOp {
    pub token: Token,
    pub op: Token,
    pub expr: Node,
}

impl UnaryOp {
    pub fn new(op: Token, expr: Node) -> Self {
        UnaryOp {
            token: op.clone(),
            op,
            expr,
        }
    }
}
