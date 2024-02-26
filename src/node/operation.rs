use super::*;
use std::{cell::RefCell, rc::Rc};
pub mod add;
pub mod mul;
use add::Add;
use mul::Mul;

pub trait Propogation {
    fn new(t: &str) -> Opeartion;
    fn add_parents(&mut self, node: Rc<RefCell<Node>>);
    fn forward(&self, node1: &Rc<RefCell<Node>>, node2: &Rc<RefCell<Node>>) -> Rc<RefCell<Node>>;
    fn backward(&self, grad: Array2<f32>, node: Option<Rc<RefCell<Node>>>);
}

#[derive(Debug, Clone)]
pub enum Opeartion {
    Add(Add),
    Mul(Mul),
}

impl Propogation for Opeartion {
    fn new(t: &str) -> Opeartion {
        match t {
            "Add" => Opeartion::Add(Add::new()),
            "Mul" => Opeartion::Mul(Mul::new()),
            _ => panic!("Operation name is needed"),
        }
    }

    fn add_parents(&mut self, node: Rc<RefCell<Node>>) {
        match self {
            Opeartion::Add(t) => t.add_parents(node),
            Opeartion::Mul(t) => t.add_parents(node),
        }
    }

    fn forward(&self, node1: &Rc<RefCell<Node>>, node2: &Rc<RefCell<Node>>) -> Rc<RefCell<Node>> {
        match self {
            Opeartion::Add(t) => t.forward(node1, node2),
            Opeartion::Mul(t) => t.forward(node1, node2),
        }
    }

    fn backward(&self, grad: Array2<f32>, node: Option<Rc<RefCell<Node>>>) {
        match self {
            Opeartion::Add(t) => t.backward(grad, node),
            Opeartion::Mul(t) => t.backward(grad, node),
        }
    }
}
