use std::fs;
use std::{env, process};

mod emitter;
mod lexer;
mod parser;

use emitter::builtin::emit_builtin;
use emitter::Emitter;
use lexer::lexer::Lexer;
use parser::parser;
use std::io::prelude::*;

fn compiler(code: String) {
    let mut file = fs::File::create("target.c").unwrap();
    file.write_all(code.as_bytes()).unwrap();

    let lexer = Lexer::new();
    let mut tokens = lexer.lex(code);
    //dbg!(tokens.clone());
    let node = parser(&mut tokens);
    // dbg!(node.clone());
    let mut emitter = Emitter::new();
    emitter.emit(node);
    emitter.print_to_file();

    // emit builtin functions.
    emit_builtin();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage rexparser \"<filepath>\"");
        process::exit(1);
    }
    let filepath = args[1].to_string();
    let code: String =
        fs::read_to_string(filepath).expect("something went wrong reading the file.");
    compiler(code);
}
