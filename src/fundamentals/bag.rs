use std::collections::{LinkedList, binary_heap::Iter};



pub struct Bag<T> { 
    elements: LinkedList<T>,
    size: usize,
}

impl<T> Bag<T> where T:Clone {

    pub fn new() -> Bag<T>{
        Bag{
            elements: LinkedList::new(),
            size: 0,
        }
    }

    pub fn add(&mut self, item: T){
        self.elements.push_front(item);
        self.size += 1;
    }

    pub fn is_empty(&mut self) -> bool{
        self.elements.is_empty()
    }

    pub fn size(&mut self) -> usize{
        self.size
    }

    pub fn iterator(&mut self) -> std::collections::linked_list::Iter<'_, T>{
        self.elements.iter()
    }

    pub fn clone(&self) -> Bag<T>{
        Bag { elements:self.elements.clone(), size: self.size }
    }
}