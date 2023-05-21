/// The Quick Find fundamentals module represents a union find structure
/// which allows for checking dynamic connectivity. For example. P is connected to Q,
/// If P is connected to Q, then Q is connected to P, and last but not least, if
/// P is connected to Q, and Q is connected to Z, then P is connected to Z
/// 
/// Quick Find has a time complexity for union at O(N^2)
/// Quick Find has a really quick find, however being constant or O(1)
///  
/// Author: cave
/// 
/// # Examples
/// ```
/// use itualgs_rs::fundamentals::quick_find::QuickFind;
/// 
/// let mut our_qu = QuickFind::new(5);
/// our_qu.union(0, 2);
/// our_qu.union(2, 4);
/// 
/// // Check for reflexivity
/// assert!(our_qu.connected(0,0) == true);
///
/// // Check for symmetry 
/// assert!(our_qu.connected(0,2) == true);
/// assert!(our_qu.connected(2,0) == true);
/// 
/// // Check for transitivity 
/// assert!(our_qu.connected(0,4) == true);
/// ``` 
pub struct QuickFind {
    count: usize,
    id: Vec<usize>,
}

impl QuickFind {
    // Create a new QuickFind with N elements were all elements are singletons
    pub fn new(n: usize) -> QuickFind {
        let mut id = vec![0; n];
        for i in 0..n {
            id[i] = i;
        }
        QuickFind { count: n, id }
    }

    // Return the count
    pub fn count(&self) -> usize {
        self.count
    }
    
    // Check if two elements are connected
    pub fn connected(&self, p: usize, q: usize) -> bool {
        self.id[p] == self.id[q]
    }

    // Find the root of a node
    pub fn find(&self, p: usize) -> usize {
        self.id[p]
    }

    // Union two nodes
    pub fn union(&mut self, p: usize, q: usize) {
        let p_id = self.id[p];
        let q_id = self.id[q];

        if p_id == q_id {
            return;
        }

        for i in 0..self.id.len() {
            if self.id[i] == p_id {
                self.id[i] = q_id;
            }
        }
        self.count -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::QuickFind;

    #[test]
    // Test from [https://algs4.cs.princeton.edu/15uf/tinyUF.txt]
    pub fn tiny_uf_sedgewick() {
        let mut qu = QuickFind::new(10);
        qu.union(4, 3);
        qu.union(3, 8);
        qu.union(6, 5);
        qu.union(9, 4);
        qu.union(2, 1);
        qu.union(8, 9);
        qu.union(5, 0);
        qu.union(7, 2);
        qu.union(6, 1);
        qu.union(1, 0);
        qu.union(6, 7);

        assert!(qu.connected(1, 6) == true);
        assert!(qu.connected(7, 2) == true);
        assert!(qu.connected(6, 6) == true);
    }
    #[test]
    pub fn dynamic_connectivity_test() {
        let mut our_qu = QuickFind::new(5);
        our_qu.union(0, 2);
        our_qu.union(2, 4);
            
        // Check for reflexivity
        assert!(our_qu.connected(0,0) == true);
            
        // Check for symmetry 
        assert!(our_qu.connected(0,2) == true);
        assert!(our_qu.connected(2,0) == true);
            
        // Check for transitivity 
        assert!(our_qu.connected(0,4) == true);
    }
}