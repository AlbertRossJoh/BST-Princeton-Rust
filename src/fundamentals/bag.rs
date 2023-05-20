use std::collections::{LinkedList, binary_heap::Iter};


/// A bag is an onorderd data strructure of items which can be iterated over.
/// 
/// it supports the operations `add`, `iterator`, `size`, `to_list`, `new_from_vec` and `clone`.
/// 
/// Author: AlberRossJoh
/// 
/// # Examples
/// ```
/// use itualgs_rs::fundamentals::bag::Bag;
///
/// 
/// let mut list = vec!['f','e','r','r','i','s'];
/// let mut bag = Bag::new_from_vec(&list);
/// 
/// let mut to_list = bag.to_list();
/// to_list.reverse();
/// assert_eq!(&to_list,&list);
/// 
/// ```
pub struct Bag<T> { 
    elements: LinkedList<T>,
    size: usize,
}

impl<T> Bag<T> where T:Clone {

    /// creates a new bag
    pub fn new() -> Bag<T>{
        Bag{
            elements: LinkedList::new(),
            size: 0,
        }
    }

    /// creates a new bag from a vector, while the vector maintains ownership
    pub fn new_from_vec(v:&Vec<T>) -> Bag<T> {
        let mut b = Bag{
            elements: LinkedList::new(),
            size: 0,
        };
        for item in v {
            b.elements.push_front(item.clone());
        }
        b
    }

    /// adds element to the bag
    pub fn add(&mut self, item: T){
        self.elements.push_front(item);
        self.size += 1;
    }

    /// Returns true if bag is empty
    pub fn is_empty(&mut self) -> bool{
        self.elements.is_empty()
    }

    /// Size of the bag
    pub fn size(&mut self) -> usize{
        self.size
    }

    /// Gets an iterator of the bag
    pub fn iterator(&self) -> std::collections::linked_list::Iter<'_,T>{
        self.elements.iter()
    }

    /// turns the bag into a vector, the bag goes out of scope
    pub fn to_list(self) -> Vec<T> {
        self.iterator().map(|t | {t.clone()}).collect()
    }

    pub fn clone(&self) -> Bag<T>{
        Bag { elements:self.elements.clone(), size: self.size }
    }
}



#[cfg(test)]
mod tests {

    use super::Bag;


    #[test]
    fn test_bag() {
        let list = vec!['f','e','r','r','i','s'];
        let mut bag = Bag::new_from_vec(&list);

        let mut to_list = bag.to_list();
        to_list.reverse();
        assert_eq!(&to_list,&list);
    }
}
