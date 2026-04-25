use std::mem::swap;

/// An efficient FIFO queue.
pub struct Queue<T> {
    in_: Vec<T>,
    out_: Vec<T>,
}

impl<T> Queue<T> {
    /// Creates an empty `Queue`.
    pub fn new() -> Self {
        Self {
            in_: vec![],
            out_: vec![],
        }
    }

    /// Creates a `Queue` from the contents of `in_`.
    pub fn from_vec(in_: Vec<T>) -> Self {
        Self { in_, out_: vec![] }
    }

    /// Number of items in the queue.
    pub fn count(&self) -> usize {
        self.in_.len() + self.out_.len()
    }

    /// Iterates over the elements of the queue, 
    /// from front to back.
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.out_.iter().rev().chain(self.in_.iter())
    }

    /// Is the queue empty?
    pub fn is_empty(&self) -> bool {
        self.count() == 0
    }

    /// Clears the contents of the queue.
    pub fn clear(&mut self) {
        self.in_.clear();
        self.out_.clear();
    }

    /// Adds an item to the end of the queue.
    pub fn enqueue(&mut self, value: T) {
        self.in_.push(value)
    }

    /// Takes an item from the front of the queue
    /// (if it has any).
    pub fn dequeue(&mut self) -> Option<T> {
        if self.out_.is_empty() {
            self.in_.reverse();
            swap(&mut self.out_, &mut self.in_);
        }

        self.out_.pop()
    }

    /// Gets a reference to the next item
    /// in the queue (if it has any).
    pub fn peek(&self) -> Option<&T> {
        self.out_.last().or(self.in_.first())
    }
}
