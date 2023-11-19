use std::rc::Rc;
use std::cell::{Ref, RefCell};
use std::fmt::Debug;

#[derive(Debug)]
pub struct List<T: Debug> {
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

impl<T: Debug> List<T> {
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

    pub fn pop_head(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            match old_head.borrow_mut().next.as_mut() {
                Some(next_node) => {next_node.borrow_mut().previous.take();}
                None => {self.tail.take();}
            };
            let Node { next, previous: _, element } = Rc::try_unwrap(old_head).unwrap().into_inner();
            self.head = next;
            element
        })
    }

    pub fn peek_head(&self) -> Option<Ref<T>> {
        self.head.as_ref().map(|rc_node| {
            Ref::map(rc_node.borrow(), |node| &node.element)
        })
    }
}

impl<T: Debug> Drop for List<T> {
    fn drop(&mut self) {
        while self.pop_head().is_some() {}
    }
}

#[cfg(test)]
mod test {
    use std::ops::Deref;
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop_head(), None);

        // Populate list
        list.push_head(1);
        list.push_head(2);
        list.push_head(3);

        // Check normal removal
        assert_eq!(list.pop_head(), Some(3));
        assert_eq!(list.pop_head(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push_head(4);
        list.push_head(5);

        // Check normal removal
        assert_eq!(list.pop_head(), Some(5));
        assert_eq!(list.pop_head(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop_head(), Some(1));
        assert_eq!(list.pop_head(), None);
    }

    #[test]
    fn peek() {
        let mut list = List::new();
        assert!(list.peek_head().is_none());

        // The tutorial said to use `&*` instead of `.deref()`. I think `.deref()` looks nicer.
        list.push_head(5);
        assert_eq!(list.peek_head().unwrap().deref(), &5);

        list.push_head(10);
        assert_eq!(list.peek_head().unwrap().deref(), &10);

        list.pop_head();
        assert_eq!(list.peek_head().unwrap().deref(), &5);
    }
}
