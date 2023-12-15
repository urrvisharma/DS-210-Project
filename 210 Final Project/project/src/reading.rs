use std::fs::File;
use std::io::{self, BufRead, BufReader};
use crate::graph::TwitterGraph;

//function to read the text file (called in main)
pub fn read_data_from_file(file_path: &str) -> Result<TwitterGraph, io::Error> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut twitter_graph = TwitterGraph::new();

    for line in reader.lines() {
        let line = line?;
        let mut split = line.split_whitespace();
        if let (Some(source_str), Some(target_str)) = (split.next(), split.next()) {
            if let (Ok(source), Ok(target)) = (source_str.parse(), target_str.parse()) {
                twitter_graph.add_edge(source, target);
            }
        }
    }
    Ok(twitter_graph)
}