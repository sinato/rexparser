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

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Token::FloatNum(num, _) => write!(f, "{}", num),
            Token::IntNum(num, _) => write!(f, "{}", num),
            Token::Op(op, _) => write!(f, "{}", op),
            Token::PrefixOp(op, _) => write!(f, "{}", op),
            Token::SuffixOp(op, _) => write!(f, "{}", op),
            Token::Ide(ide, _) => write!(f, "{}", ide),
            Token::Type(val, _) => write!(f, "{:?}", val),
            Token::Return(_) => write!(f, "return"),
            Token::If(_) => write!(f, "if"),
            Token::Else(_) => write!(f, "else"),
            Token::While(_) => write!(f, "while"),
            Token::Break(_) => write!(f, "break"),
            Token::Continue(_) => write!(f, "continue"),
            Token::For(_) => write!(f, "for"),
            Token::Semi(_) => write!(f, ";"),
            Token::CurlyS(_) => write!(f, "{{"),
            Token::CurlyE(_) => write!(f, "}}"),
            Token::SquareE(_) => write!(f, "]"),
            Token::ParenE(_) => write!(f, ")"),
            Token::Colon(_) => write!(f, ":"),
            Token::Question(_) => write!(f, "?"),
        }
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
}
