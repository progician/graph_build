use super::graph::{Graph, Node};

pub fn read(_input: &String) -> Result<Graph, String> {
    let mut build_graph = Graph::new();
    build_graph.build(Node::new("loremipsum.txt.u", "capitalize", "loremipsum.txt")).unwrap();
    Ok(build_graph)
}