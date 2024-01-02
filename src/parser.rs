use crate::nodes::{
    Assign, BinOp, BoolExpr, DeclAssign, ForLoop, IfStatement, Node, Num, PrintStr, PrintVar,
    Program, Read, Str, Type, UnaryOp, Var, VarDecl,
};
use crate::scanner::Scanner;
use crate::tokens::{Token, TokenType, Value};

pub struct Parser {
    scanner: Scanner,
    current_token: Option<Token>,
}

impl Parser {
    pub fn new(mut scanner: Scanner) -> Self {
        let current_token = Some(scanner.get_next_token());
        Parser {
            scanner,
            current_token,
        }
    }

    pub fn parse(&mut self) -> Node {
        let node = self.program();
        if let TokenType::EOF = self.current_token.as_ref().unwrap().type_ {
            return node;
        } else {
            self.error();
        }
        unreachable!()
    }

    fn program(&mut self) -> Node {
        let nodes = self.statement_list();
        let mut root = Program::new();
        for node in nodes {
            root.children.push(node);
        }
        Node::Program(root)
    }

    fn statement_list(&mut self) -> Vec<Node> {
        let node = self.statement();
        let mut results = vec![node];

        while let TokenType::Semi = self.current_token.as_ref().unwrap().type_ {
            self.eat(TokenType::Semi);
            results.push(self.statement());
        }
        if let TokenType::ID = self.current_token.as_ref().unwrap().type_ {
            self.error();
        }
        results
    }

    fn statement(&mut self) -> Node {
        match self.current_token.as_ref().unwrap().type_ {
            TokenType::ID => self.assignment_statement(),
            TokenType::Var => self.declaration_statement(),
            TokenType::Print => self.print_statement(),
            TokenType::Read => self.read_statement(),
            TokenType::For => self.for_loop(),
            TokenType::If => self.if_statement(),
            _ => self.empty(),
        }
    }

    fn empty(&self) -> Node {
        Node::NoOp
    }

    fn variable(&mut self) -> Var {
        let node = Var::new(self.current_token.clone().unwrap());
        self.eat(TokenType::ID);
        node
    }

    fn print_statement(&mut self) -> Node {
        let mut node = Node::NoOp;
        self.eat(TokenType::Print);
        match self.current_token.as_ref().unwrap().type_ {
            TokenType::ID => {
                let var_node = self.variable();
                node = Node::PrintVar(Box::new(PrintVar::new(var_node)))
            }
            TokenType::StringLiteral => {
                let string_token = self.current_token.clone().unwrap();
                self.eat(TokenType::StringLiteral);
                node = Node::PrintStr(Box::new(PrintStr::new(Value::String(
                    string_token.value.to_string(),
                ))));
            }
            _ => self.error(),
        }
        node
    }

    fn read_statement(&mut self) -> Node {
        let mut node = Node::NoOp;
        self.eat(TokenType::Read);
        match self.current_token.as_ref().unwrap().type_ {
            TokenType::ID => {
                let var_node = self.variable();
                node = Node::Read(Box::new(Read::new(var_node)))
            }
            _ => self.error(),
        }
        node
    }

    fn assignment_statement(&mut self) -> Node {
        let left = self.variable();
        let token = self.current_token.clone().unwrap();
        self.eat(TokenType::Assign);
        let right = self.expr();
        Node::Assign(Box::new(Assign::new(left, token, right)))
    }

    fn declaration_statement(&mut self) -> Node {
        let mut node = Node::NoOp;
        if let TokenType::Var = self.current_token.as_ref().unwrap().type_ {
            self.eat(TokenType::Var);
            let var_node = self.variable();
            self.eat(TokenType::Colon);
            match self.current_token.as_ref().unwrap().type_ {
                TokenType::Integer => {
                    let type_node = Type::new(self.current_token.clone().unwrap());
                    self.eat(TokenType::Integer);
                    match self.current_token.as_ref().unwrap().type_ {
                        TokenType::Semi => {
                            node = Node::VarDecl(Box::new(VarDecl::new(var_node, type_node)));
                        }
                        TokenType::Assign => {
                            let token = self.current_token.clone().unwrap();
                            self.eat(TokenType::Assign);
                            let right = self.expr();
                            node = Node::DeclAssign(Box::new(DeclAssign::new(
                                var_node, type_node, token, right,
                            )));
                        }
                        _ => self.error(),
                    }
                }
                TokenType::Str => {
                    let type_node = Type::new(self.current_token.clone().unwrap());
                    self.eat(TokenType::Str);
                    match self.current_token.as_ref().unwrap().type_ {
                        TokenType::Semi => {
                            //no assign
                            node = Node::VarDecl(Box::new(VarDecl::new(var_node, type_node)));
                        }
                        TokenType::Assign => {
                            //declaration assignment
                            let token = self.current_token.clone().unwrap();
                            self.eat(TokenType::Assign);
                            let string_token = self.current_token.clone().unwrap();
                            let right = match self.current_token.as_ref().unwrap().type_ {
                                TokenType::StringLiteral => {
                                    self.eat(TokenType::StringLiteral);
                                    Node::Str(Str::new(string_token))
                                }
                                _ => self.expr(),
                            };
                            node = Node::DeclAssign(Box::new(DeclAssign::new(
                                var_node, type_node, token, right,
                            )));
                        }
                        _ => self.error(),
                    }
                }
                TokenType::Bool => {
                    let type_node = Type::new(self.current_token.clone().unwrap());
                    self.eat(TokenType::Bool);
                    match self.current_token.as_ref().unwrap().type_ {
                        TokenType::Semi => {
                            //no assign
                            node = Node::VarDecl(Box::new(VarDecl::new(var_node, type_node)));
                        }
                        TokenType::Assign => {
                            //declaration assignment
                            let token = self.current_token.clone().unwrap();
                            self.eat(TokenType::Assign);
                            let right = self.bool_expr();
                            node = Node::DeclAssign(Box::new(DeclAssign::new(
                                var_node, type_node, token, right,
                            )));
                        }
                        _ => self.error(),
                    }
                }
                _ => self.error(),
            }
        }
        node
    }

