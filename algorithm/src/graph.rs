use std::{hash::Hash, collections::{VecDeque, HashSet}};

pub trait Graph {
   type Node: Hash + Eq + PartialEq + Clone; 
   fn get_neighbors(&self, node: &Self::Node) -> Vec<&Self::Node>;
}

fn bfs<G: Graph>(g: G, node: G::Node) {
    let mut queue = VecDeque::new();    
    let mut visited: HashSet<&<G as Graph>::Node> = HashSet::new();

    queue.push_back(&node);

    while let Some(n) =  queue.pop_front(){
        if visited.contains(&n) {
            continue;
        }
        visited.insert(&n);
        for n in g.get_neighbors(&n) {
            queue.push_back(&n);
        }
    }
}