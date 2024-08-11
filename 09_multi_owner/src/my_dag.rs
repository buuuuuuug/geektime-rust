use std::cell::RefCell;
use std::rc::Rc;

#[allow(dead_code)]
#[derive(Debug)]
struct Node {
    data: usize,
    next: Option<Rc<RefCell<Node>>> // Option--实现可能有next，也可能没有next ｜ Rc -- 多所有者 ｜ RefCell -- 实现Node的可变借用（内部可变形）
}

impl Node {

    pub fn new(val: usize) -> Self {
        Self {
            data: val,
            next: None
        }
    }

    pub fn set_next(&mut self, next: Rc<RefCell<Node>>) {
        self.next = Some(next);
    }

    pub fn get_next(&self) -> Option<Rc<RefCell<Node>>> {
        self.next.as_ref().cloned()
    }
}

fn main() {
    let mut node1 = Node::new(1);
    let mut node2 = Node::new(2);
    let mut node3 = Node::new(3);
    let node4 = Node::new(4);
    let node5 = Node::new(5);
    
    node2.set_next(Rc::new(RefCell::new(node4)));
    node1.set_next(Rc::new(RefCell::new(node2)));
    println!("node1: {:?}", &node1);
    node3.set_next(node1.get_next().unwrap());
    println!("node3: {:?}", node3);

    node1.get_next().unwrap().borrow_mut().set_next(Rc::new(RefCell::new(node5)));
    
    println!("node1: {:?}", node1);
    println!("node3: {:?}", node3);
    
}