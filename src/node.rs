use std::{cell::RefCell, rc::Rc};
pub mod operation;
use operation::Opeartion;
use operation::Propogation;
use ndarray::Array2;



#[derive(Debug, Clone)]
pub struct Node {
    label: String,
    value: Array2<f32>,
    grad: Array2<f32>,
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

    pub fn get_value(&self) -> Array2<f32> {
        return self.value.clone();
    }

    pub fn get_grad(&self) -> Array2<f32>  {
        return self.grad.clone();
    }

    pub fn increase_grad(&mut self, grad: Array2<f32>) {
        self.grad = self.grad.clone() + grad;
    }

    pub fn new(value: Array2<f32>, name: String) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            label: name,
            value: value.clone(),
            grad: Array2::<f32>::zeros(value.dim()),
            opeartion: None,
            children: vec![],
            requires_grad: true,
        }))
    }

    pub fn backward(
        pnode: &Rc<RefCell<Node>>,
        grad: Option<Array2<f32>>,
        node: Option<Rc<RefCell<Node>>>,
    ) {
        if pnode.borrow().requires_grad == false {
            print!("This tensor has requires gradient to false")
        }

        let x = match grad {
            Some(x) => x,
            None => Array2::<f32>::ones(pnode.borrow().get_value().dim())
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
