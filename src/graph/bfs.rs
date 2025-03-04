use std::collections::{HashMap, VecDeque};

pub struct Graph {
    map: HashMap<usize, Vec<usize>>,
    visited: Vec<bool>,
    queue: VecDeque<usize>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            map: HashMap::new(),
            visited: vec![],
            queue: VecDeque::new(),
        }
    }

    pub fn add_edge(&mut self, start: usize, end: usize) {
        self.map.entry(start).or_insert(Vec::new()).push(end);
        self.map.entry(end).or_insert(Vec::new());
        self.visited = vec![false; self.map.len()]
    }

    pub fn bfs(&mut self, start: usize) {
        self.visited[start] = true;
        println!("now it is passing {}", start);
        if let Some(neighbors) = self.map.get(&start) {
            let neighbors = neighbors.clone();
            for neighbor in neighbors {
                self.queue.push_back(neighbor)
            }

            while let Some(node) = self.queue.pop_front() {
                self.visited[node] = true;
                println!("now it is passing {}", node);
                if let Some(neighbors) = self.map.get(&node) {
                    let neighbors = neighbors.clone();
                    for neighbor in neighbors {
                        if self.visited[neighbor] == false {
                            self.queue.push_back(neighbor)
                        }
                    }
                }
            }
        }
    }
}