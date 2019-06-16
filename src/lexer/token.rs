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
    Void,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    FloatNum(String),
    IntNum(String),
    Op(String, Property),
    PrefixOp(String),
    SuffixOp(String),
    Ide(String),
    Type(BasicType),
    Return,
    If,
    Else,
    While,
    Break,
    Continue,
    For,
    Semi,
    CurlyS,
    CurlyE,
    SquareE,
    ParenE,
    Colon,
    Question,
}
impl Token {
    pub fn get_len(&self) -> usize {
        match self {
            Token::FloatNum(num) => num.to_string().len(),
            Token::IntNum(num) => num.to_string().len(),
            Token::Op(op, _) => op.len(),
            Token::PrefixOp(op) => op.len(),
            Token::SuffixOp(op) => op.len(),
            Token::Ide(ide) => ide.len(),
            _ => panic!("this kind of token does not have the length."),
        }
    }
}
impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Token::FloatNum(num) => write!(f, "{}", num),
            Token::IntNum(num) => write!(f, "{}", num),
            Token::Op(op, _) => write!(f, "{}", op),
            Token::PrefixOp(op) => write!(f, "{}", op),
            Token::SuffixOp(op) => write!(f, "{}", op),
            Token::Ide(ide) => write!(f, "{}", ide),
            Token::Type(val) => write!(f, "{:?}", val),
            Token::Return => write!(f, "return"),
            Token::If => write!(f, "if"),
            Token::Else => write!(f, "else"),
            Token::While => write!(f, "while"),
            Token::Break => write!(f, "break"),
            Token::Continue => write!(f, "continue"),
            Token::For => write!(f, "for"),
            Token::Semi => write!(f, ";"),
            Token::CurlyS => write!(f, "{{"),
            Token::CurlyE => write!(f, "}}"),
            Token::SquareE => write!(f, "]"),
            Token::ParenE => write!(f, ")"),
            Token::Colon => write!(f, ":"),
            Token::Question => write!(f, "?"),
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
