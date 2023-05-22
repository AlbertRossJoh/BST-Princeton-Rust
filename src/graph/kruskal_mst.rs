use std::rc::Rc;

use crate::fundamentals::{queue::Queue, uf::WeightedQuickUnionUF};

use super::{edge::Edge, edge_weighted_graph::EdgeWeightedGraph};


pub struct KruskalMST {
    pub weight: f64,
    mst: Queue<Rc<Edge>>,
}

impl KruskalMST {
    
    pub fn new(G: EdgeWeightedGraph) -> Self {
        let mut kruskal = KruskalMST{weight: 0., mst: Queue::new()};
        let mut edges: Vec<&Rc<Edge>> = Vec::new();
        let tmp = G.edges();
        for e in tmp.iterator() {
            edges.push(e);
        }


        edges.sort_by(|a, b| a.cmp(b));

        let mut uf = WeightedQuickUnionUF::new(G.V);

        let mut i: usize = 0;
        loop {
            if i >= G.E || kruskal.mst.size() > G.V - 1 {
                break;
            }
            let edge = edges[i];
            let v = edge.either();
            let w = edge.other(*v);

            if uf.find(*v) != uf.find(*w) {
                uf.union(*v, *w);
                kruskal.mst.enqueue(edge.clone());
                kruskal.weight += edge.weight
            }
            i+=1
        }
        kruskal
    }

    pub fn edges(self) -> Queue<Rc<Edge>>{
        self.mst
    }
}

#[cfg(test)]
mod tests {
    use crate::graph::edge::Edge;

    use super::EdgeWeightedGraph;

    use super::KruskalMST;

    #[test]
    fn test_create_graph(){
        let mut g = EdgeWeightedGraph::new(4);
        let list = vec![
            Edge::new(0, 1, 0.2), 
            Edge::new(2, 1, 5.), 
            Edge::new(2, 0, 0.3)];
        
        for ele in list {
            g.add_edge(ele);
        }
        let mut k = KruskalMST::new(g);
        let mut edges = &mut k.edges();
        for _ in 0..edges.size().clone() {
            println!("{}", edges.dequeue().unwrap().weight);
        }
        // let paths = DFP::new(&mut g, 2);
        // assert_eq!(paths.marked[1], true);
        // assert_eq!(paths.edge_to[2], 0);
        // assert_eq!(paths.marked[2], true);
        // assert_eq!(paths.marked[3], false);
    }
}
