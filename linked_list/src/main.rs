struct BinarySearchTree {
    head : Option<Box<Node>>,
}


struct Node {
    value : u32,
     left : Option<Box<Node>>,
    right : Option<Box<Node>>,
}

impl BinarySearchTree {

    fn new(value : u32) -> Self {

        BinarySearchTree{
            head : Some(Box::new(Node{value, left : None, right: None}))
        }
    }

    fn insert(&mut self, value : u32) {

        
        let mut current =  self.head.as_mut(); 

        loop {

            match current { 

                None => break, 

                Some(node) => 
                { 
                    if ( node.value > value ) {
                        match node.left {
                            None => {
                                node.left = Some(())

                            }

                            Some() => {
                                

                            }

                        }
                       
                    }
            
                },

            }

            return;
        }


    }

}


fn main() {

    let mut x = BinarySearchTree::new(3);

    x.insert(2);


}