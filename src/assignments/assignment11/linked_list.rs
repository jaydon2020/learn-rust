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
        // 1. Create the new node
        let new_node = Box::new(Node {
            value,
            // 2. Steal the old head!
            // .take() replaces self.head with None and returns the old value.
            // If self.head was None, next becomes None.
            // If self.head was Some(node), next becomes Some(node).
            next: self.head.take().map(Box::new),
        });

        // 3. Update head to point to the new node
        self.head = Some(*new_node);
    }

    /// Adds the given node to the back of the list.
    pub fn push_back(&mut self, value: T) {
        // CASE 1: The list is empty.
        // We simply set head to the new node.
        if self.head.is_none() {
            self.head = Some(Node::new(value));
            return;
        }

        // CASE 2: The list is NOT empty.
        // We can safely unwrap head because we checked is_none() above.

        // Create the new node (boxed)
        let new_node = Box::new(Node::new(value));

        // Start cursor at head.next
        let mut cursor = &mut self.head.as_mut().unwrap().next;

        // Traverse to the end
        while let Some(node) = cursor {
            cursor = &mut node.next;
        }

        // Attach the new node at the end
        *cursor = Some(new_node);
    }

    /// Removes and returns the node at the front of the list.
    pub fn pop_front(&mut self) -> Option<T> {
        // 1. Take the node out of head.
        // If head is None, return None immediately.
        let node = self.head.take()?;

        // 2. Update head to point to the next node.
        // node.next is Option<Box<Node>>, but self.head needs Option<Node>.
        // We must unbox the next node using * (dereference).
        self.head = node.next.map(|boxed_node| *boxed_node);

        // 3. Return the value
        Some(node.value)
    }

    /// Removes and returns the node at the back of the list.
    pub fn pop_back(&mut self) -> Option<T> {
        // Case 1: The list is empty
        let _unused = self.head.as_ref()?;

        // Case 2: The list has only one element (head.next is None)
        // We can just reuse pop_front logic here!
        if self.head.as_ref().unwrap().next.is_none() {
            return self.pop_front();
        }

        // Case 3: The list has multiple elements.
        // We need to find the LAST `Some(Box<Node>)`.

        // Start our cursor at the first BOXED node (head.next)
        // usage of '?' is safe here because we checked is_none() above.
        let mut cursor = &mut self.head.as_mut()?.next;

        // Loop condition: Does the node inside the current cursor have a neighbor?
        // We stop when cursor points to the LAST node.
        while cursor.as_ref()?.next.is_some() {
            cursor = &mut cursor.as_mut()?.next;
        }

        // At this point, `cursor` is the Option<Box<Node>> that holds the last node.
        // We take it out (leaving None behind).
        let node = cursor.take()?;

        Some(node.value)
    }

    /// Create a new list from the given vector `vec`.
    pub fn from_vec(vec: Vec<T>) -> Self {
        let mut new_list = SinglyLinkedList::new();

        for v in vec {
            new_list.push_back(v);
        }

        new_list
    }

    /// Convert the current list into a vector.
    pub fn into_vec(self) -> Vec<T> {
        let mut ret = Vec::new();

        // 1. Take ownership of the head (consume the list)
        let mut current = self.head;

        // 2. Loop while `current` is Some(node)
        while let Some(node) = current {
            // 3. Move the value into the vector
            ret.push(node.value);

            // 4. Update current to point to the next node.
            // node.next is Option<Box<Node>>, so we map it to unbox it.
            current = node.next.map(|boxed_node| *boxed_node);
        }

        ret
    }

    /// Return the length (i.e., number of nodes) of the list.
    pub fn length(&self) -> usize {
        let mut len = 0;

        // 1. Initialize 'current' as Option<&Node>
        // self.head is Option<Node>. .as_ref() borrows the content.
        let mut current = self.head.as_ref();

        // 2. Loop while current is Some(&Node)
        while let Some(node) = current {
            len += 1;

            // 3. Update current to point to the next node.
            // node.next is Option<Box<Node>>.
            // .as_deref() borrows the box's content, returning Option<&Node>.
            // Now the types match perfectly!
            current = node.next.as_deref();
        }

        len
    }

    /// Apply function `f` on every element of the list.
    ///
    /// # Examples
    ///
    /// `self`: `[1, 2]`, `f`: `|x| x + 1` ==> `[2, 3]`
    pub fn map<F: Fn(T) -> T>(self, f: F) -> Self {
        let mut new_list = Self::new(); // Use Self::new() for flexibility

        // 1. Take ownership of the head (consume the list)
        let mut current = self.head;

        // 2. Loop while `current` is Some(node)
        while let Some(node) = current {
            // 3. Move the value into the vector
            let value = f(node.value);
            new_list.push_back(value);

            // 4. Update current to point to the next node.
            // node.next is Option<Box<Node>>, so we map it to unbox it.
            current = node.next.map(|boxed_node| *boxed_node);
        }

        new_list
    }

    /// Apply given function `f` for each adjacent pair of elements in the list.
    /// If `self.length() < 2`, do nothing.
    ///
    /// # Examples
    ///
    /// `self`: `[1, 2, 3, 4]`, `f`: `|x, y| x + y`
    /// // each adjacent pair of elements: `(1, 2)`, `(2, 3)`, `(3, 4)`
    /// // apply `f` to each pair: `f(1, 2) == 3`, `f(2, 3) == 5`, `f(3, 4) == 7`
    /// ==> `[3, 5, 7]`
    pub fn pair_map<F: Fn(T, T) -> T>(mut self, f: F) -> Self
    where
        T: Clone,
    {
        let mut new_list = Self::new();

        // 1. We need at least 2 items.
        // Let's pop the first item off to start our "window".
        // self.pop_front() returns Option<T>.
        // If it returns None, list is empty -> Loop won't run.
        let mut prev_val = match self.pop_front() {
            Some(v) => v,
            None => return new_list, // List was empty
        };

        // 2. Iterate through the REST of the list using pop_front()
        // We act like a sliding window: [prev_val, curr_val]
        while let Some(curr_val) = self.pop_front() {
            // Apply function to (prev, curr)
            // Note: We might need prev_val for the NEXT pair?
            // Actually no, pair_map (1,2), (2,3).
            // So 'curr_val' becomes the 'prev_val' for the next iteration.

            // We clone because 'f' might consume the values,
            // but we need 'curr_val' for the next loop iteration.
            let new_val = f(prev_val.clone(), curr_val.clone());
            new_list.push_back(new_val);

            // Shift the window: current becomes previous
            prev_val = curr_val;
        }

        new_list
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
        let mut flat = SinglyLinkedList::new();

        // 1. Take ownership of the head (consume the outer list)
        let mut current_outer = self.head;

        // 2. Loop through the outer list
        while let Some(outer_node) = current_outer {
            // outer_node.value is the Inner List.
            // We can consume it directly!
            let mut inner_list = outer_node.value;

            // 3. Drain the inner list directly into 'flat'
            // No vector allocation needed!
            while let Some(item) = inner_list.pop_front() {
                flat.push_back(item);
            }

            // 4. Move to next outer node
            // Note: We already moved 'value' out, but we can still move 'next' out.
            // Rust allows moving individual fields independent of each other.
            current_outer = outer_node.next.map(|boxed| *boxed);
        }

        flat
    }
}
