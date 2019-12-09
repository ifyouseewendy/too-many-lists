use std::fmt;
// Structure;
//
#[derive(Debug)]
pub struct List {
    head: Link,
}

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.head)
    }
}

#[derive(Debug)]
enum Link {
    Empty,
    More(Box<Node>),
}

impl fmt::Display for Link {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Link::Empty => write!(f, "()"),
            Link::More(ref node) => write!(f, "{} -> {}", node.elem, format!("{}", &node.next)),
        }
    }
}

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
        let mut node = Node { elem: 3, next: Link::Empty };
        node         = Node { elem: 2, next: Link::More(Box::new(node)) };
        node         = Node { elem: 1, next: Link::More(Box::new(node)) };

        let list = List { head: Link::More(Box::new(node)) };

        assert_eq!("1 -> 2 -> 3 -> ()", format!("{}", list));
    }
}
