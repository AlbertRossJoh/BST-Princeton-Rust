#[derive(Debug)]
/// The fundamentals stack module represents a last-in-first-out (LIFO) stack
/// of generic items. It supports <em>push</em> and <em>pop</em> operations,
/// along with methods for peeking the top item, getting the size, and testing 
/// if the stack is empty.
/// 
/// Push takes constant amortized running time. Worst-case *O*(n) if a resize occurs. 
/// Pop, peek, size, and is_empty all take constant worst-case running time: *O*(1). 
///
/// This implementation uses a Vec\<T\>, to hold its elements.
/// 
/// Author: cave
/// 
/// # Examples
///
/// ```
/// use itualgs_rs::fundamentals::stack::Stack;
///
/// let mut stack = Stack::new();
/// 
/// // A stack is exactly like a book stack!
/// stack.push("The Great Gatsby");
/// // TOP
/// // The Great Gatsby
/// // BOTTOM
/// stack.push("Alice in Wonderland");
/// // TOP
/// // Alice in Wonderland
/// // The Great Gatsby
/// // BOTTOM
/// 
/// stack.push("The Trial");
/// // TOP
/// // The Trial
/// // Alice in Wonderland
/// // The Great Gatsby
/// // BOTTOM
/// 
/// stack.pop();
/// // TOP
/// //                      ---> The Trial
/// // Alice in Wonderland
/// // The Great Gatsby
/// // BOTTOM
/// 
/// let mut alice_in_wonderland = stack.peek().unwrap().to_string();
/// // TOP
/// // Alice in Wonderland <---
/// // The Great Gatsby
/// // BOTTOM
///
/// assert!(alice_in_wonderland == "Alice in Wonderland")
/// ```
pub struct Stack<T> {
    elements: Vec<T>,
}


impl<T> Stack<T> {
    /// Create a new last-in-first-out (LIFO) stack of generic items. 
    pub fn new() -> Stack<T> {
        Stack {
            elements: Vec::<T>::new(),
        }
    }

    /// Push an element to the stack.
    pub fn push(&mut self, val: T) {
        self.elements.push(val);
    }

    /// Pop an element from the stack.
    pub fn pop(&mut self) -> Option<T> {
        self.elements.pop()
    }

    /// Peek at the top element on the stack.
    pub fn peek(&self) -> Option<&T> {
        self.elements.last()
    }
    
    /// Return the size of the stack (how many elements have been placed).
    pub fn size(&self) -> usize {
        self.elements.len()
    }

    /// Check if the stack is empty.
    pub fn is_empty(&self) -> bool {
        self.elements.len() == 0
    }
}

impl<T> Default for Stack<T> {
    fn default() -> Stack<T> {
        Stack::new()
    }
}

#[cfg(test)]
mod tests {
    use super::Stack;

    #[test]
    fn test_push() {
        let mut stack: Stack<u8> = Stack::new();
        stack.push(20);
        stack.push(30);
        assert!(stack.size() != 0);
    }

    #[test]
    fn test_pop() {
        let mut stack: Stack<u8> = Stack::new();
        stack.push(20);
        stack.push(30);
        let popped_value = stack.pop();
        assert_eq!(popped_value, Some(30));
    }

    #[test]
    fn test_peek() {
        let mut stack: Stack<u8> = Stack::new();
        stack.push(20);
        stack.push(30);

        assert_eq!(stack.peek(), Some(&30));
    }

    #[test]
    fn test_size() {
        let mut stack: Stack<u8> = Stack::new();
        stack.push(20);
        stack.push(30);

        assert_eq!(stack.size(), 2);
    }

    #[test]
    fn test_is_empty() {
        let mut stack: Stack<u8> = Stack::new();
        assert_eq!(stack.is_empty(), true);

        stack.push(20);

        assert_eq!(stack.is_empty(), false);
    }
}
