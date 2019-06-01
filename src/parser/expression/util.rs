use crate::parser::expression::node::ExpressionNode;

pub fn print_entry(node: ExpressionNode) {
    print_node(node, 0);
    print!("\n");
}

fn print_dash(l: usize) {
    print!(" ");
    for _ in 0..l {
        print!("─");
    }
    print!(" ");
}

pub fn print_node(node: ExpressionNode, depth: u32) {
    let default_dash_num = 6;
    match node {
        ExpressionNode::BinExp(node) => {
            print!("{}", node.op);
            print_dash(default_dash_num - node.op.token.get_len());
            print_node(*node.lhs, depth + 1);
            print!("\n");
            for _ in 0..depth {
                print!("|       ");
            }
            print!("└────── ");
            print_node(*node.rhs, depth + 1);
        }
        ExpressionNode::TernaryExp(node) => {
            print!("?");
            print_dash(default_dash_num - 1);
            print_node(*node.condition, depth + 1);
            for _ in 0..depth + 1 {
                print!("|       ");
            }
            print!("└────── ");
            print_node(*node.lhs, depth + 2);
            for _ in 0..depth + 1 {
                print!("|       ");
            }
            print!("└────── ");
            print_node(*node.rhs, depth + 2);
        }
        ExpressionNode::Prefix(node) => {
            print!("{}", node.prefix);
            print_dash(default_dash_num - node.prefix.token.get_len());
            print_node(*node.node, depth + 1);
        }
        ExpressionNode::Suffix(node) => {
            print!("{}", node.suffix);
            print_dash(default_dash_num - node.suffix.token.get_len());
            print_node(*node.node, depth + 1);
        }
        ExpressionNode::ArrayIndex(node) => {
            print_node(*node.array, depth + 1);
            print_dash(default_dash_num - 2);
            print!("{}", "[]");
            print_dash(default_dash_num - 2);
            print_node(*node.index, depth + 1);
        }
        ExpressionNode::FunctionCall(node) => {
            print!("{}", node.identifier.token);
            print_dash(default_dash_num - node.identifier.token.get_len());
            print!("{}", "()");
            print_node(*node.parameters, depth + 1);
            print!("\n");
        }
        ExpressionNode::Token(node) => {
            print!("{}", node.token);
        }
        ExpressionNode::Empty => {}
    }
}
