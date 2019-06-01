pub mod declare;
pub mod expression;
pub mod statement;

use crate::lexer::lexer::Lexer;
use crate::parser::expression::parser::toplevel;
use crate::parser::expression::util::print_entry;

pub fn run() {
    // let input = String::from("a = b = 1 + 2++ * 3 + 4++");
    // let input = String::from("-1 + +3");
    // let input = String::from("a = (1 * (2++ + -3)) * 4");
    let input = String::from("a = (1 * (2 + 3)) * 4");
    let lexer = Lexer::new();
    let tokens = lexer.lex(input);
    println!("tokens: {:?}", tokens);
    let node = toplevel(tokens);
    // dbg!(node);
    print_entry(node);
}
