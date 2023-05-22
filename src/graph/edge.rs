use std::cmp::Ordering;

#[derive(Clone)]
pub struct Edge {
    v: usize,
    w: usize,
    weight: f64
}

impl Edge {
    
    pub fn new(v: usize, w:usize, weight:f64) -> Self{
        Edge { v: v, w: w, weight: weight }
    }

    pub fn either(&self) -> &usize{
        &self.v
    }

    pub fn other(&self, vertex: usize) -> &usize{
        if vertex == self.v {&self.w}
        else if vertex == self.w {&self.v}
        else {
            panic!("Non legal \"other\"");
        }
    }

    pub fn cmp(&self, that:Edge) -> Ordering{
        self.weight.total_cmp(&that.weight)
    }

    pub fn clone(&self) -> Edge {
        Edge { v: self.v.clone(), w: self.w.clone(), weight: self.weight.clone() }
    }
}