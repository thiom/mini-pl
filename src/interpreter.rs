use crate::nodes::{
    Assign, BinOp, BoolExpr, DeclAssign, ForLoop, IfStatement, Node, Num, PrintStr, PrintVar,
    Program, Read, Str, Type, UnaryOp, Var, VarDecl,
};
use crate::parser::Parser;
use crate::tokens::{TokenType, Value};
use std::collections::HashMap;
use std::io::stdin;

trait NodeVisitor {
    fn visit_read(&mut self, read: &Read);
    fn visit_print_var(&mut self, print_var: &PrintVar);
    fn visit_print_str(&mut self, print_str: &PrintStr);
    fn visit_num(&self, num: &Num) -> i32;
    fn visit_str(&self, str_node: &Str) -> String;
    fn visit_bin_op(&mut self, bin_op: &BinOp) -> Value;
    fn visit_bool_expr(&mut self, bool_expr: &BoolExpr) -> bool;
    fn visit_unary_op(&mut self, unary_op: &UnaryOp) -> i32;
    fn visit_assign(&mut self, assign: &Assign);
    fn visit_var(&self, var: &Var) -> Value;
    fn visit_program(&mut self, program: &Program);
    fn visit_var_decl(&mut self, var_decl: &VarDecl);
    fn visit_decl_assign(&mut self, decl_assign: &DeclAssign);
    fn visit_type(&self, type_: &Type);
    fn visit_for_loop(&mut self, for_loop: &ForLoop);
    fn visit_if_statement(&mut self, if_statement: &IfStatement);
}

pub struct Interpreter {
    parser: Parser,
    pub global_scope: HashMap<String, Value>,
}

impl Interpreter {
    pub fn new(parser: Parser) -> Self {
        Interpreter {
            parser,
            global_scope: HashMap::new(),
        }
    }

    pub fn interpret(&mut self) -> Value {
        let tree = self.parser.parse();
        self.visit(&tree)
    }

    pub fn visit(&mut self, node: &Node) -> Value {
        match node {
            Node::BinOp(n) => self.visit_bin_op(n),
            Node::UnaryOp(n) => Value::Number(self.visit_unary_op(n)),
            Node::Num(n) => Value::Number(self.visit_num(n)),
            Node::Str(n) => Value::String(self.visit_str(n)),
            Node::NoOp => Value::None,
            Node::BoolExpr(n) => Value::Boolean(self.visit_bool_expr(n)),
            Node::ForLoop(n) => {
                self.visit_for_loop(n);
                Value::None
            }
            Node::IfStatement(n) => {
                self.visit_if_statement(n);
                Value::None
            }
            Node::Assign(n) => {
                self.visit_assign(n);
                Value::None
            }
            Node::Var(n) => self.visit_var(n),
            Node::Program(n) => {
                self.visit_program(n);
                Value::None
            }
            Node::VarDecl(n) => {
                self.visit_var_decl(n);
                Value::None
            }
            Node::DeclAssign(n) => {
                self.visit_decl_assign(n);
                Value::None
            }
            Node::PrintStr(n) => {
                self.visit_print_str(n);
                Value::None
            }
            Node::PrintVar(n) => {
                self.visit_print_var(n);
                Value::None
            }
            Node::Read(n) => {
                self.visit_read(n);
                Value::None
            }
        }
    }
}

impl NodeVisitor for Interpreter {
    fn visit_for_loop(&mut self, for_loop: &ForLoop) {
        match self.visit_var(&for_loop.var_node) {
            Value::Number(_) => {}
            Value::String(_) => panic!("loop variable must be declared as integer"),
            Value::Boolean(_) => panic!("loop variable must be declared as integer"),
            _ => panic!("variable used before declaration"),
        };
        let var_name = match &for_loop.var_node.value {
            Value::String(s) => s.to_string(),
            _ => panic!("Error"),
        };
        let start = match self.visit(&for_loop.start) {
            Value::Number(n) => n,
            _ => panic!("Error"),
        };
        let end = match self.visit(&for_loop.end) {
            Value::Number(n) => n,
            _ => panic!("Error"),
        };
        for i in start..end {
            self.global_scope
                .insert(var_name.to_lowercase(), Value::Number(i));

            for statement in &for_loop.statements {
                self.visit(statement);
            }
        }
    }

