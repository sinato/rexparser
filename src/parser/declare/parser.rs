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
        let mut statements: VecDeque<StatementNode> = VecDeque::new();
        statements.push_back(statement);
        let parameters: VecDeque<DeclareVariableNode> = VecDeque::new();
        let expected = DeclareNode::Function(FunctionNode {
            identifier,
            return_type,
            parameters,
            statements,
        });
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_int_declare() {
        let code = get_code("test_int_declare.c");
        let actual = run(code);

        let identifier = String::from("main");
        let return_type = BasicType::Int;

        let mut statements: VecDeque<StatementNode> = VecDeque::new();

        let declare_variable_node = DeclareVariableNode {
            value_type: BasicType::Int,
            identifier: String::from("a"),
            initialize_expression: None,
        };
        let statement = StatementNode::Declare(DeclareStatementNode {
            declare_variable_node,
        });
        statements.push_back(statement);

        let statement = StatementNode::Return(ReturnStatementNode {
            expression: get_num(42),
        });
        statements.push_back(statement);

        let parameters: VecDeque<DeclareVariableNode> = VecDeque::new();
        let expected = DeclareNode::Function(FunctionNode {
            identifier,
            return_type,
            parameters,
            statements,
        });
        assert_eq!(actual, expected);
    }
}
