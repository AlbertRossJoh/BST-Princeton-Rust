pub mod fundamentals;
pub mod searching;

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

    use super::searching::BST;
    #[test]
    fn test_fundamentals_BST() {
        let mut bst: BST::BST<i32,i32> = BST::BST::new();
        bst.put(5, 15);
        bst.put(4, 23);
        bst.put(10, 30);
        bst.put(12321321, 2330);
        bst.put(-1, 34);
        bst.put(2, 48905);

    //    bst.delete_max();
        assert_eq!(bst.delete_max().unwrap(), 2330);
        assert_eq!(bst.delete_min().unwrap(), 34);
        // assert!( == );
    }
}
