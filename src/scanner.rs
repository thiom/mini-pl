use crate::tokens::{Token, TokenType, Value};
use phf::phf_map;

const RESERVED_KEYWORDS: phf::Map<&'static str, TokenType> = phf_map! {
    "bool" => TokenType::Bool,
    "var" => TokenType::Var,
    "int" => TokenType::Integer,
    "string" => TokenType::Str,
    "print" => TokenType::Print,
    "read" => TokenType::Read,
    "if" => TokenType::If,
    "else" => TokenType::Else,
    "do" => TokenType::Do,
    "for" => TokenType::For,
    "end" => TokenType::End,
    "in" => TokenType::In,
};

pub struct Scanner {
    text: String,
    pos: usize,
    current_char: Option<char>,
}

impl Scanner {
    pub fn new(text: String) -> Self {
        Scanner {
            text: text.clone(),
            pos: 0,
            current_char: Some(text.as_bytes()[0] as char),
        }
    }

    pub fn get_next_token(&mut self) -> Token {
        while let Some(c) = self.current_char {
            if c.is_whitespace() {
                self.skip_whitespace();
                continue;
            }
            if c.is_numeric() {
                return Token::new(TokenType::Integer, Value::Number(self.integer()));
            }
            match c {
                '!' => {
                    self.advance();
                    return Token::new(TokenType::Not, Value::Char(c));
                }
                '&' => {
                    self.advance();
                    return Token::new(TokenType::And, Value::Char(c));
                }
                '=' => {
                    self.advance();
                    return Token::new(TokenType::Equal, Value::Char(c));
                }
                '<' => {
                    self.advance();
                    return Token::new(TokenType::LessThan, Value::Char(c));
                }
                '+' => {
                    self.advance();
                    return Token::new(TokenType::Plus, Value::Char(c));
                }
                '-' => {
                    self.advance();
                    return Token::new(TokenType::Minus, Value::Char(c));
                }
                '*' => {
                    self.advance();
                    return Token::new(TokenType::Mul, Value::Char(c));
                }
                '/' => {
                    match self.peek() {
                        Some('/') => self.skip_comment(),
                        Some('*') => self.skip_comment(),
                        _ => {
                            self.advance();
                            return Token::new(TokenType::Div, Value::Char(c));
                        }
                    }
                    continue;
                }
                '(' => {
                    self.advance();
                    return Token::new(TokenType::LeftParen, Value::Char(c));
                }
                ')' => {
                    self.advance();
                    return Token::new(TokenType::RightParen, Value::Char(c));
                }
                ':' => {
                    if let Some('=') = self.peek() {
                        self.advance();
                        self.advance();
                        return Token::new(TokenType::Assign, Value::String(String::from(":=")));
                    } else {
                        self.advance();
                        return Token::new(TokenType::Colon, Value::Char(c));
                    }
                }
                ';' => {
                    self.advance();
                    return Token::new(TokenType::Semi, Value::Char(c));
                }
                '.' => {
                    if let Some('.') = self.peek() {
                        self.advance();
                        self.advance();
                        return Token::new(TokenType::To, Value::String(String::from("..")));
                    } else {
                        self.error()
                    }
                }
                '\"' => {
                    let token = self.string_literal();
                    if token.type_ == TokenType::StringLiteral {
                        return token;
                    } else {
                        self.error()
                    }
                }
                c => {
                    if c.is_alphanumeric() || c == '_' {
                        return self.id();
                    } else {
                        self.error()
                    }
                }
            }
        }
        Token::new(TokenType::EOF, Value::None)
    }

    fn error(&self) {
        panic!("Lexical error, invalid token");
    }

    fn advance(&mut self) {
        self.pos += 1;
        if self.pos > self.text.len() - 1 {
            self.current_char = None;
        } else {
            self.current_char = Some(self.text.as_bytes()[self.pos] as char);
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.current_char {
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn skip_comment(&mut self) {
        self.advance();
        if let Some(c) = self.current_char {
            match c {
                '/' => {
                    self.advance();
                    while let Some(ch) = self.current_char {
                        match ch {
                            '\n' => return,
                            _ => self.advance(),
                        }
                    }
                }
                '*' => {
                    self.advance();
                    while let Some(ch) = self.current_char {
                        match ch {
                            '*' => match self.peek() {
                                Some('/') => {
                                    self.advance();
                                    self.advance();
                                    return;
                                }
                                _ => self.advance(),
                            },
                            _ => self.advance(),
                        }
                    }
                }
                _ => {}
            }
        }
    }

    fn peek(&self) -> Option<char> {
        if self.pos > self.text.len() {
            None
        } else {
            Some(self.text.as_bytes()[self.pos + 1] as char)
        }
    }

    fn integer(&mut self) -> i32 {
        let mut result = String::new();
        while let Some(n) = self.current_char {
            if n.is_numeric() {
                result.push(n);
                self.advance();
            } else {
                break;
            }
        }
        result.parse().unwrap()
    }

    fn string_literal(&mut self) -> Token {
        self.advance();
        let mut result = String::new();
        while let Some(c) = self.current_char {
            match c {
                '\n' => {
                    self.error();
                    break;
                }
                ';' => {
                    self.error();
                    break;
                }
                '\\' => {
                    self.advance();
                    if let Some(ch) = self.peek() {
                        result.push(ch);
                    }
                    self.advance();
                }
                '\"' => {
                    self.advance();
                    return Token::new(TokenType::StringLiteral, Value::String(result.clone()));
                }
                _ => {
                    result.push(c);
                    self.advance();
                }
            }
        }
        Token::new(TokenType::EOF, Value::None)
    }

    fn id(&mut self) -> Token {
        let mut result = String::new();
        while let Some(c) = self
            .current_char
            .filter(|c| c.is_alphanumeric() || c == &'_')
        {
            result.push(c);
            self.advance();
        }
        RESERVED_KEYWORDS.get(&result[..]).map_or(
            Token::new(TokenType::ID, Value::String(result.clone())),
            |t| Token::new(t.clone(), Value::String(result)),
        )
    }
}
