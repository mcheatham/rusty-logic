mod ast;

use ast::Node;

fn main() {
    let mut tree_root = Node{val:"and", l:None, r:None};
    tree_root.set_left("A");
    tree_root.set_right("B");
    tree_root.print();
}
