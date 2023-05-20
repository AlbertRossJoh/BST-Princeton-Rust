pub mod fundamentals;
pub mod searching;
pub mod sorting;

#[cfg(test)]
mod tests {
    use crate::sorting::insertion::{sort, self};

    use super::fundamentals;
    use fundamentals::queue::Queue;
    #[test]
    fn test_fundamentals_stack() {
        let mut stack: fundamentals::stack::Stack<u8> = fundamentals::stack::Stack::new();
        stack.push(30);
        stack.push(40);

        assert!(stack.size() == 2);
    }

    use super::searching;
    use searching::BST::BST;
    
    #[test]
    fn test_string_result_queue() {
        let mut haps: BST<i32,i32> = BST::new();
        let mut queue: Queue<String> = Queue::new();
        queue.enqueue("Bob".to_string());
        
        let dequeued = queue.dequeue();
        assert!(Some("Bob".to_string()) == dequeued);
    }

    // use super::sorting::insertion;
    #[test]
    fn test_list_sorting() {
        let mut list = vec![9,8,7,6,5,4,3,2,1,0];
        // list.reverse();
        println!("{:?}",insertion::sort_slice(&mut list, 3,8));
        // insertion::sort(&mut list);
        assert_eq!(list,vec![9,8,7,2,3,4,5,6,1,0]);
    }
}
