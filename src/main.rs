mod lexer;
mod parser;

/// expression := num op num (op num)?
fn main() {
    parser::run();
}