    fn visit_bool_expr(&mut self, bool_expr: &BoolExpr) -> bool {
        match &bool_expr.op.type_ {
            TokenType::And => {
                let left_bool = match self.visit(&bool_expr.left) {
                    Value::Boolean(b) => b,
                    _ => panic!("Error"),
                };
                let right_bool = match self.visit(&bool_expr.right) {
                    Value::Boolean(b) => b,
                    _ => panic!("Error"),
                };
                return left_bool && right_bool;
            }
            TokenType::Semi => {
                match self.visit(&bool_expr.left) {
                    Value::Boolean(b) => return b,
                    _ => panic!("Error"),
                };
            }
            TokenType::Not => {
                match self.visit(&bool_expr.right) {
                    Value::Boolean(b) => return !b,
                    _ => panic!("Type error"),
                };
            }
            _ => {}
        }
        let left = match self.visit(&bool_expr.left) {
            Value::Number(n) => n,
            _ => panic!("Type error"),
        };
        let right = match self.visit(&bool_expr.right) {
            Value::Number(n) => n,
            _ => panic!("Type error"),
        };
        match &bool_expr.op.type_ {
            TokenType::Equal => left == right,
            TokenType::LessThan => left < right,
            _ => unimplemented!(),
        }
    }

    fn visit_if_statement(&mut self, if_statement: &IfStatement) {
        let boolean = match self.visit(&if_statement.bool_expr) {
            Value::Boolean(b) => b,
            _ => panic!("Error: If statement condition must be a boolean value"),
        };
        if boolean {
            for statement in &if_statement.statements {
                self.visit(statement);
            }
        } else {
            for statement in &if_statement.else_statements {
                self.visit(statement);
            }
        }
    }

    fn visit_print_var(&mut self, print_var: &PrintVar) {
        let var_value = match self.visit_var(&print_var.var_node) {
            Value::Number(v) => v.to_string(),
            Value::String(v) => v,
            Value::Boolean(v) => v.to_string(),
            _ => panic!("variable used before declaration"),
        };
        println!("{}", var_value);
    }

    fn visit_print_str(&mut self, print_str: &PrintStr) {
        let string_literal = match &print_str.value {
            Value::String(s) => s.to_string(),
            _ => panic!("Error"),
        };
        println!("{}", string_literal);
    }

    fn visit_read(&mut self, read: &Read) {
        let var_name = match &read.var_node.value {
            Value::String(s) => s.to_string(),
            _ => panic!("Error"),
        };
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        if let Some('\n') = input.chars().next_back() {
            input.pop();
        };

        let var_value = self
            .global_scope
            .get(&var_name.to_lowercase())
            .unwrap()
            .clone();
        match var_value {
            Value::String(_) => {
                self.global_scope
                    .insert(var_name.to_lowercase(), Value::String(input));
            }
            Value::Number(_) => {
                if input.parse::<i32>().is_ok() {
                    self.global_scope
                        .insert(var_name.to_lowercase(), Value::String(input));
                } else {
                    panic!("Error: cannot read non-numeric value into numeric variable");
                }
            }
            _ => panic!("variable {} used before declaration", var_name),
        };
    }

    fn visit_num(&self, num: &Num) -> i32 {
        match num.value {
            Value::Number(n) => n,
            _ => unimplemented!(),
        }
    }

    fn visit_str(&self, str_node: &Str) -> String {
        match &str_node.value {
            Value::String(n) => n.clone(),
            _ => unimplemented!(),
        }
    }

