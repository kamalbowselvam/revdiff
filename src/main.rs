mod node;
use node::Node;
use node::operation::Propogation;
use node::operation::Opeartion;



fn main() {
    
    let n1 = Node::new(10, String::from("A"));
    let n2 = Node::new(20, String::from("B"));
    let d = Opeartion::new("Add").forward(&n1, &n2);
    Node::backward(&d, None, None);
    println!("{:?}", d.borrow().get_value());
}

