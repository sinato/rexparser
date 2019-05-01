mod lexer;
mod parser;
mod util;

use lexer::lexer::Lexer;
use parser::parser::toplevel;
use util::print_entry;

/// expression := num op num (op num)?
fn main() {
    // let input = String::from("a = b = 1 + 2++ * 3 + 4++");
    let input = String::from("b = ++a * 1 + 2");
    let lexer = Lexer::new();
    let tokens = lexer.lex(input);
    println!("tokens: {:?}", tokens);
    let node = toplevel(tokens);
    // dbg!(node);
    print_entry(node);
}
