mod lexer;
mod parser;
mod util;

use lexer::lexer::Lexer;
use parser::parser::toplevel;
use util::print_node;

/// expression := num op num (op num)?
fn main() {
    // let input = String::from("a = b = 1 + 2++ * 3 + 4++");
    let input = String::from("b = func(1, 2, 3) + 7 * 2434");
    let lexer = Lexer::new();
    let tokens = lexer.lex(input);
    println!("tokens: {:?}", tokens);
    let node = toplevel(tokens);
    print_node(node, 0, 0);
}
