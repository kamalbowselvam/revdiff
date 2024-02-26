use std::rc::Rc;
mod node;
use node::Node;
use node::operation::Opeartion;
use node::operation::Add;

fn main() {
    
    let n1 = Node::new(10, String::from("A"));
    let n2 = Node::new(20, String::from("B"));
    let n3 = Node::new(50, String::from("C"));
    let d = Add::forward(&n1, &n2);
    let e = Add::forward(&d, &n3);
    let chk = Rc::clone(&e);
    Node::backward(e, None, None);
    println!("after adding {:?}", chk.borrow().get_value());
}
