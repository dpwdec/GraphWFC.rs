mod observe;
mod propagate;
mod utils;
mod graph;

use std::collections::{HashSet, BinaryHeap, HashMap};
use crate::observe::Observe;
use crate::propagate::Propagate;
use crate::graph::{Rules, Graph, VertexIndex, VertexLabel, Frequencies};
use crate::utils::{hash_set, hash_map};

fn main() {
    // turn input into a graph
    // index number represents each vertex position
    // generate the rules
    // Set up uncollapsed output labels
    // generate output graph from input graph

    // let output_graph_vertices = vec![hash_set(&[0, 1, 2]); 4];
    //
    // let output_graph_edges: HashMap<vertexIndex, Vec<(vertexIndex, EdgeDirection)>> = hash_map(&[
    //     (0, vec![(1, 0), (3, 2)]),
    //     (1, vec![(0, 1), (2, 2)]),
    //     (2, vec![(3, 1), (1, 3)]),
    //     (3, vec![(0, 3), (2, 0)])
    // ]);
    //
    // let output_graph = Graph::new(
    //     output_graph_vertices,
    //     output_graph_edges
    // );
}

fn collapse_algorithm(rules: &Rules, frequencies: &Frequencies, out_graph: Graph) -> Option<Graph> {
    let N0ne = None;

    let mut heap: BinaryHeap<Observe> = BinaryHeap::new();
    let mut gen_observe: HashSet<VertexIndex> = HashSet::new();
    let mut observed: HashSet<VertexIndex> = HashSet::new();
    let mut propagations: Vec<Propagate> = Vec::new();

    // Initialize binary heap
    // todo: ensure random order of initial observes.
    // todo: add initial propagation step to the collapse algorithm.
    out_graph.vertices.iter().enumerate().for_each(|(index, labels)| {
        heap.push(Observe::new(&(index as i32), labels, frequencies))
    });

    loop {
        if observed.len() == out_graph.vertices.len() || heap.is_empty() { return Some(out_graph) }
        if propagations.is_empty() {
            gen_observe.drain().for_each(|index| {  // algo: 4.2
                let labels = out_graph.vertices.get(index as usize).unwrap();
                heap.push(Observe::new(&index, labels, frequencies))
            });
            
            //heap.pop().unwrap()
        } else {
            // do propagate
        }
    }

    N0ne
}
