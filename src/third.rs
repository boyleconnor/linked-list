use std::rc::Rc;

pub struct List<T> {
    head: Link<T>
}

type Link<T> = Option<Rc<Node<T>>>;

pub struct Node<T> {
    element: T,
    next: Link<T>
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn prepend(&self, element: T) -> List<T> {
        List {
            head: Some(Rc::from(Node {
                element,
                next: self.head.clone() // FIXME: Shouldn't it be `next: self.head.as_ref().map(|node| Rc::clone(node))`
            }))
        }
    }

    pub fn tail(&self) -> List<T> {
        List {
            head: self.head.as_ref().and_then(
                |head_node| head_node.next.clone()
            )
        }
    }

    pub fn iter(&self) -> Iter<T> {
        Iter { next: self.head.as_deref() }
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.element
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut current_link = self.head.take();
        while let Some(rc_node) = current_link {
            if let Ok(mut node) = Rc::try_unwrap(rc_node) {
                current_link = node.next.take();
            } else {
                break;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;
    #[test]
    fn prepend() {
        let mut x = List::new();
        x = x.prepend(5);
        assert_eq!(x.head.clone().unwrap().element, 5);
        x = x.prepend(10);
        assert_eq!(x.head.clone().unwrap().element, 10);
        x = x.prepend(15);
        assert_eq!(x.head.clone().unwrap().element, 15);
    }

    #[test]
    fn tail() {
        let mut x = List::new();
        x = x.prepend(5);
        x = x.prepend(10);
        x = x.prepend(15);

        assert_eq!(x.head.clone().unwrap().element, 15);
        x = x.tail();
        assert_eq!(x.head.clone().unwrap().element, 10);
        x = x.tail();
        assert_eq!(x.head.clone().unwrap().element, 5);
        x = x.tail();
        assert!(x.head.is_none());
    }

    #[test]
    fn iter() {
        let list = List::new().prepend(3).prepend(5).prepend(11);
        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&11));
        assert_eq!(iter.next(), Some(&5));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }
}


