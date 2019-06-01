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
pub enum Token {
    Num(i32),
    Op(String, Property),
    PrefixOp(String),
    SuffixOp(String),
    Ide(String),
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
            Token::Num(num) => num.to_string().len(),
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
            Token::Num(num) => write!(f, "{}", num),
            Token::Op(op, _) => write!(f, "{}", op),
            Token::PrefixOp(op) => write!(f, "{}", op),
            Token::SuffixOp(op) => write!(f, "{}", op),
            Token::Ide(ide) => write!(f, "{}", ide),
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
