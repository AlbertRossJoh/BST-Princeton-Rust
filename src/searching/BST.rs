use std::{cmp::Ordering::{*, self}, mem};
/// The BST class represents an ordered symbol table og generic key pair values
/// It supports the operations `put`, `get`, `delete_max` and `delete_min`.
/// 
/// Every method should take constant worst-case running time: *O(N)*. With an average of *O(log N)*
/// Author: AlbertRossJoh
/// 
/// # Examples
///
/// ```
/// use itualgs_rs::searching::BST::BST;
///
/// let mut bst: BST<i32,&str> = BST::new();
/// 
/// bst.put(24, "Ferris");
/// 
/// assert_eq!(bst.get_root().unwrap(), &"Ferris");
/// 
/// let mut another_bst: BST<u8,&str> = BST::new();
/// another_bst.put(4, "val4");
/// another_bst.put(10, "val10");
/// another_bst.put(2, "val2");
/// another_bst.put(3, "val3");
/// another_bst.put(11, "val11");
/// 
/// assert_eq!(another_bst.get(&2).unwrap(), &"val2");
/// 
/// ```

struct Value<T,K> {
    key: T,
    val: K,
}

type Edge<T,K> = Option<Box<Node<T,K>>>;

struct Node<T,K> {
    value: Value<T,K>,
    left: Edge<T,K>,
    right: Edge<T,K>,
}


impl<T, K> Node<T,K> 
    where T:Ord
    {
    fn new(key: T, val: K) -> Self {
        Node { value: Value{ key: key, val: val }, left: None , right: None }
    }
    fn cmp(&self, other: &Node<T,K>) -> Ordering{
        other.value.key.cmp(&self.value.key)
    }

    fn cmp_to_key(&self, key: &T) -> Ordering {
        key.cmp(&self.value.key)
    }
}
fn get_min_node_mut<T,K>(node:&mut Edge<T,K>) -> &mut Edge<T,K>{
    let mut curr: *mut Edge<T,K> = node;
    unsafe{
        while let Some(ref mut e) = *curr {
            if e.left.is_none() {
                break;
            }
            curr = &mut e.left;
        }
        return &mut *curr;
    }
}

fn get_max_node_mut<T,K>(node:&mut Edge<T,K>) -> &mut Edge<T,K>{
    let mut curr: *mut Edge<T,K> = node;
    unsafe{
        while let Some(ref mut e) = *curr {
            if e.right.is_none() {
                break;
            }
            curr = &mut e.right;
        }
        return &mut *curr;
    }
}

pub struct BST<T,K>{
    root: Edge<T,K>,
}

impl<T,K> BST<T,K> 
    where T:Ord
{
    
    pub fn new() -> Self {
        BST { root: None }
    }

    pub fn get_root(&self) -> Option<&K>{
        if let Some(e) = &self.root {
            return Some(&e.value.val);
        }
        None
    }

    pub fn delete_max(&mut self){
        Self::del(get_max_node_mut(&mut self.root))
    }

    pub fn delete_min(&mut self){
        Self::del(get_min_node_mut(&mut self.root))
    }

    pub fn delete(&mut self, key: &T){
        let curr = self.get_mut(key);
        Self::del(curr);
    }

    /// Credit to https://codereview.stackexchange.com/users/32521/shepmaster
    /// https://codereview.stackexchange.com/questions/133209/binary-tree-implementation-in-rust/133776#133776
    fn del(node: &mut Edge<T,K>){
        if let Some(mut e) = node.take() {
            match (e.left.take(), e.right.take()) {
                (None, None) => (),
                (Some(o), None) | 
                (None, Some(o)) => *node = Some(o),
                (left, right) => {
                    e.left = left;
                    e.right = right;
                    {
                        let tmp = &mut *e;
                        let succ = get_min_node_mut(&mut tmp.right);
                        mem::swap(&mut tmp.value, &mut succ.as_mut().unwrap().value);
                        Self::del(succ);
                    }
                    *node = Some(e);
                }
            }
        }
    }
    
    fn get_mut(&mut self, key: &T) -> &mut Edge<T,K>
        where T:Ord
    {
        let mut curr: *mut Edge<T,K> = &mut self.root;
        unsafe {
            while let Some(ref mut node) = *curr {
                match node.cmp_to_key(&key) {
                    Less => curr = &mut node.left,
                    Greater => curr = &mut node.right,
                    Equal => {
                        return &mut *curr;
                    },
                }
            }
            return &mut *curr;

        }
    }


    pub fn get(&self, key: &T) -> Option<&K>
        where T:Ord
    {
        let mut curr = &self.root;
        while let Some(ref node) = curr {
            match node.cmp_to_key(&key) {
                Less => curr = &node.left,
                Greater => curr = &node.right,
                Equal => {
                    return Some(&curr.as_ref().unwrap().value.val);
                },
            }
        }
        None
    }

    pub fn put(&mut self, key: T, val: K) 
        where T:Ord
    {
        let to_insert = Node::new(key, val);
        if self.root.is_none() {
            self.root = Some(Box::new(to_insert));
            return;
        }
        let mut curr: *mut Edge<T,K> = &mut self.root;
        unsafe{
            while let Some(ref mut node) = *curr {
                match node.cmp(&to_insert) {
                    Less => curr = &mut node.left,
                    Greater => curr = &mut node.right,
                    Equal => {
                        node.value.val = to_insert.value.val;
                        return;
                    },
                }
            }
            curr.replace(Some(Box::new(to_insert)));
            return;
        }
    }

    
}


