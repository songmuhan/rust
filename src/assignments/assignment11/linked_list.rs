//! Singly linked list.
//!
//! Consult <https://doc.rust-lang.org/book/ch15-01-box.html>.

use std::fmt::Debug;

/// Node of the list.
#[derive(Debug)]
pub struct Node<T: Debug> {
    /// Value of current node.
    pub value: T,

    /// Pointer to the next node. If it is `None`, there is no next node.
    pub next: Option<Box<Node<T>>>,
}

impl<T: Debug> Node<T> {
    /// Creates a new node.
    pub fn new(value: T) -> Self {
        Self { value, next: None }
    }
}

/// A singly-linked list.
#[derive(Debug)]
pub struct SinglyLinkedList<T: Debug> {
    /// Head node of the list. If it is `None`, the list is empty.
    head: Option<Node<T>>,
}

impl<T: Debug> Default for SinglyLinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Debug> SinglyLinkedList<T> {
    /// Creates a new list.
    pub fn new() -> Self {
        Self { head: None }
    }

    /// Adds the given node to the front of the list.
    pub fn push_front(&mut self, value: T) {
        let mut node = Node::new(value);
        node.next = self.head.take().map(Box::new);
        self.head = Some(node);
    }

    /// Adds the given node to the back of the list.
    pub fn push_back(&mut self, value: T) {
        match self.head {
            None => self.head = Some(Node::new(value)),
            Some(ref mut node) => {
                let mut current = &mut node.next;
                loop {
                    match current {
                        None => {
                            *current = Some(Box::new(Node::new(value)));
                            break;
                        }
                        Some(boxed_next) => {
                            current = &mut boxed_next.next;
                        }
                    }
                }
            }
        }
    }

    /// Removes and returns the node at the front of the list.
    pub fn pop_front(&mut self) -> Option<T> {
        let head = self.head.take();
        match head {
            None => None,
            Some(node) => {
                self.head = node.next.map(|v| *v);
                Some(node.value)
            }
        }
    }

    /// Removes and returns the node at the back of the list.
    pub fn pop_back(&mut self) -> Option<T> {
        match &mut self.head {
            None => None,
            Some(ref mut head) => {
                if head.next.is_none() {
                    self.head.take().map(|node| node.value)
                } else {
                    let mut cur = &mut *head;
                    loop {
                        if cur.next.as_ref().is_some() && cur.next.as_ref().unwrap().next.is_none()
                        {
                            // .... cur -> cur.next -> NULL
                            return cur.next.take().map(|node| node.value);
                        } else {
                            cur = &mut *cur.next.as_mut().unwrap();
                        }
                    }
                }
            }
        }
    }

    /// Create a new list from the given vector `vec`.p
    pub fn from_vec(vec: Vec<T>) -> Self {
        let mut list = Self::new();
        for item in vec.into_iter().rev() {
            list.push_front(item);
        }
        list
    }

    /// Convert the current list into a vector.
    pub fn into_vec(self) -> Vec<T> {
        let mut v = Vec::new();
        let mut list = self;
        loop {
            match list.pop_front() {
                None => return v,
                Some(value) => {
                    v.push(value);
                }
            }
        }
    }

    /// Return the length (i.e., number of nodes) of the list.
    pub fn length(&self) -> usize {
        match &self.head {
            None => 0,
            Some(head) => {
                match &head.next {
                    None => 1,
                    Some(node) => {
                        // head -> node -> ..
                        let mut count = 2;
                        let mut current = &node.next;
                        while let Some(node) = current {
                            count += 1;
                            current = &node.next;
                        }
                        count
                    }
                }
            }
        }
    }

    /// Apply function `f` on every element of the list.
    ///
    /// # Examples
    ///
    /// `self`: `[1, 2]`, `f`: `|x| x + 1` ==> `[2, 3]`
    pub fn map<F: Fn(T) -> T>(self, f: F) -> Self {
        Self::from_vec(self.into_vec().into_iter().map(f).collect::<Vec<_>>())
    }

    /// Apply given function `f` for each adjacent pair of elements in the vec.
    /// If `self.length() < 2`, do nothing.
    ///
    /// # Examples
    ///
    /// `self`: `[1, 2, 3, 4]`, `f`: `|x, y| x + y`
    /// // each adjacent pair of elements: `(1, 2)`, `(2, 3)`, `(3, 4)`
    /// // apply `f` to each pair: `f(1, 2) == 3`, `f(2, 3) == 5`, `f(3, 4) == 7`
    /// ==> `[3, 5, 7]`
    pub fn pair_map<F: Fn(T, T) -> T>(self, f: F) -> Self
    where
        T: Clone,
    {
        if self.length() < 2 {
            return self;
        }

        let mut iter = self.into_vec().into_iter();
        let mut result = Vec::new();

        let mut current = iter.next().unwrap(); // Safe to unwrap because length >= 2
        for next in iter {
            result.push(f(current.clone(), next.clone()));
            current = next;
        }

        Self::from_vec(result)
    }
}

// A list of lists.
impl<T: Debug> SinglyLinkedList<SinglyLinkedList<T>> {
    /// Flatten the list of lists into a single list.
    ///
    /// # Examples
    /// `self`: `[[1, 2, 3], [4, 5, 6], [7, 8]]`
    /// ==> `[1, 2, 3, 4, 5, 6, 7, 8]`
    pub fn flatten(self) -> SinglyLinkedList<T> {
        SinglyLinkedList::from_vec(
            self.into_vec()
                .into_iter()
                .flat_map(SinglyLinkedList::into_vec)
                .collect::<Vec<_>>(),
        )
    }
}
