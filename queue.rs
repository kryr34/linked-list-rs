use std::rc::Rc;
use std::cell::RefCell;
use std::mem;

struct Node {
    value: usize,
    next: Option<Rc<RefCell<Node>>>,
}
struct List {
    head: Option<Rc<RefCell<Node>>>,
    tail: Option<Rc<RefCell<Node>>>,
}
impl List {
    fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }
    fn push(&mut self, elem: usize) {
        let newtail = Rc::new(RefCell::new(Node{value:elem, next: None}));
        if let Some(ref x) = self.tail {
            x.borrow_mut().next = Some(Rc::clone(&newtail));
        } else {
            self.tail = Some(Rc::clone(&newtail));
            self.head = Some(Rc::clone(&newtail));
        }
    }
    fn pop(&mut self) -> usize {
        let head = mem::take(&mut self.head);
        if let Some(ref x) = head {
            let mut x = x.borrow_mut();
            self.head = mem::take(&mut x.next);
            if self.head.is_none() {
                self.tail = None;
            }
            x.value
        } else {
            panic!("headless panic");
        }
    }
}

fn main() {
    let mut list = List::new();

    list.push(1);
    assert_eq!(1, list.pop());
    
    list.push(1);
    list.push(2);
    assert_eq!(1, list.pop());
    assert_eq!(2, list.pop());
}
