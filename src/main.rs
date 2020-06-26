mod ast;
mod enum_ast;

use ast::Node;

fn main() {
	
    let mut tree_root = Node{val:"and", l:None, r:None};
    tree_root.set_left("A");
    tree_root.set_right("B");
    tree_root.print();

    let leaf_1: enum_ast::Node = enum_ast::make_leaf_node(true);
    let leaf_2: enum_ast::Node = enum_ast::make_leaf_node(false);
    let leaf_3: enum_ast::Node = enum_ast::make_leaf_node(true);
    let leaf_4: enum_ast::Node = enum_ast::make_leaf_node(false);

    let inner_1_2: enum_ast::Node = enum_ast::make_binary_inner_node(leaf_1, leaf_2, Box::new(|a, b| a || b));
    let inner_3_4: enum_ast::Node = enum_ast::make_binary_inner_node(leaf_3, leaf_4, Box::new(|a, b| a && b));

    let inner_12_34: enum_ast::Node = enum_ast::make_binary_inner_node(inner_1_2, inner_3_4, Box::new(|a, b| a && b));
    let unary_last: enum_ast::Node = enum_ast::make_unary_inner_node(inner_12_34, Box::new(|a| !a));

    let bool_ret = enum_ast::eval(&unary_last);
    println!("{}", bool_ret);

    println!("{}", enum_ast::print_expression(&unary_last));
}
