use std::fs::File;
use std::collections::HashMap;

pub struct Graph {
    index: usize,
    adj_matrix: Vec<Vec<bool>>, // true if x is a large cave where x is the origin in adj_matrix[x][y]
    v_map: HashMap<String, usize>,
}

impl Graph {
    fn new() -> Graph {
        Graph {
            index:0,
            adj_matrix: Vec::new()
        }
    }

    fn add_vertex() {}

    fn get_vertex_by_name(&self, name: &str) -> usize {}

    fn add_edge(&mut self, from: &str, to: &str) {}
    
    fn get_edge() -> bool {}

    // recursive search
    fn traverse(&mut self, mut visited: HashSet<usize>, hit_twice: bool) -> usize {
        let mut count: usize = 0

        count
    }

}


fn main() {
}
