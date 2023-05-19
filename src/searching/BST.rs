use std::{cmp::Ordering::*, fmt::Debug, mem};

#[derive(Clone)]
#[derive(Debug)]
struct Node<T:Ord, K> {
    key: T,
    val: K,
    size: i32,
    left: Option<Box<Node<T,K>>>,
    right: Option<Box<Node<T,K>>>,
}

/// The BST class represents an ordered symbol table og generic key pair values
/// It supports the operations `put`, `get`, `delete_max` and `delete_min`.
/// 
/// Every method should take constant worst-case running time: *O(N)*. With an average of *O(log N)*
/// /// Author: AlbertRossJoh
/// 
/// # Examples
///
/// ```
/// use searching::BST;;
///
/// let mut bst<i32,&str> = BST::new();
/// 
/// bst.put(24, "Ferris");
/// 
/// let mut another_bst: BST<u8,&str> = BST::new();
/// another_bst.put(4, "val4");
/// another_bst.put(10, "val10");
/// another_bst.put(2, "val2");
/// another_bst.put(3, "val3");
/// another_bst.put(11, "val11");
/// 
/// assert_eq!(another_bst.get(2).unwrap(), &"val2");
/// let val = another_bst.delete_min();
/// assert_eq!(another_bst.get(2).is_none(), true);
/// assert_eq!(another_bst.get(3).is_some(), true);
/// 
/// ```
pub struct BST<T:Ord, K> {root:Option<Box<Node<T, K>>>}


unsafe fn has_children<T:Ord,K:Ord>(node: *mut Option<Box<Node<T,K>>>) -> (bool, bool) 
where Box<Node<T,K>> : Clone
{
    let temp = node.read_volatile().clone();
    let clean_val = temp.unwrap_or(return (false,false));
    (clean_val.left.is_some(), clean_val.right.is_some())
}

impl<'a, T:Ord, K> BST<T,K> where T:Clone, K:Clone {
    pub fn new()->BST<T,K>{
        BST { root: None }
    }
    
    /// function for deleting the min node, returns true if successful false otherwise
    pub fn delete_min(&mut self) -> bool{
        // Get the root
        let mut root: *mut Option<Box<Node<T,K>>> = &mut self.root;
        // Make a clone of the root to make sure that we're not intrusive
        let mut parent: &mut Box<Node<T,K>> = &mut self.root.clone().unwrap();
        unsafe {
            while let Some(ref mut node) = *root {
                // if left is none we have reached our goal
                if node.left.is_none(){
                    // There might be a right value which we want to keep
                    if node.right.is_some() {
                        // get the right node of the current
                        let curr = node.right.clone().unwrap();
                        // set current node to none to remove value
                        *root = None;
                        // we should make sure that the parents left value gets a reference to the right value
                        parent.left.insert(curr);
                        return true;
                    }
                    // There is no right value, just remove
                    *root = None;
                    return true;
                }
                // go to next node
                root = &mut node.left;
                // get the parent of the next
                parent = node;
            }
        }
        false
    }

    /// function for deleting the max node, returns true if successful false otherwise
    pub fn delete_max(&mut self) -> bool{
        // Get the root
        let mut root: *mut Option<Box<Node<T,K>>> = &mut self.root;
        // Make a clone of the root to make sure that we're not intrusive
        let mut parent: &mut Box<Node<T,K>> = &mut self.root.clone().unwrap();
        unsafe {
            while let Some(ref mut node) = *root {
                // if right is none we have reached our goal
                if node.right.is_none(){
                    // There might be a left value which we want to keep
                    if node.left.is_some() {
                        // get the left node of the current
                        let curr = node.left.clone().unwrap();
                        // set current node to none to remove value
                        *root = None;
                        // we should make sure that the parents right value gets a reference to the right value
                        parent.right.insert(curr);
                        return true;
                    }
                    // There is no left value, just remove
                    *root = None;
                    return true;
                }
                // go to next node
                root = &mut node.right;
                // get the parent of the next
                parent = node;
            }
        }
        false
    }
    
    pub fn get_root(&self) -> Option<&K>{
        if let Some(e) = &self.root {
            return Some(&e.val);
        }
        None
    }
    
    pub fn get(&mut self, key: T) -> Option<&mut K>{
        if self.root.is_none(){
            return None;
        }
        let mut curr: &mut Option<Box<Node<T,K>>> = &mut self.root;
        while let Some(e) = curr{
            let cmp = &key.cmp(&e.key); 
            match cmp {
                Less => {
                    if e.left.is_none(){
                        return None;
                    }
                    curr =&mut e.left
                },
                Greater => {
                    if e.right.is_none(){
                        return None;
                    }
                    curr =&mut e.right
                },
                Equal => {
                    return Some(&mut e.val);
                },
            }
        }
        None
    }

    pub fn put(&mut self, key: T, val: K){
        let n = Node::new(key,val,0);
        if self.root.is_none(){
            self.root = Some(Box::new(n));
            return;
        }
        let mut curr: &mut Option<Box<Node<T,K>>> = &mut self.root;
        while let Some(e) = curr{
            let cmp = &n.key.cmp(&e.key); 
            match cmp {
                Less => {
                    if e.left.is_none() {
                        e.left = Some(Box::new(n));
                        return;
                    }
                    curr =&mut e.left
                },
                Greater => {
                    if e.right.is_none() {
                        e.right = Some(Box::new(n));
                        break;
                    }
                    curr =&mut e.right
                },
                Equal => {
                    e.val = n.val;
                    return;
                },
            }
        }
    }
}

