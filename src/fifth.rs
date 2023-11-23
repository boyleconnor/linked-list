use std::ptr;

pub struct List<T> {
    head: Link<T>,
    tail: Link<T>
}

type Link<T> = *mut Node<T>;

struct Node<T> {
    element: T,
    next: Link<T>
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: ptr::null_mut(), tail: ptr::null_mut() }
    }

    pub fn push(&mut self, element: T) {
        let new_tail = Box::into_raw(Box::new(Node {
            element,
            next: ptr::null_mut()
        }));

        if !self.tail.is_null() {
            unsafe { (*self.tail).next = new_tail; }
        } else {
            self.head = new_tail;
        }

        self.tail = new_tail;
    }

    pub fn pop(&mut self) -> Option<T> {
        if !self.head.is_null() {
            let old_head = self.head;
            unsafe { self.head = (*old_head).next; }

            if self.head.is_null() {
                self.tail = ptr::null_mut();
            }

            let boxed_old_head = unsafe {
                Box::from_raw(old_head)
            };

            Some(boxed_old_head.element)

        } else {
            None
        }
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop() {}
    }
}

pub struct IntoIter<T>(List<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        self.0.pop()
    }
}

#[cfg(test)]
mod test {
    use super::List;
    #[test]
    fn push() {
        let mut list = List::new();

        assert_eq!(list.pop(), None);
        list.push(3);
        list.push(2);
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), None);

        list.push(4);
        list.push(5);
        list.push(6);
        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.pop(), Some(5));
        list.push(7);
        assert_eq!(list.pop(), Some(6));
        assert_eq!(list.pop(), Some(7));
        assert_eq!(list.pop(), None);
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), None);

        // Check the exhaustion case fixed the pointer right
        list.push(6);
        list.push(7);

        // Check normal removal
        assert_eq!(list.pop(), Some(6));
        assert_eq!(list.pop(), Some(7));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push(10);
        list.push(12);
        list.push(23);
        let mut iter = list.into_iter();

        assert_eq!(iter.next(), Some(10));
        assert_eq!(iter.next(), Some(12));
        assert_eq!(iter.next(), Some(23));
        assert_eq!(iter.next(), None);
    }
}
