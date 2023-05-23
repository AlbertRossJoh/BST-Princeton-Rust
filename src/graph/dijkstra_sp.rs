use crate::{sorting::index_min_pq::IndexMinPQ, fundamentals::stack::Stack};

use super::{edge::Edge, edge_weighted_graph::EdgeWeightedGraph, graph};



pub struct DijstraSP {
    dist_to: Vec<u128>,
    edge_to: Vec<Option<Edge>>,
    pq: IndexMinPQ<u128>,
}

impl DijstraSP {
    
    pub fn new(G: EdgeWeightedGraph, s: usize) -> Self{
        let mut dist_to = vec![u128::MAX;G.V];
        let edge_to:Vec<Option<Edge>> = vec![None; G.V];

        dist_to[s] = 0;
        let pq = IndexMinPQ::<u128>::new(G.V);

        let mut tmp = DijstraSP{dist_to: dist_to, edge_to: edge_to, pq:pq};
        tmp.pq.insert(&s, tmp.dist_to[s]);

        while !tmp.pq.is_empty() {
            let v = tmp.pq.delete_min();
            for e in G.adj(&v){
                tmp.relax(e, &v);
            }
        }
        tmp
    }

    pub fn get_distance_to(&self, v:&usize) -> u128 {
        self.dist_to[*v]
    }

    pub fn has_path_to(&self, v:&usize) -> bool {
        self.dist_to[*v] < u128::MAX
    }

    pub fn path_to(&self, v: &usize) -> Option<Stack<Edge>> {
        if !self.has_path_to(v) {
            return None;
        }
        let mut s = Stack::<Edge>::new();
        let mut x = *v;
        loop {
            if let Some(e) = &self.edge_to[x] {
                s.push(e.clone());
                x = *e.other(x);
            } else {
                break;
            }
        }
        Some(s)
    }


    fn relax(&mut self, e:&Edge, v: &usize){
        let w = e.other(*v);
        let tmp = &mut self.dist_to;
        if tmp[*w] > tmp[*v]+e.weight {
            tmp[*w] = tmp[*v]+e.weight;
            self.edge_to[*w] = Some(e.clone());
            if self.pq.contains(*w) {
                self.pq.decrease_key(w, self.dist_to[*w]);
            } else {
                self.pq.insert(w, self.dist_to[*w]);
            }
        }
        
    }
}