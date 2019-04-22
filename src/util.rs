use crate::node::Node;

pub fn print_node(node: Node, depth: u32, width: u32) {
    match node {
        Node::BinExp(node) => {
            print!("{} ── ", node.op);
            print_node(*node.lhs, depth + 1, width);
            for _ in 0..depth {
                print!("|    ");
            }
            print!("└─── ");
            print_node(*node.rhs, depth + 1, width);
        }
        Node::Token(node) => {
            node.token.print();
            print!("\n");
        }
    }
}
