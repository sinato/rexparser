use crate::lexer::token::{Token, Tokens};
use crate::parser::expression::node::*;

pub fn toplevel(tokens: &mut Tokens) -> ExpressionNode {
    BinExpNode::new(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::lexer;
    use crate::parser::expression::util::print_node;

    fn run(input: String) -> ExpressionNode {
        let lexer = lexer::Lexer::new();
        let mut tokens = lexer.lex(input);
        toplevel(&mut tokens)
    }
    fn get_num(num: i32) -> ExpressionNode {
        ExpressionNode::Token(TokenNode {
            token: Token::Num(num),
        })
    }
    fn get_ide(ide: &str) -> ExpressionNode {
        ExpressionNode::Token(TokenNode {
            token: Token::Ide(String::from(ide)),
        })
    }
    fn get_op(op: &str) -> TokenNode {
        let op = String::from(op);
        let property = lexer::get_property(&op);
        TokenNode {
            token: Token::Op(op, property),
        }
    }
    fn get_suffix(op: &str) -> TokenNode {
        let op = String::from(op);
        TokenNode {
            token: Token::SuffixOp(op),
        }
    }
    fn get_prefix(op: &str) -> TokenNode {
        let op = String::from(op);
        TokenNode {
            token: Token::PrefixOp(op),
        }
    }
    fn get_bin_exp(op: &str, lhs: ExpressionNode, rhs: ExpressionNode) -> ExpressionNode {
        ExpressionNode::BinExp(BinExpNode {
            op: get_op(op),
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        })
    }
    fn get_suffix_exp(suffix: &str, lhs: ExpressionNode) -> ExpressionNode {
        ExpressionNode::Suffix(SuffixNode {
            suffix: get_suffix(suffix),
            node: Box::new(lhs),
        })
    }
    fn get_prefix_exp(prefix: &str, lhs: ExpressionNode) -> ExpressionNode {
        ExpressionNode::Prefix(PrefixNode {
            prefix: get_prefix(prefix),
            node: Box::new(lhs),
        })
    }
    fn get_array_index_exp(array: ExpressionNode, index: ExpressionNode) -> ExpressionNode {
        ExpressionNode::ArrayIndex(ArrayIndexNode {
            array: Box::new(array),
            index: Box::new(index),
        })
    }
    fn get_function_call_exp(identifier: TokenNode, parameters: ExpressionNode) -> ExpressionNode {
        ExpressionNode::FunctionCall(FunctionCallNode {
            identifier,
            parameters: Box::new(parameters),
        })
    }
    fn get_ternary_exp(
        condition: ExpressionNode,
        lhs: ExpressionNode,
        rhs: ExpressionNode,
    ) -> ExpressionNode {
        ExpressionNode::TernaryExp(TernaryExpNode {
            condition: Box::new(condition),
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        })
    }
    #[allow(dead_code)]
    fn show(actual: ExpressionNode, expected: ExpressionNode) {
        println!("actual   ============");
        print_node(actual, 0);
        println!("expected ============");
        print_node(expected, 0);
    }

    #[test]
    fn test_add() {
        let actual = run(String::from("1 + 2"));
        let lhs = get_num(1);
        let rhs = get_num(2);
        let expected = get_bin_exp("+", lhs, rhs);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_mul() {
        let actual = run(String::from("1 * 2"));
        let lhs = get_num(1);
        let rhs = get_num(2);
        let expected = get_bin_exp("*", lhs, rhs);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_three_terms() {
        let actual = run(String::from("1 * 2 + 3"));
        let lhs = get_num(1);
        let rhs = get_num(2);

        let lhs = get_bin_exp("*", lhs, rhs);
        let rhs = get_num(3);

        let expected = get_bin_exp("+", lhs, rhs);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_multi_terms() {
        let actual = run(String::from("1 * 2 + 3 * 4"));
        // expect: (1 * 2) + (3 * 4)
        let lhs1 = get_num(1);
        let rhs1 = get_num(2);
        let lhs = get_bin_exp("*", lhs1, rhs1);

        let lhs2 = get_num(3);
        let rhs2 = get_num(4);
        let rhs = get_bin_exp("*", lhs2, rhs2);

        let expected = get_bin_exp("+", lhs, rhs);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_multi_terms2() {
        let actual = run(String::from("1 + 2 * 3 + 4 * 5"));
        // expect: (1 + (2 * 3)) + (4 * 5)
        let lhs1 = get_num(1);
        let lhs2 = get_bin_exp("*", get_num(2), get_num(3));
        let lhs = get_bin_exp("+", lhs1, lhs2);
        let rhs = get_bin_exp("*", get_num(4), get_num(5));
        let expected = get_bin_exp("+", lhs, rhs);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_multi_terms_with_assign() {
        let actual = run(String::from("a = b = 1 + 2 * 3 + 4"));
        // expect: a = (b = ((1 + (2 * 3)) + 4))
        let lhs = get_ide("a");
        let rhs0 = get_bin_exp("*", get_num(2), get_num(3));
        let rhs1 = get_bin_exp("+", get_num(1), rhs0);
        let rhs2 = get_bin_exp("+", rhs1, get_num(4));
        let rhs = get_bin_exp("=", get_ide("b"), rhs2);
        let expected = get_bin_exp("=", lhs, rhs);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_suffix_increment() {
        let actual = run(String::from("a++"));
        // expect: a++
        let lhs = get_ide("a");
        let expected = get_suffix_exp("++", lhs);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_prefix_increment() {
        let actual = run(String::from("b = ++a * 1 + 2"));
        let lhs = get_ide("b");
        let rhs = get_prefix_exp("++", get_ide("a"));
        let rhs = get_bin_exp("*", rhs, get_num(1));
        let rhs = get_bin_exp("+", rhs, get_num(2));
        let expected = get_bin_exp("=", lhs, rhs);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_sign_operator() {
        let actual = run(String::from("a = +1 + -2"));
        let lhs = get_ide("a");
        let rhs1 = get_prefix_exp("+", get_num(1));
        let rhs2 = get_prefix_exp("-", get_num(2));
        let rhs = get_bin_exp("+", rhs1, rhs2);
        let expected = get_bin_exp("=", lhs, rhs);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_array_index() {
        let actual = run(String::from("a = b[1 + 2]"));
        // expect: a = (b[(1 + 2)])
        let lhs = get_ide("a");
        let index = get_bin_exp("+", get_num(1), get_num(2));
        let array = get_ide("b");
        let rhs = get_array_index_exp(array, index);
        let expected = get_bin_exp("=", lhs, rhs);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_array_multi() {
        let actual = run(String::from("b = a[1][2][3]++ * 4 + 5"));
        let lhs = get_ide("b");
        let array = get_array_index_exp(get_ide("a"), get_num(1));
        let array = get_array_index_exp(array, get_num(2));
        let array = get_array_index_exp(array, get_num(3));
        let array_with_suffix = get_suffix_exp("++", array);
        let rhs = get_bin_exp("*", array_with_suffix, get_num(4));
        let rhs = get_bin_exp("+", rhs, get_num(5));
        let expected = get_bin_exp("=", lhs, rhs);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_function_call() {
        let actual = run(String::from("a = func(1, 2, 3) + 5"));
        let lhs = get_ide("a");
        let identifier = TokenNode {
            token: Token::Ide("func".to_string()),
        };
        let parameters = get_bin_exp(",", get_num(1), get_num(2));
        let parameters = get_bin_exp(",", parameters, get_num(3));
        let function_call = get_function_call_exp(identifier, parameters);
        let rhs = get_bin_exp("+", function_call, get_num(5));
        let expected = get_bin_exp("=", lhs, rhs);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_function_call_without_args() {
        let actual = run(String::from("a = func() + 5"));
        let lhs = get_ide("a");
        let identifier = TokenNode {
            token: Token::Ide("func".to_string()),
        };
        let parameters = ExpressionNode::Empty;
        let function_call = get_function_call_exp(identifier, parameters);
        let rhs = get_bin_exp("+", function_call, get_num(5));
        let expected = get_bin_exp("=", lhs, rhs);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_commma() {
        let actual = run(String::from("a = 1 + 2 * 3, 4 + 5"));
        let lhs = get_bin_exp("*", get_num(2), get_num(3));
        let lhs = get_bin_exp("+", get_num(1), lhs);
        let lhs = get_bin_exp("=", get_ide("a"), lhs);
        let rhs = get_bin_exp("+", get_num(4), get_num(5));
        let expected = get_bin_exp(",", lhs, rhs);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_ternary() {
        let actual = run(String::from("a = 1 ? 10 * 20 + 30 : b++"));
        let lhs = get_ide("a");
        let condition = get_num(1);
        let ternary_lhs = get_bin_exp("*", get_num(10), get_num(20));
        let ternary_lhs = get_bin_exp("+", ternary_lhs, get_num(30));
        let ternary_rhs = get_suffix_exp("++", get_ide("b"));
        let rhs = get_ternary_exp(condition, ternary_lhs, ternary_rhs);
        let expected = get_bin_exp("=", lhs, rhs);
        assert_eq!(actual, expected);
    }
    #[test]
    fn test_parenthesis() {
        let actual = run(String::from("a = (1 * (2 + 3)) * 4"));
        let rhs = get_bin_exp("+", get_num(2), get_num(3));
        let rhs = get_bin_exp("*", get_num(1), rhs);
        let rhs = get_bin_exp("*", rhs, get_num(4));
        let expected = get_bin_exp("=", get_ide("a"), rhs);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_all() {
        let actual = run(String::from("a = b = 1 + 2 * 3++ + 4"));
        // expect: a = (b = ((1 + (2 * 3)) + 4))
        let lhs = get_ide("a");
        let rhs_with_suffix = get_suffix_exp("++", get_num(3));
        let rhs0 = get_bin_exp("*", get_num(2), rhs_with_suffix);
        let rhs1 = get_bin_exp("+", get_num(1), rhs0);
        let rhs2 = get_bin_exp("+", rhs1, get_num(4));
        let rhs = get_bin_exp("=", get_ide("b"), rhs2);
        let expected = get_bin_exp("=", lhs, rhs);
        assert_eq!(actual, expected);
    }
}
