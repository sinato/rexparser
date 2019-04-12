use log::debug;
use regex::Regex;

pub struct Lexer {
    re: Regex,
    names: Vec<&'static str>,
}
impl Lexer {
    // static constructor
    pub fn new() -> Lexer {
        let token_patterns = vec![("NUM", r"\d+(\.\d)*"), ("OP", r"\+")];
        let re = make_regex(&token_patterns);
        let names = get_names(&token_patterns);
        let re = Regex::new(&re).expect("something went wrong making the regex");
        Lexer { re, names }
    }
    pub fn lex(&self, code: String) -> Tokens {
        let tokens = self.tokenize(code);
        tokens
    }
    fn tokenize(&self, code: String) -> Tokens {
        let mut tokens: Vec<Token> = Vec::new();
        for caps in self.re.captures_iter(&code) {
            let mut typ = String::from("nil");
            let val = String::from(&caps[0]);
            for name in &self.names {
                if caps.name(name).is_some() {
                    typ = name.to_string();
                }
            }
            match typ.as_ref() {
                "NUM" => tokens.push(Token::Num(
                    val.parse::<i32>()
                        .expect("something went wrong parsing a number"),
                )),
                "OP" => tokens.push(Token::Op(val)),
                _ => panic!("This is not an expected panic"),
            }
        }
        debug!("tokens:  {:?}", tokens);
        Tokens { tokens }
    }
}
fn make_regex(token_patterns: &Vec<(&str, &str)>) -> String {
    token_patterns
        .into_iter()
        .map(|pattern| format!("(?P<{}>{})", pattern.0, pattern.1))
        .collect::<Vec<String>>()
        .join("|")
}

fn get_names<'a, 'b>(token_patterns: &Vec<(&'a str, &'b str)>) -> Vec<&'a str> {
    token_patterns
        .into_iter()
        .map(|pattern| pattern.0)
        .collect()
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Num(i32),
    Op(String),
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
}
