pub mod fundamentals;

#[cfg(test)]
mod tests {
    use super::fundamentals;
    use fundamentals::queue::Queue;
    #[test]
    fn test_fundamentals_stack() {
        let mut stack: fundamentals::stack::Stack<u8> = fundamentals::stack::Stack::new();
        stack.push(30);
        stack.push(40);

        assert!(stack.size() == 2);
    }

    #[test]
    fn test_string_result_queue() {
        let mut queue: Queue<String> = Queue::new();
        queue.enqueue("Bob".to_string());

        let dequeued = queue.dequeue();
        assert!(Some("Bob".to_string()) == dequeued);
    }
}
