use crate::custom_error::AocError;
use itertools::Itertools;
use petgraph::{
    dot::{Config, Dot},
    prelude::*,
};
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use petgraph::graph::Node;
use petgraph::visit::Walker;


#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String, AocError> {
    let graph_vec: Vec<Vec<u32>> = _input
        .lines()
        .map(|line| {
            line
                .chars()
                .map(|ch| ch.to_digit(10).unwrap())
                .collect()
        }).collect();
    println!("graph: {:?}", graph_vec);


    // let mut graph = DiGraph::<&str, u32>::new();
    // let node_map: HashMap<&str, NodeIndex> = graph_vec
    //     .iter()
    //     .map(|row|
    //         row.iter()
    //             .map(|col|{
    //                 ( ,graph.add_node(col));
    //             })).collect();
    //


    // println!("{:?}", node_map);
    //
    //
    // let dot_txt = format!(
    //     "{:?}",
    //     Dot::with_config(&nodes, &[Config::EdgeNoLabel])
    // );
    // let mut file = File::create("graph.dot")?;
    // file.write_all(dot_txt.as_bytes())?;

    let result = 0;

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = include_str!("../test_input1.txt");
        assert_eq!("102", process(input)?);
        Ok(())
    }
}
