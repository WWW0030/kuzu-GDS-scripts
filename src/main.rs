use petgraph::graph::Graph;
use petgraph::algo::connected_components;
use petgraph::prelude::*;
use csv::Reader;
use std::error::Error;
use std::collections::HashMap;
use std::time::Instant;
use csv::ReaderBuilder;
use rayon;

fn main() -> Result<(), Box<dyn Error>> {

    // Set these two path to path of node and edge csv
    let node_path = "datagen-9_0-fbv.csv";
    let edge_path = "datagen-9_0-fbe.csv";

    rayon::ThreadPoolBuilder::new()
        .num_threads(10) // Set the desired number of threads
        .build_global()
        .expect("Failed to configure the global thread pool");

    let global_num_threads = rayon::current_num_threads();

    println!("Global Rayon thread pool is running on {} threads.", global_num_threads);

    let mut graph: Graph<u64, ()> = Graph::<u64, ()>::new();

    let mut node_map: HashMap<u64, NodeIndex> = HashMap::new();

    let mut rdr = Reader::from_path(node_path)?;
    for result in rdr.records() {
        let record = result?;
        let node_id: u64 = record.get(0).unwrap().parse()?;
        let node_index = graph.add_node(node_id);
        node_map.insert(node_id, node_index);
    }

    let mut rdr = ReaderBuilder::new().delimiter(b' ').from_path(edge_path)?;

    for result in rdr.records() {
        let record = result?;
        let from: u64 = record.get(0).unwrap().parse()?;
        let to: u64 = record.get(1).unwrap().parse()?;

        // Add edge using the node_map
        if let (Some(&src_idx), Some(&tgt_idx)) = (node_map.get(&from), node_map.get(&to)) {
            graph.add_edge(src_idx, tgt_idx, ());
        }
    }

    // Runs three times to get more accurate result
    let start = Instant::now();
    connected_components(&graph);
    let duration = start.elapsed();
    println!("Time taken: {:?}", duration);
    
    let start = Instant::now();
    connected_components(&graph);
    let duration = start.elapsed();
    println!("Time taken: {:?}", duration);

    let start = Instant::now();
    connected_components(&graph);
    let duration = start.elapsed();
    println!("Time taken: {:?}", duration);
    Ok(())
}
