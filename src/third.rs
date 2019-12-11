use std::rc::Rc;

// list1 = A -> B -> C -> D
// list1.head() = A
// list2 = list1.tail() = B -> C -> D
// list3 = list2.append(X) = X -> B -> C -> D
//
// list1 -> A ---+
//               |
//               v
// list2 ------> B -> C -> D
//               ^
//               |
// list3 -> X ---+

pub struct List<T> {
    head: Link<T>,
}
type Link<T> = Option<Rc<Node<T>>>;
struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn append(&self, elem: T) -> Self {
        Self {
            head: Some(Rc::new(Node {
                elem,
                next: self.head.clone(),
            }))
        }
    }

    pub fn tail(&self) -> List<T> {
        Self {
            head: self.head.as_ref().and_then(|node| node.next.clone()),
        }
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }
}

// Iter
impl<T> List<T> {
    pub fn iter(&self) -> Iter<T> {
        Iter { next: self.head.as_ref().map(|node| &**node) }
    }
}
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_ref().map(|node| &**node);
            &node.elem
        })
    }
}

// Drop
//
// Rc only give shared access, we could only try_unwrap
impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_node = self.head.take();

        while let Some(node) = cur_node {
            if let Ok(mut node) = Rc::try_unwrap(node) {
                cur_node = node.next.take();
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
    fn basics() {
        let list = List::new();
        assert_eq!(list.head(), None);

        let list = list.append(1).append(2).append(3);
        assert_eq!(list.head(), Some(&3));

        let list = list.tail();
        assert_eq!(list.head(), Some(&2));
        let list = list.tail();
        assert_eq!(list.head(), Some(&1));
        let list = list.tail();
        assert_eq!(list.head(), None);

        // Make sure empty tail works
        let list = list.tail();
        assert_eq!(list.head(), None);
    }

    #[test]
    fn iter() {
        let list = List::new().append(1).append(2).append(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
    }
}

