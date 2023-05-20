

pub struct QuickFind{
    count: usize,
    id: Vec<usize>,
}



impl QuickFind {

    pub fn new(&self, n:usize) -> QuickFind{
        let mut qf = QuickFind{count: n, id: Vec::with_capacity(n)};
        for i in 0..n {
            qf.id.push(i);
        }
        qf
    }

    pub fn count(&self) -> usize {
        self.count
    }

    pub fn find(&self, p: usize) -> usize {
        self.validate(p);
        self.id[p]
    }

    fn validate(&self, p: usize){
        if p>self.id.len() {
            panic!("Index is out of bounds!")
        }
    }

    pub fn connected(&self, p: usize,q:usize) -> bool{
        self.validate(p);
        self.validate(q);
        self.id[p] == self.id[q]
    }

    pub fn union(&mut self, p: usize, q:usize){
        self.validate(p);
        self.validate(q);
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