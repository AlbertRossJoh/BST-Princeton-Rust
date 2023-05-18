use std::collections::LinkedList;

#[derive(Debug)]
/// The fundamentals queue module represents a first-in-first-out (FIFO) queue 
/// of generic items. It supports <em>enqueue</em> and <em>dequeue</em> operations,
/// along with methods for peeking the start of the queue, getting the size of the queue,
/// and testing if the queue is empty.
/// 
/// Every method should take constant worst-case running time: *O*(1).
///
/// This implementation uses a LinkedList\<T\>, to hold its elements.
/// 
/// Author: cave
/// 
/// # Examples
///
/// ```
/// use fundamentals::queue::Queue;
///
/// let mut queue = Queue::new();
/// 
/// // S indicated the start of the queue
/// // D indicates the door to the store
/// // S                 D
/// queue.enqueue("Bob");
/// assert_eq!(queue.size(), 1);
/// // Bob stands in queue
/// // S                B D
/// 
/// queue.enqueue("Alice");
/// assert_eq!(queue.size(), 2);
/// // Alice stands in queue
/// // S              A B D
/// 
/// queue.enqueue("Eve");
/// assert_eq!(queue.size(), 3);
/// // Eve stands in queue
/// // S            E A B D
/// 
/// queue.dequeue();
/// assert_eq!(queue.size(), 2);
/// // The doors open, one person goes in
/// // S              E A D -> B
/// ```
pub struct Queue<T> {
    elements: LinkedList<T>,
}

impl<T> Queue<T> {
    /// Create a new first-in-first-out (FIFO) queue of generic items. 
    pub fn new() -> Queue<T> {
        Queue { 
            elements: LinkedList::<T>::new()
        }
    }

    /// Adds an element to the back of the queue.
    pub fn enqueue(&mut self, val: T) {
        self.elements.push_back(val);
    }

    /// Removes an element from the front of the queue.
    pub fn dequeue(&mut self) -> Option<T> {
        self.elements.pop_front()
    }

    /// Returns the value at the front of the queue.
    pub fn peek(&self) -> Option<&T> {
        self.elements.front()
    }

    /// Checks if the queue is empty.
    pub fn is_empty(&self) -> bool {
        self.elements.front().is_none()
    } 

    /// Checks if the size is empty.
    pub fn size(&self) -> usize {
        self.elements.len()
    }
} 

impl<T> Default for Queue<T> {
    fn default() -> Queue<T> {
        Queue::new() 
    }
}


#[cfg(test)]
mod tests {
    use super::Queue;

    #[test]
    fn test_push() {
        let mut queue: Queue<u8> = Queue::new();
        queue.enqueue(20);
        queue.enqueue(30);
        assert!(queue.size() != 0);
    }

    #[test]
    fn test_pop() {
        let mut queue: Queue<u8> = Queue::new();
        queue.enqueue(20);
        queue.enqueue(30);
        let popped_value = queue.dequeue();
        assert_eq!(popped_value, Some(20));
    }

    #[test]
    fn test_peek() {
        let mut queue: Queue<u8> = Queue::new();
        queue.enqueue(20);
        queue.enqueue(30);

        assert_eq!(queue.peek(), Some(&20));
    }

    #[test]
    fn test_size() {
        let mut queue: Queue<u8> = Queue::new();
        queue.enqueue(20);
        queue.enqueue(30);

        assert_eq!(queue.size(), 2);
    }

    #[test]
    fn test_is_empty() {
        let mut queue: Queue<u8> = Queue::new();
        assert_eq!(queue.is_empty(), true);

        queue.enqueue(20);

        assert_eq!(queue.is_empty(), false);
    }
}
