#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Num(i32),
    Op(String),
    Ide(String),
}
impl Token {
    pub fn print(&self) {
        match self {
            Token::Num(num) => print!("{}", num),
            Token::Op(op) => print!("{}", op),
            Token::Ide(ide) => print!("{}", ide),
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
    pub fn consume_op(&mut self) -> Result<String, String> {
        let token = match self.pop() {
            Some(token) => token,
            None => return Err("Expect an operator, but there is no token.".to_string()),
        };
        match token {
            Token::Op(operator) => return Ok(operator),
            _ => return Err("Expect an operator, but not found.".to_string()),
        }
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
            Token::Ide(_) => {
                if ty == "Ide" {
                    return Ok(token);
                }
            }
        }
        Err("Token type does not match the expected type.".to_string())
    }
}
