use std::marker::PhantomData;
use std::ptr::NonNull;

pub struct LinkedList<T> {
    head: Link<T>,
    tail: Link<T>,
    length: usize,
    _boo: PhantomData<T>
}

type Link<T> = Option<NonNull<Node<T>>>;

struct Node<T> {
    previous: Link<T>,
    element: T,
    next: Link<T>
}

impl<T> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList { head: None, tail: None, length: 0, _boo: PhantomData }
    }

    pub fn push_front(&mut self, element: T) {
        unsafe {
            let new_node = NonNull::new_unchecked(Box::into_raw(Box::new(Node {
                previous: None,
                element,
                next: None
            })));
            if let Some(old_node) = self.head {
                debug_assert!(self.length >= 1);
                (*new_node.as_ptr()).next = Some(old_node);
                (*old_node.as_ptr()).previous = Some(new_node);
            } else {
                debug_assert!(self.head.is_none());
                debug_assert!(self.tail.is_none());
                debug_assert_eq!(self.length, 0);
                self.tail = Some(new_node);
            }

            self.head = Some(new_node);
            self.length += 1;
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.map(|tail_node| {
            // FIXME: does dereferencing the tail ensure it gets dropped?
            unsafe {
                let boxed_node = Box::from_raw(tail_node.as_ptr());
                let new_tail = boxed_node.previous;
                if let Some(new_tail_node) = new_tail {
                    (*new_tail_node.as_ptr()).next = None;
                } else {
                    debug_assert_eq!(self.length, 1);
                }
                self.tail = new_tail;
                self.length -= 1;
                (*boxed_node).element
            }
        })
    }
}

#[cfg(test)]
mod test {
    use super::LinkedList;

    #[test]
    fn basics() {
        let mut list = LinkedList::new();
        list.push_front(3);
        list.push_front(6);
        list.push_front(2);
        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_back(), Some(6));
        list.push_front(4);
        assert_eq!(list.pop_back(), Some(2));
        assert_eq!(list.pop_back(), Some(4));
        assert_eq!(list.pop_back(), None);
    }
}
