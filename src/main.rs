mod ast;
mod enum_ast;

use ast::Node;

fn main() {
    let mut tree_root = Node{val:"and", l:None, r:None};
    tree_root.set_left("A");
    tree_root.set_right("B");
    tree_root.print();

    let a: Box<enum_ast::Node> = Box::new(enum_ast::Node::LeafNode(true));
    let b: Box<enum_ast::Node> = Box::new(enum_ast::Node::LeafNode(false));
    let my_fn: Box<dyn Fn(bool, bool) -> bool> = Box::new(|a, b| a || b);

    let inner_node = enum_ast::Node::InnerNode(a, b, my_fn);

    let inner_ret = enum_ast::eval(inner_node);
    println!("{}", inner_ret);
}
