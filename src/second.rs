use std::fmt;

#[derive(Debug)]
pub struct List {
    head: Link,
}

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.head)
    }
}

impl Drop for List {
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

impl List {
    fn new() -> Self {
        Self { head: None }
    }

    fn push(&mut self, elem: i32) {
        let new_node = Node {
            elem,
            next: self.head.take(),
        };
        self.head = Some(Box::new(new_node));
    }

    fn pop(&mut self) -> Option<i32> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }
}

type Link = Option<Box<Node>>;

// impl fmt::Display for Link {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             None => write!(f, "()"),
//             Some(ref node) => write!(f, "{} -> {}", node.elem, format!("{}", &node.next)),
//         }
//     }
// }

#[derive(Debug)]
struct Node {
    elem: i32,
    next: Link,
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

        assert_eq!("Some(Node { elem: 1, next: Some(Node { elem: 2, next: Some(Node { elem: 3, next: None }) }) })", format!("{}", list));
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