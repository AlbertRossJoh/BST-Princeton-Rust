use std::{cmp::Ordering::*, any::{Any, TypeId}};

// #[derive(Clone)]
pub struct Node<T:Ord, K:Ord> {
    key: T,
    val: K,
    size: i32,
    left: Option<Box<Node<T,K>>>,
    right: Option<Box<Node<T,K>>>,
}

pub fn test_ting(){
    let mut n = Node::new(1, "hej", 0);
    let mut m = Node { key: 1, val: "jeh", size: 0, left: None, right: None };
    let mut bs: BST<i32,&str> = BST::new(n);
    if let Some(e) = bs.get_root(){
        println!("{}", e.val);
    }
    bs.insert(2, "val");
    if let Some(e) = bs.get_root(){
        if let Some(o) = &e.right{
            println!("{}", o.val);
        }
        // println!("{}",e.left.is_none());
        // println!("{}",e.right.unwrap().val);
    }
    
}

pub struct BST<T:Ord, K:Ord> {root:Option<Box<Node<T, K>>>}

impl<'a, T:Ord, K:Ord> BST<T,K> {
    pub fn new(root: Node<T, K>)->BST<T,K>{
        BST { root: Some(Box::new(root)) }
    }

    pub fn get_root(&self) -> &Option<Box<Node<T,K>>>{
        &self.root
    }

    pub fn insert(&mut self, key: T, val: K){
        let mut curr: &mut Option<Box<Node<T,K>>> = &mut self.root;
        let n = Node::new(key,val,0);
        while let Some(e) = curr{
            let cmp = &n.key.cmp(&e.key); 
            match cmp {
                Less => {
                    if (e.left.is_none()){
                        e.left = Some(Box::new(n));
                        return;
                    }
                    curr =&mut e.left
                },
                Greater => {
                    if (e.right.is_none()){
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
        
        // if let Some(e) = curr {
        //     match n.key.cmp(&e.key) {
        //         Less => {
        //             e.left = Some(Box::new(n))
        //         },
        //         Greater => {
        //             e.right = Some(Box::new(n))
        //         },
        //         Equal => {
        //             println!("hit");
        //             e.val = n.val
        //         },
        //     }
        // }
    }
}

impl<T:Ord, K:Ord> Node<T,K> {
    fn new(key:T, val:K, size: i32)->Node<T,K>{
        Node{
            key: key,
            val: val,
            size: size,
            left: None,
            right: None,
        }
    }
}
