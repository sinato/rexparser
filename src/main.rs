use std::{env, process};

mod emitter;
mod lexer;
mod parser;

use emitter::Emitter;
use lexer::lexer::Lexer;
use parser::parser;

fn compiler(code: String) {
    let lexer = Lexer::new();
    let mut tokens = lexer.lex(code);
    // dbg!(tokens.clone());
    let node = parser(&mut tokens);
    // dbg!(node.clone());
    let mut emitter = Emitter::new();
    emitter.emit(node);
    emitter.print_to_file();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage rexparser \"<code>\"");
        process::exit(1);
    }
    let code = args[1].to_string();
    compiler(code);
}
