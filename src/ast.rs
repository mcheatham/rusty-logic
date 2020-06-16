pub struct Node<'a> {
    pub val: &'a str,
    pub l: Option<Box<Node<'a>>>,
    pub r: Option<Box<Node<'a>>>,
}

impl<'a> Node<'a> {
    pub fn set_left(&mut self, new_val: &'a str) {
        let child_node = Node{val:new_val, l:None, r:None};
        self.l = Some(Box::new(child_node));
    }

    pub fn set_right(&mut self, new_val: &'a str) {
        let child_node = Node{val:new_val, l:None, r:None};
        self.r = Some(Box::new(child_node));
    }

    pub fn print(&self) {
        match &self.l {
            &Some(ref child) => {
                child.print();
            }
            &None => { }
        }
        println!("{}", self.val);
        match &self.r {
            &Some(ref child) => {
                child.print();
            }
            &None => { }
        }
    }
}

