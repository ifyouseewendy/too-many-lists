use std::fmt;

pub struct List<T> {
    head: Link<T>,
}

impl<T: fmt::Debug> fmt::Debug for List<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.head)
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();

        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
            // boxed_node goes out of scope and gets dropped here;
            // but its Node's `next` field has been set to None
            // so no unbounded recursion occurs.
        }
    }
}

// into_iter()
//
// let mut iter = list.into_iter();
// assert_eq!(iter.next(), Some(3));
impl<T> List<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}
pub struct IntoIter<T>(List<T>);
impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.head.take().map(|node| {
            self.0.head = node.next;
            node.elem
        })
    }
}

// iter()
//
// let mut iter = list.iter();
// assert_eq!(iter.next(), Some(&3));
impl<T> List<T> {
    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        // turbofish ::<> could hint proper deref coercion
        Iter { next: self.head.as_ref().map::<&Node<T>, _>(|node| &node) }
        // Iter { next: self.head.as_ref().map(|node| &**node) }
    }
}
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_ref().map::<&Node<T>, _>(|node| &node);
            // self.next = node.next.as_ref().map(|node| &**node);
            &node.elem
        })
    }
}

// iter_mut()
//
// let mut iter = list.iter_mut();
// assert_eq!(iter.next(), Some(&mut 3));
impl<T> List<T> {
    pub fn iter_mut<'a>(&'a mut self) -> IterMut<'a, T> {
        // turbofish ::<> could hint proper deref coercion
        // IterMut { next: self.head.as_mut().map::<&mut Node<T>, _>(|node| &mut node) }
        IterMut { next: self.head.as_mut().map(|node| &mut(**node)) }
    }
}
pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}
impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            // self.next = node.next.as_mut().map::<&mut Node<T>, _>(|node| &mut node);
            self.next = node.next.as_mut().map(|node| &mut(**node));
            &mut node.elem
        })
    }
}

impl<T> List<T> {
    fn new() -> Self {
        Self { head: None }
    }

    fn push(&mut self, elem: T) {
        let new_node = Node {
            elem,
            next: self.head.take(),
        };
        self.head = Some(Box::new(new_node));
    }

    fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    // self.head.map(..) won't work as `map` takes `self` by value
    // as_ref converts &Option<T> to Option<&T>
    fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.elem
        })
    }
    // as_mut converts &mut Option<T> to Option<&mut T>
    fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| {
            &mut node.elem
        })
    }
}

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: Link<T>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut node = Node { elem: 3, next: None };
        node         = Node { elem: 2, next: Some(Box::new(node)) };
        node         = Node { elem: 1, next: Some(Box::new(node)) };

        let list = List { head: Some(Box::new(node)) };

        assert_eq!("Some(Node { elem: 1, next: Some(Node { elem: 2, next: Some(Node { elem: 3, next: None }) }) })", format!("{:?}", list));
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

        // Check peek
        assert_eq!(list.peek(), Some(&3));

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

    #[test]
    fn peek() {
        let mut list = List::new();
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);

        list.push(1); list.push(2); list.push(3);
        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));

        list.peek_mut().map(|value| {
            *value = 42
        });
        assert_eq!(list.peek(), Some(&42));
        assert_eq!(list.peek_mut(), Some(&mut 42));
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter_mut() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
        assert_eq!(iter.next(), None);
    }
}
