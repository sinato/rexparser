#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Num(i32),
    Op(String),
}
impl Token {
    pub fn print(&self) {
        match self {
            Token::Num(num) => print!("{}", num),
            Token::Op(op) => print!("{}", op),
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
    pub fn consume(&mut self, ty: &str) -> Result<Token, String> {
        let token = match self.pop() {
            Some(token) => token,
            None => return Err("There is no token.".to_string()),
        };
        match token {
            Token::Num(_) => {
                if ty == "Num" {
                    return Ok(token);
                }
            }
            Token::Op(_) => {
                if ty == "Op" {
                    return Ok(token);
                }
            }
        }
        Err("Token type does not match the expected type.".to_string())
    }
    pub fn expect(&self, ty: &str) -> Result<Token, String> {
        let mut tokens = self.clone();
        let token = match tokens.pop() {
            Some(token) => token,
            None => return Err("There is no token.".to_string()),
        };
        match token {
            Token::Num(_) => {
                if ty == "Num" {
                    return Ok(token);
                }
            }
            Token::Op(_) => {
                if ty == "Op" {
                    return Ok(token);
                }
            }
        }
        Err("Token type does not match the expected type.".to_string())
    }
}
