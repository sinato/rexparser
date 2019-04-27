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
            for _ in 0..depth {
                print!("|       ");
            }
            print!("└────── ");
            print_node(*node.rhs, depth + 1, width);
        }
        Node::Suffix(node) => {
            print!("{}", node.suffix);
            print_dash(default_dash_num - node.suffix.token.get_len());
            print_node(*node.node, depth + 1, width);
        }
        Node::Token(node) => {
            node.token.print();
            print!("\n");
        }
    }
}
