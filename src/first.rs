use std::mem;


#[derive(Debug)]
pub struct List<T> {
    head: Link<T>
}

impl<T> List<T> {
    pub fn push(&mut self, value: T) {
        let new_node = Box::new(Node {
            element: value,
            next: mem::replace(&mut self.head, Link::Empty)
        });
        self.head = Link::More(new_node);
    }
    pub fn new() -> Self {
        List { head: Link::Empty }
    }
}

#[derive(Debug)]
struct Node<T> {
    element: T,
    next: Link<T>
}

#[derive(Debug)]
enum Link<T> {
    More(Box<Node<T>>),
    Empty
}

#[test]
fn test_list_manual() {
    let list: List<i32> = List {
        head: Link::More(Box::new(Node {
            element: 1,
            next: Link::More(Box::new(Node {
                element: 2,
                next: Link::Empty
            }))
        }))
    };
    println!("{:?}", list);
}

#[test]
fn test_list_push() {
    let mut list: List<i32> = List { head: Link::Empty };
    list.push(12);
    list.push(8);
    println!("{:?}", list);
}
