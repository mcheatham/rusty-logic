pub enum Node
{
	BinaryInnerNode(Box<Node>, Box<Node>,  Box<dyn Fn(bool, bool) -> bool>),
	UnaryInnerNode(Box<Node>, Box<dyn Fn(bool) -> bool>),
	LeafNode(bool),
}

pub fn eval(node: &Node) -> bool
{
	let val = match &*node
	{
		Node::BinaryInnerNode(a, b, op) =>
		{
			let _nested_a = match &**a
			{
				Node::BinaryInnerNode(n_a, n_b, n_op) => n_op(eval(&*n_a), eval(&*n_b)),
				Node::UnaryInnerNode(b, n_op) => n_op(eval(&*b)),
				Node::LeafNode(b) => *b,
			};

			let _nested_b = match &**b
			{
				Node::BinaryInnerNode(n_a, n_b, n_op) => n_op(eval(&*n_a), eval(&*n_b)),
				Node::UnaryInnerNode(b, n_op) => n_op(eval(&*b)),
				Node::LeafNode(b) => *b,
			};

			op(_nested_a, _nested_b)
			
		},
		Node::UnaryInnerNode(b, op) => op(eval(&*b)),
		Node::LeafNode(b) => *b
	};

	val
}

pub fn print_expression(node: &Node) -> String
{
	let mut s: String = String::from("");

	match node
	{
		Node::BinaryInnerNode(a, b, _op) => 
		{
			s.push_str(format!("{}{}{}{}{}", "[ ", print_expression(&*a).as_str(), " OP ", print_expression(&*b).as_str(), " ]").as_str());
		},
		Node::UnaryInnerNode(b, _op) =>
		{
			s.push_str(format!("{}{}{}{}", "[ ", " OP ", print_expression(&*b).as_str(), " ]").as_str());
		},
		Node::LeafNode(b) =>
		{
			s.push_str(format!("{}", b).as_str());
		},
	}
	s
}

pub fn make_binary_inner_node(left_node: Node, right_node: Node, op: Box<dyn Fn(bool, bool) -> bool>) -> Node
{
	let node = Node::BinaryInnerNode(Box::new(left_node), Box::new(right_node), op);
	node
}

pub fn make_unary_inner_node(child_node: Node, op: Box<dyn Fn(bool) -> bool>) -> Node
{
	let node = Node::UnaryInnerNode(Box::new(child_node), op);
	node
}

pub fn make_leaf_node(val: bool) -> Node
{
	let node = Node::LeafNode(val);
	node
}
