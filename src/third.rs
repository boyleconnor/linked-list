use std::rc::Rc;

pub struct List<T> {
    head: Link<T>
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
}

type Link<T> = Option<Rc<Node<T>>>;

pub struct Node<T> {
    element: T,
    next: Link<T>
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
}