impl<T:Ord, K> Node<T,K> where T:Clone, K:Clone {
    fn new(key:T, val:K, size: i32)->Node<T,K>{
        Node{
            key: key,
            val: val,
            size: size,
            left: None,
            right: None,
        }
    }

    fn clone(&self)->Node<T,K>{
        Node{
            key: self.key.clone(),
            val: self.val.clone(),
            size: self.size.clone(),
            left: self.right.clone(),
            right: self.left.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::BST;

    #[test]
    fn test_get() {
        let mut bst: BST<u8,&str> = BST::new();
        bst.put(4, "val4");
        bst.put(10, "val10");
        bst.put(2, "val2");
        bst.put(3, "val3");
        bst.put(11, "val11");
        
        let val = bst.get(4).unwrap();
        assert_eq!(val, &"val4");
        let val2 = bst.get(11).unwrap();
        assert_eq!(val2, &"val11");
    }

    #[test]
    fn test_put() {
        let mut bst: BST<u8,&str> = BST::new();
        bst.put(4, "val4");
        bst.put(10, "val10");
        bst.put(2, "val2");
        bst.put(3, "val3");
        bst.put(11, "val11");
        
        let val = bst.get_root().unwrap();
        assert_eq!(val, &"val4");
    }

    #[test]
    fn test_delete_min() {
        let mut bst: BST<u8,&str> = BST::new();
        bst.put(4, "val4");
        bst.put(10, "val10");
        bst.put(2, "val2");
        bst.put(3, "val3");
        bst.put(11, "val11");
        
        assert_eq!(bst.get(2).unwrap(), &"val2");
        let val = bst.delete_min();
        assert_eq!(bst.get(2).is_none(), true);
        // assert_eq!(&val.unwrap(), &"val4");
    }

    #[test]
    fn test_delete_max() {
        let mut bst: BST<u8,&str> = BST::new();
        bst.put(4, "val4");
        bst.put(10, "val10");
        bst.put(2, "val2");
        bst.put(3, "val3");
        bst.put(11, "val11");
        
        assert_eq!(bst.get(11).unwrap(), &"val11");
        let val = bst.delete_max();
        assert_eq!(bst.get(11).is_none(), true);
        assert_eq!(bst.get(3).is_some(), true);
    }
}

