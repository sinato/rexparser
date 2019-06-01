use crate::lexer::token::{Associativity, Property, Token, Tokens};
use log::debug;
use regex::Regex;
use std::collections::HashMap;

pub struct Lexer {
    re: Regex,
    names: Vec<&'static str>,
}

pub fn get_property(op: &String) -> Property {
    let mut map = HashMap::new();
    map.insert("=", (2, Associativity::Right));
    map.insert("+", (12, Associativity::Left));
    map.insert("-", (12, Associativity::Left));
    map.insert("*", (13, Associativity::Left));
    map.insert("[", (16, Associativity::Left));
    map.insert("(", (16, Associativity::Left));
    map.insert(",", (1, Associativity::Left));
    let op: &str = &op;
    let (precedence, associativity): (u32, Associativity) = map[op].clone();
    Property {
        precedence,
        associativity,
    }
}

impl Lexer {
    // static constructor
    pub fn new() -> Lexer {
        let token_patterns = vec![
            ("COLON", r":"),
            ("QUESTION", r"\?"),
            ("FLOAT_NUM", r"(\d+\.\d+)"),
            ("INT_NUM", r"(\d+)"),
            ("SEMI", r";"),
            ("SQUARE_E", r"\]"),
            ("PAREN_E", r"\)"),
            ("CURLY_S", r"\{"),
            ("CURLY_E", r"\}"),
            ("PREFIXOP", r"(\s|^)\+\+"),
            ("SUFFIXOP", r"(\+\+(\s|$)|\[|\()"),
            ("OP", r"(\+|-|\*|=|,)"),
            ("IDE", r"[a-z]+"),
        ];
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
                "COLON" => tokens.push(Token::Colon),
                "QUESTION" => tokens.push(Token::Question),
                "FLOAT_NUM" => tokens.push(Token::FloatNum(val)),
                "INT_NUM" => tokens.push(Token::IntNum(val)),
                "SEMI" => tokens.push(Token::Semi),
                "SQUARE_E" => tokens.push(Token::SquareE),
                "PAREN_E" => tokens.push(Token::ParenE),
                "CURLY_S" => tokens.push(Token::CurlyS),
                "CURLY_E" => tokens.push(Token::CurlyE),
                "PREFIXOP" => {
                    let val = val.trim_start().to_string();
                    tokens.push(Token::PrefixOp(val));
                }
                "SUFFIXOP" => {
                    let val = val.trim_end().to_string();
                    tokens.push(Token::SuffixOp(val));
                }
                "OP" => {
                    let val = val.trim_end().to_string();
                    tokens.push(Token::Op(val.clone(), get_property(&val)))
                }
                "IDE" => tokens.push(Token::Ide(val)),
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
