use std::iter::FromIterator;

#[derive(Debug, PartialEq, Clone)]
pub struct SimpleLinkedList<T> {
    head: Link<T>,
}

#[derive(Debug, PartialEq)]
pub struct Node<T> {
    element: T,
    next: Link<T>,
}

impl<T> Clone for Node<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            element: self.element.clone(),
            next: self.next.clone(),
        }
    }
}

type Link<T> = Option<Box<Node<T>>>;

impl<T> Node<T> {
    pub fn new(element: T) -> Self {
        Self {
            element,
            next: None,
        }
    }

    pub fn with_next(element: T, next: Link<T>) -> Self {
        Self { element, next }
    }

    #[allow(dead_code)]
    fn next(mut self, value: Link<T>) -> Self {
        self.next = value;
        self
    }

    #[allow(dead_code)]
    fn has_next(&self) -> bool {
        self.next.is_some()
    }
}

impl<T> AsRef<T> for Node<T> {
    fn as_ref(&self) -> &T {
        &self.element
    }
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        match self.head {
            None => 0,
            Some(ref head) => {
                let mut count = 1;
                let mut current = head;

                while let Some(ref node) = current.next {
                    current = node;
                    count += 1;
                }

                count
            }
        }
    }

    pub fn push(&mut self, element: T) {
        let new_node = Box::new(Node::with_next(element, self.head.take()));
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.element
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.element)
    }
}

impl<T> SimpleLinkedList<T>
where
    T: Clone,
{
    pub fn rev(self) -> SimpleLinkedList<T> {
        match self.head {
            None => Self::new(),
            Some(ref head) => {
                let mut current = head;
                let mut v = vec![];

                while let Some(ref node) = current.next {
                    v.push(current.element.clone());
                    current = node;
                }

                v.push(current.element.clone());
                Self::from(v)
            }
        }
    }
}

impl<T> Drop for SimpleLinkedList<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = Self::new();

        for i in iter {
            list.push(i);
        }

        list
    }
}

impl<T> Iterator for SimpleLinkedList<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.pop()
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T> From<Vec<T>> for SimpleLinkedList<T> {
    fn from(values: Vec<T>) -> Self {
        Self::from_iter(values.into_iter())
    }
}

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        let mut v = Vec::from_iter(self.into_iter());
        v.reverse();
        v
    }
}