    fn visit_bin_op(&mut self, bin_op: &BinOp) -> Value {
        let left = self.visit(&bin_op.left);
        let right = self.visit(&bin_op.right);
        match (left, right) {
            (Value::Number(n), Value::Number(m)) => match bin_op.op.type_ {
                TokenType::Plus => Value::Number(n + m),
                TokenType::Minus => Value::Number(n - m),
                TokenType::Mul => Value::Number(n * m),
                TokenType::Div => Value::Number(n / m),
                _ => unimplemented!(),
            },
            (Value::String(s), Value::String(t)) => match bin_op.op.type_ {
                TokenType::Plus => {
                    let mut result = s.clone();
                    result.push_str(&t);
                    Value::String(result)
                }
                _ => unimplemented!(),
            },
            _ => panic!("Type mismatch"),
        }
    }

    fn visit_program(&mut self, program: &Program) {
        for child in &program.children {
            self.visit(child);
        }
    }

    fn visit_unary_op(&mut self, unary_op: &UnaryOp) -> i32 {
        let expr = match self.visit(&unary_op.expr) {
            Value::Number(n) => n,
            _ => panic!("Error"),
        };
        match unary_op.op.type_ {
            TokenType::Plus => (0) + expr,
            TokenType::Minus => (0) - expr,
            _ => unimplemented!(),
        }
    }

    fn visit_assign(&mut self, assign: &Assign) {
        let left = self.visit_var(&assign.left);
        let right = self.visit(&assign.right);
        match (left, &right) {
            (Value::Number(_), Value::Number(_)) => {}
            (Value::String(_), Value::String(_)) => {}
            (Value::Boolean(_), Value::Boolean(_)) => {}
            _ => panic!("Type mismatch"),
        };
        let var_name = match &assign.left.value {
            Value::String(s) => s.to_string(),
            _ => {
                panic!("Error");
            }
        };
        self.global_scope
            .insert(var_name.to_lowercase(), right.clone());
    }

    fn visit_var(&self, var: &Var) -> Value {
        let var_name = match &var.value {
            Value::String(s) => s.to_string(),
            _ => panic!("Error"),
        };
        self.global_scope
            .get(&var_name.to_lowercase())
            .unwrap()
            .clone()
    }

    fn visit_decl_assign(&mut self, decl_assign: &DeclAssign) {
        let var_name = match &decl_assign.left.value {
            Value::String(s) => s.to_string(),
            _ => {
                panic!("Error");
            }
        };
        let value = self.visit(&decl_assign.right);
        self.global_scope.insert(var_name.to_lowercase(), value);
    }

    fn visit_var_decl(&mut self, var_decl: &VarDecl) {
        let var_name = match &var_decl.var_node.value {
            Value::String(s) => s.to_string(),
            _ => {
                panic!("Error");
            }
        };
        match &var_decl.type_node.token.type_ {
            TokenType::Str => {
                self.global_scope
                    .insert(var_name.to_lowercase(), Value::String("".to_string()));
            }
            TokenType::Integer => {
                self.global_scope
                    .insert(var_name.to_lowercase(), Value::Number(0));
            }
            TokenType::Bool => {
                self.global_scope
                    .insert(var_name.to_lowercase(), Value::Boolean(true));
            }
            _ => unimplemented!(),
        }
    }

    fn visit_type(&self, _: &Type) {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::Interpreter;
    use crate::scanner::Scanner;
    use crate::tokens::Value;
    use std::collections::HashMap;

    #[test]
    fn variables_and_arithmetic() {
        let text = "
        var a : int := 2;
        var b : int := 10 * a + 10;
        var c : int := a - - b;";

        let scanner = Scanner::new(text.to_string());
        let parser = Parser::new(scanner);
        let mut interpreter = Interpreter::new(parser);
        interpreter.interpret();

        let mut expected: HashMap<String, Value> = HashMap::new();
        expected.insert(String::from("a"), Value::Number(2));
        expected.insert(String::from("b"), Value::Number(30));
        expected.insert(String::from("c"), Value::Number(32));

        assert_eq!(interpreter.global_scope, expected);
    }
}
