pub enum Node
{
	InnerNode(Box<Node>, Box<Node>,  Box<dyn Fn(bool, bool) -> bool>),
	LeafNode(bool),
}

pub fn eval(node: Node) -> bool
{
	let val = match node
	{
		Node::InnerNode(a, b, op) =>
		{
			let _nested_a = match *a
			{
				Node::InnerNode(n_a, n_b, n_op) => n_op(eval(*n_a), eval(*n_b)),
				Node::LeafNode(b) => b,
			};

			let _nested_b = match *b
			{
				Node::InnerNode(n_a, n_b, n_op) => n_op(eval(*n_a), eval(*n_b)),
				Node::LeafNode(b) => b,
			};

			op(_nested_a, _nested_b)
			
		},
		Node::LeafNode(b) => b
	};

	val
}
