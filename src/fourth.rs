use std::rc::Rc;
use std::cell::{Ref, RefCell, RefMut};
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
        let prev_head = self.head.clone();
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

    pub fn push_tail(&mut self, element: T) {
        let prev_tail = self.tail.clone();
        self.tail = Some(Rc::from(RefCell::from(Node {
            next: None,
            previous: prev_tail.clone(),
            element
        })));
        prev_tail.map(|rc_node| {
            rc_node.borrow_mut().next = self.tail.clone();
        });
        if self.head.is_none() {
            self.head = self.tail.clone();
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

    pub fn pop_tail(&mut self) -> Option<T> {
        self.tail.take().map(|old_tail| {
            match old_tail.borrow_mut().previous.as_mut() {
                Some(previous_node) => {previous_node.borrow_mut().next.take();}
                None => {self.head.take();}
            };
            let Node { previous, next: _, element } = Rc::try_unwrap(old_tail).unwrap().into_inner();
            self.tail = previous;
            element
        })
    }

    pub fn peek_head(&self) -> Option<Ref<T>> {
        self.head.as_ref().map(|rc_node| {
            Ref::map(rc_node.borrow(), |node| &node.element)
        })
    }

    pub fn peek_tail(&self) -> Option<Ref<T>> {
        self.tail.as_ref().map(|rc_node| {
            Ref::map(rc_node.borrow(), |node| &node.element)
        })
    }

    pub fn peek_head_mut(&mut self) -> Option<RefMut<T>> {
        self.head.as_mut().map(|rc_node| {
            RefMut::map(rc_node.borrow_mut(), |node| &mut node.element)
        })
    }

    pub fn peek_tail_mut(&mut self) -> Option<RefMut<T>> {
        self.tail.as_mut().map(|rc_node| {
            RefMut::map(rc_node.borrow_mut(), |node| &mut node.element)
        })
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T: Debug> Drop for List<T> {
    fn drop(&mut self) {
        while self.pop_head().is_some() {}
    }
}

pub struct IntoIter<T: Debug>(List<T>);

impl<T: Debug> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_head()
    }
}

impl<T: Debug> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.pop_tail()
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
    fn queue() {
        let mut list = List::new();

        list.push_tail(10);
        list.push_tail(20);
        list.push_tail(15);
        list.push_tail(12);

        assert_eq!(list.pop_head(), Some(10));
        assert_eq!(list.pop_head(), Some(20));
        assert_eq!(list.pop_head(), Some(15));
        assert_eq!(list.pop_head(), Some(12));
        assert_eq!(list.pop_head(), None);
    }

    #[test]
    fn reverse_queue() {
        let mut list = List::new();

        list.push_head(10);
        list.push_head(20);
        list.push_head(15);
        list.push_head(12);

        assert_eq!(list.pop_tail(), Some(10));
        assert_eq!(list.pop_tail(), Some(20));
        assert_eq!(list.pop_tail(), Some(15));
        assert_eq!(list.pop_tail(), Some(12));
        assert_eq!(list.pop_tail(), None);
    }

    #[test]
    fn peek_mut() {
        let mut list = List::new();
        list.push_head(23);
        list.push_head(12);
        list.push_head(92);
        let head = list.peek_head_mut();
        *head.unwrap() = 10;

        assert_eq!(list.peek_head().unwrap().deref(), &10);
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

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push_head(1); list.push_head(2); list.push_head(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next_back(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.next(), None);
    }

}
