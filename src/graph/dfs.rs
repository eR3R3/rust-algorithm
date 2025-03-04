use std::collections::HashMap;

pub struct Graph {
    map: HashMap<usize, Vec<usize>>,
    history: Vec<bool>
}

impl Graph {
    pub fn new() -> Graph {
        Graph{
            map: HashMap::new(),
            history: vec![]
        }
    }

    pub fn add_edge(self: &mut Self, start: usize, end: usize) {
        self.map.entry(start).or_insert(vec![]).push(end);
        self.map.entry(end).or_insert(vec![]);
        self.history = vec![false; self.map.len()]
    }

    pub fn dfs(self: &mut Self, start: usize)  {
        if self.history[start] {
            println!("{:?}", self.history);
            return
        }
        self.history[start] = true;
        println!("{}",format!("passing point {start}"));
        if let Some(neighbors) = self.map.get(&start) {
            let neighbors = neighbors.clone();
            for neighbor in neighbors {
                self.dfs(neighbor)
            }
        }
    }
}