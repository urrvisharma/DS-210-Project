use std::collections::{HashMap, HashSet, VecDeque};
use petgraph::graph::{DiGraph, NodeIndex};

//using a struct to create my graph created out of all of the Twitter users containing 2 fields: graph, and user_map
pub struct TwitterGraph {
    pub graph: DiGraph<u64, ()>, //nodes are specified as u64, edges are ()
    pub user_map: HashMap<u64, NodeIndex>, //hashmap that connects user ids to nodes
}

impl TwitterGraph { //implementing TwitterGraph
    pub fn new() -> Self {
        TwitterGraph { 
            graph: DiGraph::new(),
            user_map: HashMap::new(),
        }
    }

    pub fn add_edge(&mut self, source: u64, target: u64) { //adds edges between two users in the graph created 
        let source_idx = *self.user_map.entry(source).or_insert_with(|| {
            let idx = self.graph.add_node(source);
            idx
        });

        let target_idx = *self.user_map.entry(target).or_insert_with(|| {
            let idx = self.graph.add_node(target);
            idx
        });

        self.graph.add_edge(source_idx, target_idx, ());
    }

    //uses breadth first search to find the minimum distance between the two specified users in the graph
    pub fn min_distance(&self, start_user: u64, end_user: u64) -> Option<usize> {
        let start_idx = self.user_map.get(&start_user)?;
        let end_idx = self.user_map.get(&end_user)?;

        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();

        queue.push_back((*start_idx, 0));

        while let Some((current, distance)) = queue.pop_front() {
            if current == *end_idx {
                return Some(distance);
            }

            if visited.contains(&current) {
                continue;
            }

            visited.insert(current);

            for neighbor in self.graph.neighbors(current) {
                queue.push_back((neighbor, distance + 1));
            }
        }

        None
    }

    //uses breadth first search to find max distance between the specified users
    pub fn max_distance(&self, start_user: u64, end_user: u64) -> Option<usize> { 
        let start_idx = self.user_map.get(&start_user)?;
        let end_idx = self.user_map.get(&end_user)?;

        let mut visited = HashSet::new();
        let mut queue = Vec::new();

        queue.push((*start_idx, 0));

        while let Some((current, distance)) = queue.pop() {
            if current == *end_idx {
                return Some(distance);
            }

            if visited.contains(&current) {
                continue;
            }

            visited.insert(current);

            for neighbor in self.graph.neighbors(current) {
                queue.push((neighbor, distance + 1));
            }
        }

        None
    }

    //calculates the distances from the specified node to all other nodes in the graph and takes the average (uses bfs)
    pub fn average_distance_of_user(&self, user: u64) -> Option<f64> {
        let start_idx = self.user_map.get(&user)?;

    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut total_distance = 0;
    let mut visited_count = 0;

    queue.push_back((*start_idx, 0));

    while let Some((current, distance)) = queue.pop_front() {
        if visited.contains(&current) {
            continue;
        }

        visited.insert(current);

        total_distance += distance;
        visited_count += 1;

        for neighbor in self.graph.neighbors(current) {
            queue.push_back((neighbor, distance + 1));
        }
    }

    if visited_count > 0 {
        Some(total_distance as f64 / visited_count as f64)
    } else {
        None
    }
    }
    
}




