mod lexer;
mod node;
mod parser;
mod token;
mod util;

use parser::parse_entry;
use util::print_node;

/// expression := num op num (op num)?
fn main() {
    let input = String::from("a = b = 1 + 2 * 3 + 4");
    let lexer = lexer::Lexer::new();
    let tokens = lexer.lex(input);
    println!("tokens: {:?}", tokens);
    let node = parse_entry(tokens);
    print_node(node, 0, 0);
}
