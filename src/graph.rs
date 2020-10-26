use std::collections::HashMap;


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


pub struct Graph {
    rules: HashMap<String, Rule>,
    nodes: HashMap<String, Node>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            rules: HashMap::new(),
            nodes: HashMap::new(),
        }
    }


    pub fn rule(&mut self, rule: Rule) -> Result<(), String> {
        if !self.rules.contains_key(&rule.name) {
            self.rules.insert(rule.name.clone(), rule);
            Ok(())
        }
        else {
            Err(format!("duplicate rule '{}'", &rule.name))
        }
    }


    pub fn build(&mut self, node: Node) -> Result<(), String> {
        if !self.nodes.contains_key(&node.name) {
            self.nodes.insert(node.name.clone(), node);
            Ok(())
        }
        else {
            Err(format!("multiple rules generate '{}'", &node.name))
        }
    }
}


#[test]
fn cannot_add_duplicate_rule() {
    let mut graph = Graph::new();
    graph.rule(Rule::new(String::from("cc"), String::from("gcc -c $in -o $out"))).unwrap();
    assert_eq!(
        graph.rule(Rule::new(String::from("cc"), String::from("clang -c $in -o $out"))),
        Err(String::from("duplicate rule 'cc'"))
    );
}

#[test]
fn cannot_add_duplicate_build_rule() {
    let mut graph = Graph::new();
    graph.build(Node::new(String::from("foo.o"), String::from("cc"), String::from("gcc foo.cpp"))).unwrap();
    assert_eq!(
        graph.build(Node::new(String::from("foo.o"), String::from("cc"), String::from("gcc foo.cpp"))),
        Err(String::from("multiple rules generate 'foo.o'"))
    );
}