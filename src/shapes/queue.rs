struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

pub struct Queue<T> {
    lenght: usize,
    tail: Option<Box<Node<T>>>,
    head: Option<*mut Node<T>>,
}

impl<T> Queue<T> {
    fn enqueue(&mut self, item: T) {
        let node = Box::new(Node {
            value: item,
            next: None,
        });
        let raw = Box::into_raw(node);
            self.lenght += 1;
        if let Some(mut tail) = self.tail.take() {
            self tail.
            return;

        } else {
            unsafe {
                self.tail = Some(Box::from_raw(raw));
                self.head = Some(raw);
                return;
            }
        }
    }

    fn peek(&self) -> Option<T>
    where
        T: Clone,
    {
        return self.head.as_ref().map(|node| node.value.clone());
    }
    fn deque(&mut self) -> Option<T> {
        if let Some(mut head) = self.head.take() {
            self.lenght -= 1;
            self.head = head.next.take();
            head.next = None;
            return Some(head.value);
        } else {
            return None;
        }
    }
}

impl Default for Queue<usize> {
    fn default() -> Self {
        Queue {
            lenght: 0,
            head: None,
            tail: None,
        }
    }
}
