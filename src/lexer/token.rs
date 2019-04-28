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
    SuffixOp(String, Property),
    Ide(String),
    SquareE,
    ParenE,
    Comma,
}
impl Token {
    pub fn print(&self) {
        match self {
            Token::Num(num) => print!("{}", num),
            Token::Op(op, _) => print!("{}", op),
            Token::SuffixOp(op, _) => print!("{}", op),
            Token::Ide(ide) => print!("{}", ide),
            Token::SquareE => print!("]"),
            Token::ParenE => print!(")"),
            Token::Comma => print!(","),
        }
    }
    pub fn get_len(&self) -> usize {
        match self {
            Token::Num(num) => num.to_string().len(),
            Token::Op(op, _) => op.len(),
            Token::SuffixOp(op, _) => op.len(),
            Token::Ide(ide) => ide.len(),
            _ => panic!("this kind of token does not have the length."),
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
