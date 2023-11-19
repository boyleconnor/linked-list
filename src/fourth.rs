use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
pub struct List<T> {
    head: Link<T>,
    tail: Link<T>
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Debug)]
pub struct Node<T> {
    previous: Link<T>,
    next: Link<T>,
    element: T
}

impl<T> List<T> {
    pub fn new() -> List<T> {
        List { head: None, tail: None }
    }

    // I implemented this differently from the tutorial
    pub fn push_head(&mut self, element: T) {
        let mut prev_head = self.head.clone();
        self.head = Some(Rc::from(RefCell::from(Node {
            next: prev_head.clone(),
            previous: None,
            element
        })));
        prev_head.map(|rc_node| {
            rc_node.borrow_mut().previous = self.head.clone();
        });
        if self.tail.is_none() {
            self.tail = self.head.clone();
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn push_head() {
        let mut list = List::new();
        list.push_head(4);
        list.push_head(3);
        list.push_head(10);
        assert_eq!(list.head.clone().unwrap().borrow().element, 10);
        assert_eq!(list.head.clone().unwrap().borrow().next.clone().unwrap().borrow().element, 3);
        assert_eq!(list.head.clone().unwrap().borrow().next.clone().unwrap().borrow().next.clone().unwrap().borrow().element, 4);
        assert_eq!(list.tail.clone().unwrap().borrow().element, 4);
        assert_eq!(list.tail.unwrap().borrow().previous.clone().unwrap().borrow().element, 3)
    }
}
