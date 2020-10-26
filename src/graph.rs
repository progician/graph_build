pub struct Node {
    pub name: String,
    pub input: String,
    pub output: String,
}


impl Node {
    pub fn new(name: String, input: String, output: String) -> Self {
        Node {
            name: name,
            input: input,
            output: output,
        }
    }
}


pub struct Rule {
    pub name: String,
    pub command: String,
}


impl Rule {
    pub fn new(name: String, command: String) -> Self {
        Rule {
            name: name,
            command: command,
        }
    }
}


pub type Graph = Option<Node>;

pub fn input_files(g: &Graph) -> Vec<&str> {
    match g {
        None => vec!(),
        Some(n) => vec!(&n.input),
    }
}

#[test]
fn empty_graph_has_no_input() {
    let n = Graph::None;
    assert_eq!(input_files(&n).is_empty(), true);
}


#[test]
fn single_node_has_single_output() {
    let n = Graph::Some(Node::new(
        String::from("some_node"),
        String::from("Something"),
        String::new()
    ));
    assert_eq!(input_files(&n).len(), 1);
}