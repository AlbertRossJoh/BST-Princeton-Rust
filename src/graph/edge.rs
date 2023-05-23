use std::cmp::Ordering;

#[derive(Clone)]
pub struct Edge {
    v: usize,
    w: usize,
    pub weight: u128
}

impl Edge {
    
    pub fn new(v: usize, w:usize, weight:u128) -> Self{
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

    pub fn cmp(&self, that: &Edge) -> Ordering{
        self.weight.cmp(&that.weight)
    }

    // pub fn cmp_for_sort(&self, that:Edge)

    pub fn clone(&self) -> Edge {
        Edge { v: self.v.clone(), w: self.w.clone(), weight: self.weight.clone() }
    }

}