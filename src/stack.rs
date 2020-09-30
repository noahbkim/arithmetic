struct Node<T> {
    value: T,
    next: Link<T>,
}

impl<T> Node<T> {
    pub fn new(value: T, next: Link<T>) -> Self {
        Self { value, next }
    }
}

type Link<T> = Option<Box<Node<T>>>;

pub struct Stack<T> {
    head: Link<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn push(&mut self, value: T) {
        let new_head = Box::new(Node::new(value, self.head.take()));
        self.head = Some(new_head);
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|boxed_node| &boxed_node.value)
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.head.take() {
            None => None,
            Some(node) => {
                self.head = node.next;
                Some(node.value)
            }
        }
    }

    pub fn reversed(mut self) -> Self {
        let mut result: Stack<T> = Stack::new();
        while let Some(value) = self.pop() {
            result.push(value);
        }
        result
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<T> Stack<T> {
    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.as_ref().map(|boxed_node| &**boxed_node),
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|boxed_node| {
            self.next = boxed_node.next.as_ref().map(|boxed_node| &**boxed_node);
            &boxed_node.value
        })
    }
}

impl<T> Drop for Stack<T> {
    fn drop(&mut self) {
        let mut cursor = self.head.take();
        while let Some(mut boxed_node) = cursor {
            cursor = boxed_node.next.take();
        }
    }
}
