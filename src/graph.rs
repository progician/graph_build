use std::collections::HashMap;


pub struct Node {
    pub output: String,
    pub rule: String,
    pub input: String,
}


impl Node {
    pub fn new<Stringish: AsRef<str>>(output: Stringish, rule: Stringish, input: Stringish) -> Self {
        Node {
            input: input.as_ref().to_string(),
            rule: rule.as_ref().to_string(),
            output: output.as_ref().to_string(),
        }
    }
}


pub struct Rule {
    pub name: String,
    pub command: String,
}

impl Rule {
    pub fn new<Stringish: AsRef<str>>(name: Stringish, command: Stringish) -> Self {
        Rule {
            name: name.as_ref().to_string(),
            command: command.as_ref().to_string(),
        }
    }
}


pub struct Graph {
    pub rules: HashMap<String, Rule>,
    pub nodes: HashMap<String, Node>,
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
        if !self.nodes.contains_key(&node.output) {
            self.nodes.insert(node.output.clone(), node);
            Ok(())
        }
        else {
            Err(format!("multiple rules generate '{}'", &node.output))
        }
    }
}


#[test]
fn cannot_add_duplicate_rule() {
    let mut graph = Graph::new();
    graph.rule(Rule::new("cc", "gcc -c $in -o $out")).unwrap();
    assert_eq!(
        graph.rule(Rule::new("cc", "clang -c $in -o $out")),
        Err(String::from("duplicate rule 'cc'"))
    );
}

#[test]
fn cannot_add_duplicate_build_rule() {
    let mut graph = Graph::new();
    graph.build(Node::new("foo.o", "cc", "foo.cpp")).unwrap();
    assert_eq!(
        graph.build(Node::new("foo.o", "cxx", "bar.cpp")),
        Err(String::from("multiple rules generate 'foo.o'"))
    );
}