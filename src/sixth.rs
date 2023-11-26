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
            let new_head = NonNull::new_unchecked(Box::into_raw(Box::new(Node {
                previous: None,
                element,
                next: None
            })));
            if let Some(old_head) = self.head {
                (*new_head.as_ptr()).next = Some(old_head);
                (*old_head.as_ptr()).previous = Some(new_head);
            } else {
                self.tail = Some(new_head);
            }

            self.head = Some(new_head);
            self.length += 1;
        }
    }

    pub fn push_back(&mut self, element: T) {
        unsafe {
            let new_tail = NonNull::new_unchecked(Box::into_raw(Box::new(Node {
                previous: None,
                element,
                next: None
            })));
            if let Some(old_tail) = self.tail {
                (*new_tail.as_ptr()).previous = Some(old_tail);
                (*old_tail.as_ptr()).next = Some(new_tail);
            } else {
                self.head = Some(new_tail);
            }

            self.tail = Some(new_tail);
            self.length += 1;
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.map(|head_node| {
            unsafe {
                let old_head = Box::from_raw(head_node.as_ptr());
                self.head = old_head.next.map(|new_head| {
                    (*new_head.as_ptr()).previous = None;
                    new_head
                }).or_else(|| {
                    self.tail = None;
                    None
                });

                self.length -= 1;
                old_head.element
            }
        })
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.map(|tail_node| {
            unsafe {
                let old_tail = Box::from_raw(tail_node.as_ptr());
                self.tail = old_tail.previous.map(|new_tail| {
                    (*new_tail.as_ptr()).next = None;
                    new_tail
                }).or_else(|| {
                    self.head = None;
                    None
                });

                self.length -= 1;
                old_tail.element
            }
        })
    }

    pub fn front(&self) -> Option<&T> {
        self.head.map(|head_node| {
            unsafe { &(*head_node.as_ptr()).element }
        })
    }

    pub fn front_mut(&mut self) -> Option<&mut T> {
        self.head.map(|head_node| {
            unsafe { &mut (*head_node.as_ptr()).element }
        })
    }

    pub fn back(&self) -> Option<&T> {
        self.tail.map(|tail_node| {
            unsafe { &(*tail_node.as_ptr()).element }
        })
    }

    pub fn back_mut(&mut self) -> Option<&mut T> {
        self.tail.map(|tail_node| {
            unsafe { &mut (*tail_node.as_ptr()).element }
        })
    }

    pub fn len(&self) -> usize {
        self.length
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop_front() {};
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

    #[test]
    fn test_peek() {
        let mut list = LinkedList::new();
        assert_eq!(list.front(), None);
        assert_eq!(list.front_mut(), None);
        assert_eq!(list.back(), None);
        assert_eq!(list.back_mut(), None);

        list.push_front(3);
        list.push_front(2);
        list.push_back(4);
        list.push_back(5);
        list.push_front(1);
        list.push_back(6);

        assert_eq!(list.front(), Some(&1));
        assert_eq!(list.front_mut(), Some(&mut 1));
        assert_eq!(list.back(), Some(&6));
        assert_eq!(list.back_mut(), Some(&mut 6));

        *list.front_mut().unwrap() = -1;
        *list.back_mut().unwrap() = -6;

        assert_eq!(list.front(), Some(&-1));
        assert_eq!(list.front_mut(), Some(&mut -1));
        assert_eq!(list.back(), Some(&-6));
        assert_eq!(list.back_mut(), Some(&mut -6));

        list.push_front(0);
        list.push_back(0);

        assert_eq!(list.front(), Some(&0));
        assert_eq!(list.back(), Some(&0));
    }

    #[test]
    fn test_basic_front() {
        let mut list = LinkedList::new();

        // Try to break an empty list
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.len(), 0);

        // Try to break a one item list
        list.push_front(10);
        assert_eq!(list.len(), 1);
        assert_eq!(list.pop_front(), Some(10));
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.len(), 0);

        // Mess around
        list.push_front(10);
        assert_eq!(list.len(), 1);
        list.push_front(20);
        assert_eq!(list.len(), 2);
        list.push_front(30);
        assert_eq!(list.len(), 3);
        assert_eq!(list.pop_front(), Some(30));
        assert_eq!(list.len(), 2);
        list.push_front(40);
        assert_eq!(list.len(), 3);
        assert_eq!(list.pop_front(), Some(40));
        assert_eq!(list.len(), 2);
        assert_eq!(list.pop_front(), Some(20));
        assert_eq!(list.len(), 1);
        assert_eq!(list.pop_front(), Some(10));
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.len(), 0);
    }
}
