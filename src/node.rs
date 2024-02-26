use std::{cell::RefCell, rc::Rc};
pub mod operation;
use operation::Opeartion;
use operation::Propogation;

#[derive(Debug, Clone)]
pub struct Node {
    label: String,
    value: Box<i32>,
    grad: Box<i32>,
    opeartion: Option<Rc<RefCell<Opeartion>>>,
    children: Vec<Rc<RefCell<Node>>>,
    requires_grad: bool,
}

impl Node {
    pub fn add_children(&mut self, node: Rc<RefCell<Node>>) {
        self.children.push(node)
    }

    pub fn add_operation(&mut self, op: Rc<RefCell<Opeartion>>) {
        self.opeartion = Some(op)
    }

    pub fn get_value(&self) -> i32 {
        return *self.value;
    }

    pub fn get_grad(&self) -> i32 {
        return *self.grad;
    }

    pub fn increase_grad(&mut self, grad: Box<i32>) {
        *self.grad = *self.grad + *grad
    }

    pub fn new(value: i32, name: String) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            label: name,
            value: Box::new(value),
            grad: Box::new(0),
            opeartion: None,
            children: vec![],
            requires_grad: true,
        }))
    }

    pub fn backward(
        pnode: &Rc<RefCell<Node>>,
        grad: Option<Box<i32>>,
        node: Option<Rc<RefCell<Node>>>,
    ) {
        if pnode.borrow().requires_grad == false {
            print!("This tensor has requires gradient to false")
        }

        let x = match grad {
            Some(x) => x,
            None => Box::new(1),
        };

        pnode.borrow_mut().increase_grad(x);

        // remove the children
        match node {
            Some(t) => {
                let c = Rc::clone(&pnode.borrow().children[0]);
                if std::ptr::eq(&*t.borrow(), &*c.borrow()) {
                    pnode.borrow_mut().children.remove(0);
                    println!("called to remove {} ", pnode.borrow().label);
                }
            }
            None => {
                println!("called for None");
            }
        }

        let nc = Rc::clone(&pnode);
        let n_grad = pnode.borrow_mut().grad.clone();

        if let Some(value) = &pnode.borrow().opeartion {
            if pnode.borrow().children.is_empty() {
                println!("called backprop in {}", pnode.borrow().label);
                value.borrow_mut().backward(n_grad, Some(nc));
            }
        }
    }
}
