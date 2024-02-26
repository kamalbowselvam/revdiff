mod node;
use ndarray::Array2;
use node::Node;
use node::operation::Propogation;
use node::operation::Opeartion;


fn main() {
    
    let n1 = Node::new(Array2::<f32>::ones((2,2)), String::from("A"));
    let n2 = Node::new(Array2::<f32>::ones((2,2)), String::from("B"));
    let d = Opeartion::new("Mul").forward(&n1, &n2);
    //let e = Opeartion::new("Mul").forward(&d, &n3);


    Node::backward(&d, None, None);
    println!("{:?}", d.borrow().get_value());
    println!("{:?}", n1.borrow().get_grad());
    //println!("{:?}", n2.borrow().get_grad());
}

