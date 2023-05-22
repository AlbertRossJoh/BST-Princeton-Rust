use std::{collections::linked_list::Iter, rc::Rc};

use crate::fundamentals::bag::Bag;

use super::edge::Edge;


pub struct EdgeWeightedGraph {
    pub V: usize,
    pub E: usize,
    adj: Vec<Bag<Rc<Edge>>>,
}


impl EdgeWeightedGraph {
    pub fn new(V: usize) -> Self {
        let mut tmp: Vec<Bag<Rc<Edge>>> = Vec::with_capacity(V);
        for _ in 0..V {
            tmp.push(Bag::<Rc<Edge>>::new());
        }
        EdgeWeightedGraph { V: V, E: 0, adj: tmp}
    }

    pub fn add_edge(&mut self, e:Edge){
        let v = e.either();
        let w = e.other(*v);
        self.validate(v);
        self.validate(w);
        self.adj[*v].add(Rc::new(e.clone()));
        self.adj[*w].add(Rc::new(e));
        self.E += 1;
    }

    pub fn adj(&self, v:&usize) -> Iter<Rc<Edge>>{
        self.validate(v);
        self.adj[*v].iterator()
    }

    pub fn degree(&mut self, v:&usize) -> usize {
        self.validate(v);
        self.adj[*v].size()
    }

    pub fn edges(&self) -> Bag<Rc<Edge>> {
        let mut list = Bag::<Rc<Edge>>::new();
        for v in 0..self.V {
            let mut self_loops = 0;
            for e in self.adj(&v){
                if e.other(v) > &v {
                    list.add(Rc::clone(e));
                } else if e.other(v) == &v {
                    if self_loops % 2 == 0 {
                        list.add(Rc::clone(e))
                    }
                    self_loops += 1;
                }
            }
        }
        list
    }


    fn validate(&self, v:&usize){
        if v>= &self.V {
            panic!("Out of bounds!!")
        }
    }
}