use crate::lexer::token::Tokens;
use crate::parser::declare::*;
use crate::parser::statement::*;

use std::fs;

pub fn toplevel(mut tokens: Tokens) -> Node {
    Node::new(&mut tokens)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::lexer;

    fn run(input: String) -> Node {
        let lexer = lexer::Lexer::new();
        let tokens = lexer.lex(input);
        toplevel(tokens)
    }

    fn get_code(filename: &str) -> String {
        let filename = String::from("./tests/resources/") + filename;
        let code: String =
            fs::read_to_string(filename).expect("something went wrong reading the file.");
        code
    }

    #[test]
    fn test_parse_single_num() {
        let code = get_code("test_single_num.c");
        let actual = run(code);

        let identifier = String::from("main");
        let statement = StatementNode { val: 1 };
        let expected = Node::Function(FunctionNode {
            identifier,
            statement,
        });

        assert_eq!(actual, expected);
    }
}
