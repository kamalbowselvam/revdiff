use super::*;
use crate::node::operation::Propogation;
use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Clone)]
pub struct Add {
    parents: Vec<Rc<RefCell<Node>>>,
}

impl Add {
    pub fn new() -> Self {
        Self { parents: vec![] }
    }

    pub fn add_parents(&mut self, node: Rc<RefCell<Node>>) {
        self.parents.push(node)
    }

    pub fn forward(
        &self,
        node1: &Rc<RefCell<Node>>,
        node2: &Rc<RefCell<Node>>,
    ) -> Rc<RefCell<Node>> {
        let l1 = node1.borrow().get_value().clone(); // cloning here to avoid move
        let l2 = node2.borrow().get_value().clone();
        let out_value = l1 + l2;
        let s1 = node1.borrow().label.clone();
        let s2 = node2.borrow().label.clone();
        let s = s1 + " + " + &s2;
        let new_node = Node::new(out_value, s); // creating a new node
        let nrc = Rc::new(RefCell::new(Opeartion::Add(self.clone()))); // creating new op
        nrc.borrow_mut().add_parents(Rc::clone(node1)); // attaching parents to ops
        nrc.borrow_mut().add_parents(Rc::clone(node2));
        new_node.borrow_mut().add_operation(nrc); // attaching op to  child
        node1.borrow_mut().add_children(Rc::clone(&new_node)); // attaching child to parent
        node2.borrow_mut().add_children(Rc::clone(&new_node));
        new_node
    }

    pub fn backward(&self, grad: Array2<f32>, node: Option<Rc<RefCell<Node>>>) {
        match node {
            Some(x) => {
                let a_grad = 1.0 * grad.clone();
                Node::backward(
                    &Rc::clone(&self.parents[0]),
                    Some(a_grad),
                    Some(Rc::clone(&x)),
                );
                let b_grad = 1.0 * grad.clone();
                Node::backward(
                    &Rc::clone(&self.parents[1]),
                    Some(b_grad),
                    Some(Rc::clone(&x)),
                );
            }
            None => (),
        }
    }
}
