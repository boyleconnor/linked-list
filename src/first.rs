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

    pub fn pop(&mut self) -> Option<T> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::More(boxed_node) => {
                self.head = boxed_node.next;
                Some(boxed_node.element)
            }
            Link::Empty => None
        }
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

#[cfg(test)]
mod test {
    use super::List;
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
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}