#[cfg(test)]
mod tests {
    use super::BST;
    // use crate itualgs_rs::searching::BST;

    #[test]
    fn test_deletion() {
       let mut bst: BST<i32,&str> = BST::new();
       bst.put(24, "Ferris");
       bst.put(20, "John");
       bst.put(25, "Jane");
       assert_eq!(bst.get_root().unwrap(), &"Ferris");
       assert_eq!(bst.get(&20).unwrap(), &"John");
       assert_eq!(bst.get(&25).unwrap(), &"Jane");
       let tmp = bst.delete(&25);
    //    println!("{}",tmp.is_some());
    //    assert_eq!(tmp.unwrap(), "Jane");
       assert_eq!(bst.get(&25).is_none(), true);
       assert_eq!(bst.get_root().unwrap(), &"Ferris");
       assert_eq!(bst.get(&20).unwrap(), &"John");
    }

    #[test]
    fn test_deletion_2() {
       let mut bst: BST<i32,&str> = BST::new();
       bst.put(23, "Ferris");
       bst.put(20, "John");
       bst.put(25, "Jane");
       bst.put(30, "Doe");
       bst.put(24, "wut");
       assert_eq!(bst.get_root().unwrap(), &"Ferris");
       assert_eq!(bst.get(&20).unwrap(), &"John");
       assert_eq!(bst.get(&25).unwrap(), &"Jane");
       let tmp = bst.delete(&25);
    //    println!("{}",tmp.is_some());
    //    assert_eq!(tmp.unwrap(), "Jane");
       assert_eq!(bst.get(&25).is_none(), true);
       assert_eq!(bst.get_root().unwrap(), &"Ferris");
       assert_eq!(bst.get(&20).unwrap(), &"John");
       assert_eq!(bst.get(&30).unwrap(), &"Doe");
    }
    //
    //#[test]
    //fn test_get() {
    //    let mut bst: BST<u8,&str> = BST::new();
    //    bst.put(4, "val4");
    //    bst.put(10, "val10");
    //    bst.put(2, "val2");
    //    bst.put(3, "val3");
    //    bst.put(11, "val11");
    //    
    //    let val = bst.get(4).unwrap();
    //    assert_eq!(val, &"val4");
    //    let val2 = bst.get(11).unwrap();
    //    assert_eq!(val2, &"val11");
    //}
    //
    //#[test]
    //fn test_put() {
    //    let mut bst: BST<u8,&str> = BST::new();
    //    bst.put(4, "val4");
    //    bst.put(10, "val10");
    //    bst.put(2, "val2");
    //    bst.put(3, "val3");
    //    bst.put(11, "val11");
    //    
    //    let val = bst.get_root().unwrap();
    //    assert_eq!(val, &"val4");
    //}
    //
    //#[test]
    //fn test_delete_min() {
    //    let mut bst: BST<u8,&str> = BST::new();
    //    bst.put(4, "val4");
    //    bst.put(10, "val10");
    //    bst.put(2, "val2");
    //    bst.put(3, "val3");
    //    bst.put(11, "val11");
    //    
    //    assert_eq!(bst.get(2).unwrap(), &"val2");
    //    let val = bst.delete_min();
    //    assert_eq!(bst.get(2).is_none(), true);
    //}
    //
    //#[test]
    //fn test_delete_max() {
    //    let mut bst: BST<u8,&str> = BST::new();
    //    bst.put(4, "val4");
    //    bst.put(10, "val10");
    //    bst.put(2, "val2");
    //    bst.put(3, "val3");
    //    bst.put(11, "val11");
    //    
    //    assert_eq!(bst.get(11).unwrap(), &"val11");
    //    let val = bst.delete_max();
    //    assert_eq!(bst.get(11).is_none(), true);
    //    assert_eq!(bst.get(3).is_some(), true);
    //}
}

