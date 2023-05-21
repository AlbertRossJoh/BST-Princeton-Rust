/// The Quick Union fundamentals module represents a union find structure
/// which allows for checking dynamic connectivity. This specific implementation uses
/// a Non-Weighted Quick Union without path compression.
/// 
/// Afterwards all operations except for the count operation takes linear time (worst case).
/// For additional documentation, see Section 1.5 of Algorithms, 4th Edition by Robert Sedgewick and Kevin Wayne.
///  
/// Author: cave
/// 
/// # Examples
/// ```
/// use itualgs_rs::fundamentals::uf::QuickUnionUF;
/// 
/// let mut our_qu = QuickUnionUF::new(5);
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
pub struct QuickUnionUF {
    count: usize,
    id: Vec<usize>
}

impl QuickUnionUF {
    /// Create a new QuickUnionUF with N elements were all elements are singletons
    pub fn new(n: usize) -> QuickUnionUF {
        let mut id = vec![0; n];
        for i in 0..n {
            id[i] = i;
        }
        QuickUnionUF { count: n, id }
    }

    /// Finds the root of a node 
    pub fn find(&self, mut p: usize) -> usize {
        while p != self.id[p] {
            p = self.id[p];
        }
        p
    }

    /// Checks if two nodes are connected by comparing their roots
    pub fn connected(&self, p: usize, q: usize) -> bool {
        self.find(p) == self.find(q)
    }

    /// Returns the count 
    pub fn count(&self) -> usize {
        self.count
    }

    /// Unions two elements together
    pub fn union(&mut self, p: usize, q: usize) {
        let root_p = self.find(p);
        let root_q = self.find(q);

        if root_p == root_q {
            return;
        }

        self.id[root_p] = root_q;
        self.count -= 1;
    }
}

/// The Weighted Quick Union fundamentals module represents a union find structure
/// which allows for checking dynamic connectivity. This specific implementation uses
/// a Weighted Quick Union without path compression.
/// 
/// The main improvement in using a Weighted Quick Union is the fact that the tree
/// structure will be flattened, which allows for a faster find operation. The only
/// real code difference between a non-weighted quick union and a weighted being an 
/// if-statement that ensures that the smaller tree is always added to the root of
/// the larger tree. 
/// 
/// Author: cave
/// 
/// # Examples
/// ```
/// use itualgs_rs::fundamentals::uf::WeightedQuickUnionUF;
/// 
/// let mut our_qu = WeightedQuickUnionUF::new(5);
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
pub struct WeightedQuickUnionUF {
    count: usize,
    size: Vec<usize>,
    id: Vec<usize>
}

impl WeightedQuickUnionUF { 
    pub fn new(n: usize) -> WeightedQuickUnionUF {
        let mut id = vec![0; n];
        let mut size = vec![0; n];
        for i in 0..n {
            id[i] = i;
            size[i] = 1;
        }
        WeightedQuickUnionUF { count: n, size, id }
    } 

    /// Finds the root of a node 
    pub fn find(&self, mut p: usize) -> usize {
        while p != self.id[p] {
            p = self.id[p];
        }
        p
    }

    /// Checks if two nodes are connected by comparing their roots
    pub fn connected(&self, p: usize, q: usize) -> bool {
        self.find(p) == self.find(q)
    }

    /// Returns the count 
    pub fn count(&self) -> usize {
        self.count
    }

    /// Unions two elements together
    pub fn union(&mut self, p: usize, q: usize) {
        let root_p = self.find(p);
        let root_q = self.find(q);

        // They are already connected
        if root_p == root_q {
            return;
        }

        let mut smallest = root_q;
        let mut largest = root_p;

        // The smallest is actually the larger, 
        // one of the two -> swap the values around
        if self.size[root_p] < self.size[root_q] {
            smallest = root_p;
            largest = root_q;
        } 

        self.id[smallest] = largest;
        self.size[largest] += self.size[smallest];

        self.count -= 1;
    }
}

/// The Quick Find fundamentals module represents a union find structure
/// which allows for checking dynamic connectivity. For example. P is connected to Q,
/// If P is connected to Q, then Q is connected to P, and last but not least, if
/// P is connected to Q, and Q is connected to Z, then P is connected to Z
/// 
/// Quick Find has a time complexity for union at O(N^2)
/// Quick Find has a really quick find, however being constant or O(1)
///  
/// Author: cave & AlbertRossJoh
/// 
/// # Examples
/// ```
/// use itualgs_rs::fundamentals::uf::QuickFindUF;
/// 
/// let mut our_qu = QuickFindUF::new(5);
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
pub struct QuickFindUF {
    count: usize,
    id: Vec<usize>,
}

impl QuickFindUF {
    /// Create a new QuickFindUF with N elements were all elements are singletons
    pub fn new(n: usize) -> QuickFindUF {
        let mut id = vec![0; n];

        for i in 0..n {
            id[i] = i;
        }
        QuickFindUF { count: n, id }
    }

    /// Return the count
    pub fn count(&self) -> usize {
        self.count
    }
    
    /// Check if two elements are connected
    pub fn connected(&self, p: usize, q: usize) -> bool {
        self.id[p] == self.id[q]
    }

    /// Find the root of a node
    pub fn find(&self, p: usize) -> usize {
        self.id[p]
    }

    /// Union two nodes
    pub fn union(&mut self, p: usize, q: usize) {
        let root_p = self.id[p];
        let root_q = self.id[q];

        if root_p == root_q {
            return;
        }

        for i in 0..self.id.len() {
            if self.id[i] == root_p {
                self.id[i] = root_q;
            }
        }
        self.count -= 1;
    }
}

#[cfg(test)]
mod tests {
    /* Tests for QuickFindUF */
    use super::QuickFindUF;

    #[test]
    pub fn tiny_uf_sedgewick_qf() {
        let mut qu = QuickFindUF::new(10);
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
        let mut our_qu = QuickFindUF::new(5);
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

    /* Tests for Quick Union */
    use super::QuickUnionUF;

    #[test]
    pub fn tiny_uf_sedgewick_qu() {
        let mut qu = QuickUnionUF::new(10);
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

    /* Tests for Weighted Quick Union */
    use super::WeightedQuickUnionUF;

    #[test]
    pub fn tiny_uf_sedgewick_wqu() {
        let mut qu = WeightedQuickUnionUF::new(10);
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
}