    fn if_statement(&mut self) -> Node {
        self.eat(TokenType::If);
        let bool_expr = self.bool_expr();
        self.eat(TokenType::Do);
        let statements = self.statement_list();
        match self.current_token.clone().unwrap().type_ {
            TokenType::Else => {
                self.eat(TokenType::Else);
            }
            _ => {}
        };
        let else_statements = self.statement_list();
        let node = Node::IfStatement(Box::new(IfStatement::new(
            bool_expr,
            statements,
            else_statements,
        )));
        self.eat(TokenType::End);
        self.eat(TokenType::If);
        node
    }

    fn for_loop(&mut self) -> Node {
        let mut node = Node::NoOp;
        self.eat(TokenType::For);
        let var = self.variable();
        self.eat(TokenType::In);
        let start = self.expr();
        self.eat(TokenType::To);
        let end = self.expr();
        self.eat(TokenType::Do);
        let statements = self.statement_list();
        if !statements.is_empty() {
            node = Node::ForLoop(Box::new(ForLoop::new(var, start, end, statements)));
        }
        self.eat(TokenType::End);
        self.eat(TokenType::For);
        node
    }

    fn factor(&mut self) -> Node {
        let token = self.current_token.clone().unwrap();
        match &token.type_ {
            TokenType::Plus => {
                self.eat(TokenType::Plus);
                Node::UnaryOp(Box::new(UnaryOp::new(token, self.factor())))
            }
            TokenType::Minus => {
                self.eat(TokenType::Minus);
                Node::UnaryOp(Box::new(UnaryOp::new(token, self.factor())))
            }
            TokenType::Integer => {
                self.eat(TokenType::Integer);
                Node::Num(Num::new(token))
            }
            TokenType::LeftParen => {
                self.eat(TokenType::LeftParen);
                let node = self.expr();
                self.eat(TokenType::RightParen);
                node
            }
            _ => Node::Var(self.variable()),
        }
    }

    fn term(&mut self) -> Node {
        let mut node = self.factor();

        while let TokenType::Mul | TokenType::Div = self.current_token.as_ref().unwrap().type_ {
            let token = self.current_token.clone().unwrap();
            match token.type_ {
                TokenType::Mul => self.eat(TokenType::Mul),
                TokenType::Div => self.eat(TokenType::Div),
                _ => unimplemented!(),
            }
            node = Node::BinOp(Box::new(BinOp::new(node, token, self.factor())));
        }
        node
    }

    fn bool_expr(&mut self) -> Node {
        let mut token = self.current_token.clone().unwrap();
        match &token.type_ {
            TokenType::Not => {
                self.eat(TokenType::Not);
                let right = self.expr();
                return Node::BoolExpr(Box::new(BoolExpr::new(Node::NoOp, token, right)));
            }
            _ => {}
        }
        let left = self.expr();
        token = self.current_token.clone().unwrap();
        match &token.type_ {
            TokenType::LessThan => self.eat(TokenType::LessThan),
            TokenType::Equal => self.eat(TokenType::Equal),
            TokenType::And => self.eat(TokenType::And),
            _ => {
                return Node::BoolExpr(Box::new(BoolExpr::new(
                    left,
                    Token::new(TokenType::Semi, Value::None),
                    Node::NoOp,
                )))
            }
        }
        let right = self.expr();
        Node::BoolExpr(Box::new(BoolExpr::new(left, token, right)))
    }

    fn expr(&mut self) -> Node {
        let mut node = self.term();

        while let TokenType::Plus | TokenType::Minus = self.current_token.as_ref().unwrap().type_ {
            let token = self.current_token.clone().unwrap();
            match token.type_ {
                TokenType::Plus => self.eat(TokenType::Plus),
                TokenType::Minus => self.eat(TokenType::Minus),
                _ => unimplemented!(),
            }
            node = Node::BinOp(Box::new(BinOp::new(node, token, self.term())));
        }
        node
    }

    fn error(&self) {
        panic!("Syntax error");
    }

    fn eat(&mut self, token_type: TokenType) {
        if self.current_token.as_ref().unwrap().type_ == token_type {
            self.current_token = Some(self.scanner.get_next_token());
        } else {
            self.error();
        }
    }
}
