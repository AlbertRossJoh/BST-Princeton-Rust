use crate::fundamentals::stack::Stack;

use super::graph::Graph;
use std::collections::{linked_list::Iter};


/// Depth first search implmented for at graph. Depth first search is a graph searching algorithm that finds which vertices are connected to the vertex v.
/// 
/// DFS contains a vector of booleans to check weather the vertices are connected or not.
/// 
/// It does not support any operations, since it is just a data struct.
/// 
/// Author: AlberRossJoh
/// 
/// # Examples
/// ```
/// use itualgs_rs::graph::DFS::DFS;
/// use itualgs_rs::graph::graph::Graph;
///
/// let mut g = Graph::new(4);
/// g.add_edge(0, 1);
/// g.add_edge(0, 2);
/// let paths = DFS::new(&mut g, 2);
/// assert_eq!(paths.marked[1], true);
/// assert_eq!(paths.marked[0], true);
/// assert_eq!(paths.marked[2], true);
/// assert_eq!(paths.marked[3], false);
/// ```
pub struct DFS{
    pub marked: Vec<bool>,
}

impl DFS {

    /// creates a new marked list from a graph
    pub fn new(g:&mut Graph, s: usize) -> DFS{
        DFS { marked: Self::dfs(g,s) }
    }


    fn dfs(g:&mut Graph, s: usize) -> Vec<bool>{
        let mut m:Vec<bool> = vec![false;g.get_v()];
        let mut adj: Vec<Iter<usize>> = Vec::with_capacity(g.get_v());

        for v in 0..g.get_v() {
            adj.push(g.adj_vertices(v))
        }
    
        let mut stack = Stack::<usize>::new();
        m[s] = true;
        stack.push(s);
        while !stack.is_empty() {
            let v = stack.peek().unwrap();
            if let Some(w) = adj[*v].next(){
                if !m[*w] {
                    m[*w] = true;
                    stack.push(*w);
                }
            } else {
                stack.pop();
            }
        }
        m
    }
}


#[cfg(test)]
mod tests {
    use crate::graph::graph::Graph;

    use super::DFS;

    #[test]
    fn test_create_graph(){
        let mut g = Graph::new(4);
        g.add_edge(0, 1);
        g.add_edge(0, 2);
        let paths = DFS::new(&mut g, 2);
        assert_eq!(paths.marked[1], true);
        assert_eq!(paths.marked[0], true);
        assert_eq!(paths.marked[2], true);
        assert_eq!(paths.marked[3], false);
    }
}
