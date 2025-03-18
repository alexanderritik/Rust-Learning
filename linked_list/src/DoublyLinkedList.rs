use std::{cell::RefCell, collections::LinkedList, mem::take, rc::Rc};


type pointer = Option<Rc<RefCell< Node> > >;

#[derive(Debug)]
struct Double_Linklist{
    head: pointer,
    tail: pointer,
}

#[derive(Debug)]
struct Node{
    element: i32,
    next: pointer,
    prev: pointer,
}

impl Double_Linklist {
    fn new() -> Double_Linklist {
        Double_Linklist{
            head: None,
            tail: None,
        }
    }

    fn add(&mut self, element: i32) {
        let new_head = Node::new(element);

        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(new_head.clone());
                new_head.borrow_mut().next = Some(old_head.clone());
                self.head = Some(new_head);
            }
            _ => {
                self.tail = Some(new_head.clone());
                self.head = Some(new_head);
            }
        }
    }


    fn remove(&mut self) -> Option<i32> {
        if self.head.is_none(){
            None
        }else{
            let removed_val = self.head.as_ref().unwrap().borrow().element;
            // self.head.
            self.head.take().map(|old_head| match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    new_head.borrow_mut().prev = None;
                    self.head = Some(new_head);
                    self.head.clone()
                }
                None => {
                    self.tail = None;
                    println!("List is empty after removal");
                    None
                }
            });
            Some(removed_val)
        }
    }

    fn print(&self) {
        let mut traversal = self.head.clone();
        while !traversal.is_none() {
            println!("{}", traversal.as_ref().unwrap().borrow().element);
            traversal = traversal.unwrap().borrow().next.clone();
        }
    }
}

impl Node {
    fn new(element: i32) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            element: element,
            next: None,
            prev: None,
        }))
    }
}

fn main() {
    let mut list = Double_Linklist::new();
    list.add(5);
    list.add(7);
    list.add(10);
    list.add(15);
    list.add(20);
    list.remove();
    list.print();
}
use std::{cell::RefCell, collections::LinkedList, mem::take, rc::Rc};


type pointer = Option<Rc<RefCell< Node> > >;

#[derive(Debug)]
struct Double_Linklist{
    head: pointer,
    tail: pointer,
}

#[derive(Debug)]
struct Node{
    element: i32,
    next: pointer,
    prev: pointer,
}

impl Double_Linklist {
    fn new() -> Double_Linklist {
        Double_Linklist{
            head: None,
            tail: None,
        }
    }

    fn add(&mut self, element: i32) {
        let new_head = Node::new(element);

        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(new_head.clone());
                new_head.borrow_mut().next = Some(old_head.clone());
                self.head = Some(new_head);
            }
            _ => {
                self.tail = Some(new_head.clone());
                self.head = Some(new_head);
            }
        }
    }


    fn remove(&mut self) -> Option<i32> {
        if self.head.is_none(){
            None
        }else{
            let removed_val = self.head.as_ref().unwrap().borrow().element;
            // self.head.
            self.head.take().map(|old_head| match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    new_head.borrow_mut().prev = None;
                    self.head = Some(new_head);
                    self.head.clone()
                }
                None => {
                    self.tail = None;
                    println!("List is empty after removal");
                    None
                }
            });
            Some(removed_val)
        }
    }

    fn print(&self) {
        let mut traversal = self.head.clone();
        while !traversal.is_none() {
            println!("{}", traversal.as_ref().unwrap().borrow().element);
            traversal = traversal.unwrap().borrow().next.clone();
        }
    }
}

impl Node {
    fn new(element: i32) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            element: element,
            next: None,
            prev: None,
        }))
    }
}

fn main() {
    let mut list = Double_Linklist::new();
    list.add(5);
    list.add(7);
    list.add(10);
    list.add(15);
    list.add(20);
    list.remove();
    list.print();
}
