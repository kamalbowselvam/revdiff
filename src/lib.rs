mod node;

#[cfg(test)]
mod tests {

    use node::Node;
    use node::operation::Propogation;
    use node::operation::Opeartion;
    use node::operation::add::Add;

    use super::*;
    #[test]
    fn add_node_forward() {
        let n1 = Node::new(10, String::from("A"));
        let n2 = Node::new(20, String::from("B"));
        let d = Opeartion::new("Add").forward(&n1, &n2);
        Node::backward(&d, None, None);
        assert_eq!(d.borrow().get_value(), 30);
    }

    #[test]
    fn add_node_backward() {
        let n1 = Node::new(10, String::from("A"));
        let n2 = Node::new(20, String::from("B"));
        let d = Opeartion::new("Add").forward(&n1, &n2);
        Node::backward(&d, None, None);
        assert_eq!(n1.borrow().get_grad(), 1);
        assert_eq!(n2.borrow().get_grad(), 1);
    
    }


    #[test]
    fn mul_node_forward() {
        let n1 = Node::new(10, String::from("A"));
        let n2 = Node::new(20, String::from("B"));
        let d = Opeartion::new("Mul").forward(&n1, &n2);
        Node::backward(&d, None, None);
        assert_eq!(d.borrow().get_value(), 200);
    }

    #[test]
    fn mul_node_backward() {
        let n1 = Node::new(10, String::from("A"));
        let n2 = Node::new(20, String::from("B"));
        let d = Opeartion::new("Mul").forward(&n1, &n2);
        Node::backward(&d, None, None);
        assert_eq!(n1.borrow().get_grad(), 20);
        assert_eq!(n2.borrow().get_grad(), 10);
    }
}