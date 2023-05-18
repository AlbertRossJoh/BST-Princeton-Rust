use std::{cmp::Ordering::*, fmt::Debug, mem};

#[derive(Clone)]
#[derive(Debug)]
struct Node<T:Ord, K:Ord> {
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
/// ```
pub struct BST<T:Ord, K:Ord> {root:Option<Box<Node<T, K>>>}

impl<'a, T:Ord, K:Ord> BST<T,K> where T:Clone, K:Clone {
    pub fn new()->BST<T,K>{
        BST { root: None }
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

impl<T:Ord, K:Ord> Node<T,K> where T:Clone, K:Clone {
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
}

