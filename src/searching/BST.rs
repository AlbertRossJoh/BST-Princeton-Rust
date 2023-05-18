use std::{cmp::Ordering::*, any::{Any, TypeId}, iter::Cloned, fmt::Debug};

#[derive(Clone)]
#[derive(Debug)]
struct Node<T:Ord, K:Ord> {
    key: T,
    val: K,
    size: i32,
    left: Option<Box<Node<T,K>>>,
    right: Option<Box<Node<T,K>>>,
}

pub fn test_ting(){
    let mut n = Node::new(1, "hej", 0);
    let mut m = Node { key: 1, val: "jeh", size: 0, left: None, right: None };
    let mut bs: BST<i32,String> = BST::new();
    println!("{}", bs.get_root().is_none());

    for i in 0..10{
        bs.put(i, format!("{}", i+10))
    }
    if let Some(e) = bs.get_root(){
        println!("{}", e);
    }
    bs.put(2, "huh".to_string());
    if let Some(e) = bs.get_root(){
        // if let Some(o) = &e.right{
        //     println!("{}", "hit");
        //     println!("{}", o.val);
        //     println!("{}", "hit");
        // }
        // println!("{}",e.left.is_none());
        // println!("{}",e.right.unwrap().val);
    }
    
}

pub struct BST<T:Ord, K:Ord> {root:Option<Box<Node<T, K>>>}

impl<'a, T:Ord, K:Ord> BST<T,K> where T:Clone, K:Clone {
    pub fn new()->BST<T,K>{
        BST { root: None }
    }

    pub fn delete_min(&mut self) -> Option<K>{
        if let Some(e) = &mut self.root {
            if e.left.is_none() {
                if e.right.is_some() {
                    let val = e.as_mut().val.clone();
                    self.root = e.right.clone();
                    return Some(val);
                } else {
                    self.root = None;
                    return None;
                }
            }
        } else {
            return None;
        }
        let mut curr: &mut Option<Box<Node<T,K>>> = &mut self.root;
        while let Some(e) = curr {
            match (e.left.is_some(), e.right.is_some()) {
                (true, _) => {
                    curr = &mut e.left;
                }
                (false, false) => {
                    let val = e.val.clone();
                    curr = &mut  None;
                    return Some(val);
                }
                (false, true) => {
                    let val = e.val.clone();
                    e.right = None;
                    return Some(val);
                }
            }
        }
        None
    }

    pub fn delete_max(&mut self) -> Option<K>{
        if let Some(e) = &mut self.root {
            if e.right.is_none() {
                if e.left.is_some() {
                    let val = e.as_mut().val.clone();
                    self.root = e.left.clone();
                    return Some(val);
                } else {
                    self.root = None;
                    return None;
                }
            }
        } else {
            return None;
        }
        let mut curr: &mut Option<Box<Node<T,K>>> = &mut self.root;
        while let Some(e) = curr {
            match (e.right.is_some(), e.left.is_some()) {
                (true, _) => {
                    curr = &mut e.right;
                }
                (false, false) => {
                    let val = e.val.clone();
                    curr = &mut  None;
                    return Some(val);
                }
                (false, true) => {
                    let val = e.val.clone();
                    e.right = None;
                    return Some(val);
                }
            }
        }
        None
    }

    pub fn get_root(&self) -> &Option<K>{
        if let Some(e) = &self.root {
            Some(&e.val);
        }
        &None
    }

    pub fn get(&mut self, key: T) -> Option<&mut K>{
        // let n = Node::new(key,val,0);
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

    fn Clone(&self)->Node<T,K>{
        Node{
            key: self.key.clone(),
            val: self.val.clone(),
            size: self.size.clone(),
            left: self.right.clone(),
            right: self.left.clone(),
        }
    }
}

