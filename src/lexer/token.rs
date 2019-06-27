use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, PartialEq, Clone)]
pub enum Associativity {
    Right,
    Left,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Property {
    pub precedence: u32,
    pub associativity: Associativity,
}

#[derive(Debug, PartialEq, Clone)]
pub enum BasicType {
    Int,
    Float,
    Pointer(Box<BasicType>),
    Array(Box<BasicType>, u32),
    Struct(String), // struct identifier
}

#[derive(Debug, PartialEq, Clone)]
pub struct DebugInfo {
    pub start: usize,
    pub end: usize,
    pub s: String,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    FloatNum(String, DebugInfo),
    IntNum(String, DebugInfo),
    Op(String, DebugInfo),
    PrefixOp(String, DebugInfo),
    SuffixOp(String, DebugInfo),
    Ide(String, DebugInfo),
    Type(BasicType, DebugInfo),
    Struct(DebugInfo),
    Enum(DebugInfo),
    Switch(DebugInfo),
    Case(DebugInfo),
    Default(DebugInfo),
    Return(DebugInfo),
    If(DebugInfo),
    Else(DebugInfo),
    While(DebugInfo),
    Break(DebugInfo),
    Continue(DebugInfo),
    For(DebugInfo),
    Semi(DebugInfo),
    CurlyS(DebugInfo),
    CurlyE(DebugInfo),
    SquareE(DebugInfo),
    ParenE(DebugInfo),
    Colon(DebugInfo),
    Question(DebugInfo),
}
impl Token {
    pub fn get_debug_info(self) -> String {
        let debug_info = match self.clone() {
            Token::Struct(d)
            | Token::Enum(d)
            | Token::Switch(d)
            | Token::Case(d)
            | Token::Default(d)
            | Token::Return(d)
            | Token::If(d)
            | Token::Else(d)
            | Token::While(d)
            | Token::Break(d)
            | Token::Continue(d)
            | Token::For(d)
            | Token::Semi(d)
            | Token::CurlyS(d)
            | Token::CurlyE(d)
            | Token::SquareE(d)
            | Token::ParenE(d)
            | Token::Colon(d)
            | Token::Question(d) => d,
            Token::FloatNum(_, d)
            | Token::IntNum(_, d)
            | Token::Op(_, d)
            | Token::PrefixOp(_, d)
            | Token::SuffixOp(_, d)
            | Token::Ide(_, d)
            | Token::Type(_, d) => d,
        };

        let mut file = File::open("target.c").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let contents_to_err = String::from(&contents[0..debug_info.end]);
        let num: usize = contents_to_err.split("\n").count();
        let splited_contents: Vec<&str> = contents.split("\n").collect();
        format!("target.c:{}:{}", num + 1, splited_contents[num])
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Tokens {
    pub tokens: Vec<Token>,
}
impl Tokens {
    pub fn pop(&mut self) -> Option<Token> {
        self.tokens.reverse();
        let token = self.tokens.pop();
        self.tokens.reverse();
        token
    }
    pub fn peek(&self) -> Option<Token> {
        let mut tokens = self.clone();
        tokens.pop()
    }
    pub fn push(&mut self, token: Token) {
        self.tokens.push(token);
    }
    pub fn reverse(&mut self) {
        self.tokens.reverse()
    }
}
