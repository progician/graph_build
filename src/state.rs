use super::graph;
use std::collections::HashMap;

pub struct State {
    rules: HashMap<String, graph::Rule>,
    nodes: HashMap<String, graph::Node>,
}

impl State {
    pub fn new() -> Self {
        State {
            rules: HashMap::new(),
            nodes: HashMap::new(),
        }
    }


    pub fn rule(&mut self, rule: graph::Rule) -> Result<(), String> {
        if !self.rules.contains_key(&rule.name) {
            self.rules.insert(rule.name.clone(), rule);
            Ok(())
        }
        else {
            Err(format!("duplicate rule '{}'", &rule.name))
        }
    }


    pub fn build(&mut self, node: graph::Node) -> Result<(), String> {
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
    let mut state = State::new();
    state.rule(graph::Rule::new(String::from("cc"), String::from("gcc -c $in -o $out"))).unwrap();
    assert_eq!(
        state.rule(graph::Rule::new(String::from("cc"), String::from("clang -c $in -o $out"))),
        Err(String::from("duplicate rule 'cc'"))
    );
}

#[test]
fn cannot_add_duplicate_build_rule() {
    let mut state = State::new();
    state.build(graph::Node::new(String::from("foo.o"), String::from("cc"), String::from("gcc foo.cpp"))).unwrap();
    assert_eq!(
        state.build(graph::Node::new(String::from("foo.o"), String::from("cc"), String::from("gcc foo.cpp"))),
        Err(String::from("multiple rules generate 'foo.o'"))
    );
}