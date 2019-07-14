use crate::lexer::token::{BasicType, DebugInfo, Token, Tokens};
use log::debug;
use regex::Regex;

pub struct Lexer {
    re: Regex,
    names: Vec<&'static str>,
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
            ("VA", r"\.\.\."),
            ("STR", r#"".+""#),
            ("TYPE", r"(int)|(float)|(char)"),
            ("STRUCT", r"(struct)"),
            ("ENUM", r"(enum)"),
            ("SWITCH", r"switch"),
            ("CONST", r"const"),
            ("EXTERN", r"extern"),
            ("CASE", r"case"),
            ("DEFAULT", r"default"),
            ("RETURN", r"return"),
            ("IF", r"if"),
            ("ELSE", r"else"),
            ("WHILE", r"while"),
            ("BREAK", r"break"),
            ("CONTINUE", r"continue"),
            ("FOR", r"for"),
            ("SQUARE_E", r"\]"),
            ("PAREN_E", r"\)"),
            ("CURLY_S", r"\{"),
            ("CURLY_E", r"\}"),
            ("ANDOP", r"(&&)"),
            ("PREFIXOP", r"((\s|^)\+\+)|&"),
            ("SUFFIXOP", r"(\+\+|\[|\()|\."),
            ("OP", r"((\|\|)|(\+=)|(==)|>|<|\+|-|\*|=|,)"),
            ("CHAR", r"'[A-Za-z_0-9.]'"),
            ("IDE", r"[A-Za-z_][A-Za-z_0-9]*"),
        ];
        let re = make_regex(&token_patterns);
        let names = get_names(&token_patterns);
        let re = Regex::new(&re).expect("something went wrong making the regex");
        Lexer { re, names }
    }
    pub fn lex(&self, code: String) -> Tokens {
        let mut code = code;
        let tokens = self.tokenize(&mut code);
        tokens
    }
    fn tokenize(&self, code: &mut String) -> Tokens {
        let mut tokens: Vec<Token> = Vec::new();

        // get token's location
        let mut locations: Vec<DebugInfo> = Vec::new();
        for mat in self.re.find_iter(&code) {
            let location = DebugInfo {
                start: mat.start(),
                end: mat.end(),
                s: mat.as_str().to_string(),
            };
            locations.push(location);
        }
        for (i, caps) in self.re.captures_iter(&code).enumerate() {
            let mut typ = String::from("nil");
            let val = String::from(&caps[0]);
            for name in &self.names {
                if caps.name(name).is_some() {
                    typ = name.to_string();
                }
            }
            let debug_info = locations.clone().remove(i);
            match typ.as_ref() {
                "COLON" => tokens.push(Token::Colon(debug_info)),
                "QUESTION" => tokens.push(Token::Question(debug_info)),
                "FLOAT_NUM" => tokens.push(Token::FloatNum(val, debug_info)),
                "INT_NUM" => tokens.push(Token::IntNum(val, debug_info)),
                "SEMI" => tokens.push(Token::Semi(debug_info)),
                "VA" => tokens.push(Token::Va(debug_info)),
                "TYPE" => match val.as_ref() {
                    "int" => tokens.push(Token::Type(BasicType::Int, debug_info)),
                    "float" => tokens.push(Token::Type(BasicType::Float, debug_info)),
                    "char" => tokens.push(Token::Type(BasicType::Int, debug_info)),
                    _ => panic!("Unimplemented type."),
                },
                "STRUCT" => tokens.push(Token::Struct(debug_info)),
                "STR" => {
                    let val = val.trim_matches('\"').to_string();
                    tokens.push(Token::Str(val, debug_info));
                }
                "ENUM" => tokens.push(Token::Enum(debug_info)),
                "SWITCH" => tokens.push(Token::Switch(debug_info)),
                "CONST" => (),
                "EXTERN" => tokens.push(Token::Extern(debug_info)),
                "CASE" => tokens.push(Token::Case(debug_info)),
                "DEFAULT" => tokens.push(Token::Default(debug_info)),
                "RETURN" => tokens.push(Token::Return(debug_info)),
                "IF" => tokens.push(Token::If(debug_info)),
                "ELSE" => tokens.push(Token::Else(debug_info)),
                "WHILE" => tokens.push(Token::While(debug_info)),
                "BREAK" => tokens.push(Token::Break(debug_info)),
                "CONTINUE" => tokens.push(Token::Continue(debug_info)),
                "FOR" => tokens.push(Token::For(debug_info)),
                "SQUARE_E" => tokens.push(Token::SquareE(debug_info)),
                "PAREN_E" => tokens.push(Token::ParenE(debug_info)),
                "CURLY_S" => tokens.push(Token::CurlyS(debug_info)),
                "CURLY_E" => tokens.push(Token::CurlyE(debug_info)),
                "ANDOP" => {
                    let val = val.trim_end().to_string();
                    tokens.push(Token::Op(val.clone(), debug_info))
                }
                "PREFIXOP" => {
                    let val = val.trim_start().to_string();
                    tokens.push(Token::PrefixOp(val, debug_info));
                }
                "SUFFIXOP" => {
                    let val = val.trim_end().to_string();
                    tokens.push(Token::SuffixOp(val, debug_info));
                }
                "OP" => {
                    let val = val.trim_end().to_string();
                    tokens.push(Token::Op(val.clone(), debug_info))
                }
                "IDE" => tokens.push(Token::Ide(val, debug_info)),
                "CHAR" => {
                    let chars: Vec<&str> = val.split("'").collect();
                    let num: i32 = chars[1].chars().into_iter().nth(0).unwrap() as i32;
                    tokens.push(Token::IntNum(num.to_string(), debug_info))
                }
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
