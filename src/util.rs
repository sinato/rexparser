use crate::parser::node::Node;

fn print_dash(l: usize) {
    print!(" ");
    for _ in 0..l {
        print!("─");
    }
    print!(" ");
}

pub fn print_node(node: Node, depth: u32, width: u32) {
    let default_dash_num = 6;
    match node {
        Node::BinExp(node) => {
            print!("{}", node.op);
            print_dash(default_dash_num - node.op.token.get_len());
            print_node(*node.lhs, depth + 1, width);
            print!("\n");
            for _ in 0..depth {
                print!("|       ");
            }
            print!("└────── ");
            print_node(*node.rhs, depth + 1, width);
            print!("\n");
        }
        Node::Suffix(node) => {
            print!("{}", node.suffix);
            print_dash(default_dash_num - node.suffix.token.get_len());
            print_node(*node.node, depth + 1, width);
            print!("\n");
        }
        Node::ArrayIndex(node) => {
            print_node(*node.array, depth + 1, width);
            print_dash(default_dash_num - 2);
            print!("{}", "[]");
            print_dash(default_dash_num - 2);
            print_node(*node.index, depth + 1, width);
            print!("\n");
        }
        Node::FunctionCall(node) => {
            node.identifier.token.print();
            print_dash(default_dash_num - node.identifier.token.get_len());
            print!("{}", "()");
            let mut parameters = node.parameters;
            parameters.reverse();
            if let Some(parameter) = parameters.pop() {
                print_dash(default_dash_num - 2);
                print_node(*parameter, depth + 1, width);
            }
            if parameters.len() > 0 {
                print!("\n");
            }
            while let Some(parameter) = parameters.pop() {
                for _ in 0..depth + 1 {
                    print!("|       ");
                }
                print!("└────── ");
                print_node(*parameter, depth + 1, width);
                print!("\n");
            }
        }
        Node::Token(node) => {
            node.token.print();
        }
    }
}
