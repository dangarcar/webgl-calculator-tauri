use std::fmt::Display;

use super::operations::{BinaryOperation, NAryOperation, UnaryOperation};

#[derive(Debug, Clone)]
pub enum Node {
    Constant {
        value: f64
    },
    Variable {
        name: String
    },
    Unary {
        op_type: UnaryOperation,
        
        child: Option<Box<Node>>
    },
    Binary {
        op_type: BinaryOperation,
        
        lhs: Option<Box<Node>>,
        rhs: Option<Box<Node>>,
    },
    NAry {
        op_type: NAryOperation,

        children: Vec<Box<Node>>,
    },
    Unknown {
        name: String
    },
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::NAry { op_type,.. } => write!(f, "N-ary {{ {:?} }}", op_type),
            Node::Binary { op_type, .. } => write!(f, "Binary {{ {:?} }}", op_type),
            Node::Unary { op_type, .. } => write!(f, "Unary {{ {:?} }}", op_type),
            _ => write!(f, "{:?}", self)
        }
    }
}

impl Node {
    pub fn print_tree(&self) {
        println!("-Root");
        print_tree("", self, true);
    } 

    pub fn add(a: Node, b: Node) -> Self {
        Self::NAry { 
            op_type: NAryOperation::Add, 
            children: vec![
                Box::new(a),
                Box::new(b),
            ] 
        }
    }

    pub fn multiply(a: Node, b: Node) -> Self {
        Self::NAry { 
            op_type: NAryOperation::Multiply, 
            children: vec![
                Box::new(a),
                Box::new(b),
            ] 
        }
    }

    pub fn divide(a: Node, b: Node) -> Self {
        Self::Binary { 
            op_type: BinaryOperation::Division, 
            lhs: Some(Box::new(a)), 
            rhs: Some(Box::new(b)) 
        }
    }

    pub fn op(op_type: UnaryOperation, a: Node) -> Self {
        Self::Unary { 
            op_type, 
            child: Some(Box::new(a)),
        }
    }

    pub fn substract(a: Node, b: Node) -> Self {
        Self::NAry { 
            op_type: NAryOperation::Add, 
            children: vec![
                Box::new(a),
                Box::new(Node::Unary { op_type: UnaryOperation::Minus, child: Some(Box::new(b)) }),
            ] 
        }
    }
}

fn print_tree(prefix: &str, root: &Node, last: bool) {
    println!("{prefix}{}{root}", if last { "└──" } else { "├──" });
    
    let new_prefix = prefix.to_owned() + if last { "    " } else { "|   " };
    match root {
        Node::Unary { child, .. } => {
            if let Some(c) = child {
                print_tree(&new_prefix, &c, true);
            }
        },
        Node::Binary { lhs, rhs, .. } => {
            if let Some(l) = lhs {
                print_tree(&new_prefix, &l, false);
            }
            if let Some(r) = rhs {
                print_tree(&new_prefix, &r, true);
            }
        },
        Node::NAry { children,.. } => {
            for (i, n) in children.iter().enumerate() {
                print_tree(&new_prefix, n, i==children.len()-1);
            }
        }
        _ => (),
    }
}