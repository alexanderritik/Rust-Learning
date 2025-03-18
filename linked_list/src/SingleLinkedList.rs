
type pointer = Option<Box<Node>>;

#[derive(Debug)]
struct Linklist{
    head: pointer
}

#[derive(Debug)]
struct Node{
    element: i32,
    next: pointer,
}

impl Linklist {
    fn new() -> Linklist {
        Linklist { head: None }
    }

    fn add(&mut self, element: i32) {
        // fn take<T>(dest: &mut T) -> T
        let previous_head = self.head.take();
        let new_head = Some(Box::new(Node {
            element: element,
            next: previous_head,
        }));
        self.head = new_head;
    }

    fn remove(&mut self) -> Option<i32> {
        match self.head.take() {
            Some(previous_head) => {
                self.head = previous_head.next;
                Some(previous_head.element)
            }
            None => None,
        }
    }

    fn print(&self) {
        let mut list_traversal = &self.head;
        while !list_traversal.is_none() {
            println!("{:?}", list_traversal.as_ref().unwrap().element);
            list_traversal = &list_traversal.as_ref().unwrap().next;
        }
    }
}

fn main() {
    let list0 = Node {
        element: 21,
        next: None,
    };

    let list1 = Node {
        element: 2,
        next: None,
    };

    let list2 = Node {
        element: 1,
        next: Some(Box::new(list1)),
    };
    println!("{:?} {:?}", list2, list0);

    let list = Linklist {
        head: Some(Box::new(Node {
            element: 100,
            next: Some(Box::new(Node {
                element: 200,
                next: None,
            })),
        })),
    };

    println!("{:?}", &list.head.unwrap().next.unwrap());


    let mut list = Linklist::new();
    list.add(5);
    list.add(7);
    list.add(10);
    list.add(15);
    list.add(20);
    list.remove();
    list.print();

}
