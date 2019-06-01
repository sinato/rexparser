use crate::lexer::token::Tokens;
use crate::parser::declare::*;
use crate::parser::expression::node::*;
use crate::parser::statement::*;

use std::fs;

pub fn toplevel(tokens: &mut Tokens) -> DeclareNode {
    DeclareNode::new(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::lexer;

    fn run(input: String) -> DeclareNode {
        let lexer = lexer::Lexer::new();
        let mut tokens = lexer.lex(input);
        println!("tokens: {:?}", tokens);
        toplevel(&mut tokens)
    }

    fn get_code(filename: &str) -> String {
        let filename = String::from("./tests/resources/") + filename;
        let code: String =
            fs::read_to_string(filename).expect("something went wrong reading the file.");
        code
    }

    fn get_num(num: i32) -> ExpressionNode {
        ExpressionNode::Token(TokenNode {
            token: Token::IntNum(num.to_string()),
        })
    }

    #[test]
    fn test_parse_single_num() {
        let code = get_code("test_single_num.c");
        let actual = run(code);

        let identifier = String::from("main");
        let return_type = BasicType::Int;
        let expression = get_num(1);
        let statement = StatementNode::Return(ReturnStatementNode { expression });
        let expected = DeclareNode::Function(FunctionNode {
            identifier,
            return_type,
            statement,
        });

        assert_eq!(actual, expected);
    }
}
