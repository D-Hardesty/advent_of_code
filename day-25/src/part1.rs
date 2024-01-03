use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    IResult,
    character::complete::{alpha1, line_ending, space1},
    multi::separated_list1,
    sequence::separated_pair,
};
use crate::custom_error::AocError;
use petgraph::{
    dot::{Config, Dot},
    prelude::*,
};
use rustworkx_core::connectivity::stoer_wagner_min_cut;

fn parse(input: &str
) -> IResult<&str, Vec<(&str, Vec<&str>)>> {
    separated_list1(
        line_ending,
        separated_pair(
            alpha1,
            tag(": "),
            separated_list1(space1, alpha1),
        ),
    )(input)
}

#[tracing::instrument]
#[allow(unused)]
pub fn process(
    _input: &str,
) -> miette::Result<String, AocError> {
    let (_, nodes) = parse(_input).expect("Unable to parse");
    let unique_nodes = nodes
        .iter()
        .flat_map(|(key, values)| {
            let mut vs = values.clone();
            vs.push(key);
            vs
        })
        .unique()
        .collect::<Vec<&str>>();

    let mut graph = UnGraph::<&str, u32>::default();

    let node_map: HashMap<&str, NodeIndex> = unique_nodes
        .iter()
        .map(|node| {
            (*node, graph.add_node(&node))
        })
        .collect();

    for (key, value) in nodes.iter() {
        for node in value {
            graph.add_edge(
                node_map[key],
                node_map[node],
                1,
            );
        }
    }

    let min: rustworkx_core::Result<
        Option<(usize, Vec<_>)>,
    > = stoer_wagner_min_cut(&graph, |_| Ok(1));

    let (_, nodes_in_partition) =
        min.unwrap().unwrap();
    let total_nodes = unique_nodes.len();

    // let dot_txt = format!(
    //     "{:?}",
    //     Dot::with_config(&graph, &[Config::EdgeNoLabel])
    // );
    // let mut file = File::create("graph.dot")?;
    // file.write_all(dot_txt.as_bytes())?;

    let result = (total_nodes - nodes_in_partition.len()) * nodes_in_partition.len();
    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = include_str!("../test_input1.txt");
        assert_eq!("54", process(input)?);
        Ok(())
    }
}