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
    map.insert("*", (13, Associativity::Left));
    map.insert("++", (16, Associativity::Left));
    map.insert("[", (16, Associativity::Left));
    map.insert("(", (16, Associativity::Left));
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
            ("COMMA", r","),
            ("COLON", r":"),
            ("QUESTION", r"\?"),
            ("NUM", r"(\d+(\.\d)*)"),
            ("SQUARE_E", r"\]"),
            ("PAREN_E", r"\)"),
            ("SUFFIXOP", r"(\+\+|\[|\()"),
            ("OP", r"(\+|\*|=)"),
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
                "COMMA" => tokens.push(Token::Comma),
                "COLON" => tokens.push(Token::Colon),
                "QUESTION" => tokens.push(Token::Question),
                "NUM" => tokens.push(Token::Num(
                    val.parse::<i32>()
                        .expect("something went wrong parsing a number"),
                )),
                "SQUARE_E" => tokens.push(Token::SquareE),
                "PAREN_E" => tokens.push(Token::ParenE),
                "SUFFIXOP" => tokens.push(Token::SuffixOp(val.clone(), get_property(&val))),
                "OP" => tokens.push(Token::Op(val.clone(), get_property(&val))),
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
