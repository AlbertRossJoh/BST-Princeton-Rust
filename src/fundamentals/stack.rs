#[derive(Debug)]

/// A simple stack implementation using vectors
pub struct Stack<T> {
    elements: Vec<T>,
}


impl<T> Stack<T> {
    /// Create a new stack with your prevered type
    pub fn new() -> Stack<T> {
        Stack {
            elements: Vec::<T>::new(),
        }
    }

    pub fn push(&mut self, val: T) {
        self.elements.push(val);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.elements.pop()
    }

    pub fn peek(&self) -> Option<&T> {
        self.elements.last()
    }
    
    pub fn size(&self) -> usize {
        self.elements.len()
    }

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
