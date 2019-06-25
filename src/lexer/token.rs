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
