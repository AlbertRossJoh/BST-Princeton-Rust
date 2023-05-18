pub mod fundamentals;

#[cfg(test)]
mod tests {
    use super::fundamentals;
    #[test]
    fn test_fundamentals_stack() {
        let mut stack: fundamentals::stack::Stack<u8> = fundamentals::stack::Stack::new();
        stack.push(30);
        stack.push(40);

        assert!(stack.size() == 2);
    }
}
