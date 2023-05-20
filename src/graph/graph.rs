use crate::fundamentals::bag::Bag;


pub struct Graph{
    V: usize,
    E: usize,
    adj: Vec<Bag<usize>>
}

impl Graph {
    
    /// Inits a new graph with V vertices
    pub fn new(v:usize) -> Graph {
        let mut g = Graph { V: v, E: 0, adj: Vec::new() };
        for _ in 0..v {
            g.adj.push(Bag::<usize>::new());
        }
        g
    }


    /// Gets the degree of a given vertex
    pub fn degree(&mut self, v:usize) -> usize {
        self.validate(v);
        self.adj[v].size()
    }

    /// returns the vertices adjacent to v
    pub fn adj_vertices(&self, v:&usize) -> std::collections::linked_list::Iter<'_, usize> {
        self.validate(*v);
        self.adj[*v].iterator()
    }

    /// adds an edge between v and w
    pub fn add_edge(&mut self, v:usize, w:usize){
        self.validate(v);
        self.validate(w);
        self.E += 1;
        self.adj[v].add(w);
        self.adj[w].add(v);
    }

    fn validate(&self, p:usize){
        if p>= self.V {
            panic!("The index is out of bounds!")
        }
    }

    pub fn get_v(&self) -> usize {
        self.V
    }

    pub fn get_e(&self) -> usize {
        self.E
    }

    /// Creates a deep copy of the graph
    pub fn clone(&self) -> Graph {
        let mut temp: Vec<Bag<usize>> = Vec::new();
        for i in &self.adj {
            temp.push(i.clone());
        }
        Graph { V: self.V.clone(), E: self.E.clone(), adj: temp }
    }
}


#[cfg(test)]
mod tests {
    use super::Graph;

    #[test]
    fn test_create_graph(){
        let mut g = Graph::new(4);
        g.add_edge(0, 1);
        g.add_edge(0, 2);
        let mut it = g.adj_vertices(&0);
        assert_eq!(*it.next().unwrap(), 2 as usize);
        assert_eq!(*it.next().unwrap(), 1 as usize);
    }

    #[test]
    fn test_create_empty_graph(){
        let mut g = Graph::new(4);
        let mut it = g.adj_vertices(&0);
        assert_eq!(it.next(), None);
    }
}