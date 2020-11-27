use std::collections::HashMap;


pub struct Edge {
    pub output: String,
    pub rule: String,
    pub input: String,
}


impl Edge {
    pub fn new<Stringish: AsRef<str>>(output: Stringish, rule: Stringish, input: Stringish) -> Self {
        Edge {
            input: input.as_ref().to_string(),
            rule: rule.as_ref().to_string(),
            output: output.as_ref().to_string(),
        }
    }
}


pub type Bindings = HashMap<String, String>;


pub struct Rule {
    pub name: String,
    pub variables: Bindings,
}

impl Rule {
    pub fn new<Stringish: AsRef<str>>(name: Stringish, variables: Bindings) -> Self {
        Rule {
            name: name.as_ref().to_string(),
            variables: variables,
        }
    }
}


pub struct Graph {
    pub rules: HashMap<String, Rule>,
    pub edges: HashMap<String, Edge>,
    pub variables: Bindings,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            rules: HashMap::new(),
            edges: HashMap::new(),
            variables: Bindings::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.rules.is_empty() && self.edges.is_empty()
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


    pub fn build(&mut self, edge: Edge) -> Result<(), String> {
        if !self.edges.contains_key(&edge.output) {
            self.edges.insert(edge.output.clone(), edge);
            Ok(())
        }
        else {
            Err(format!("multiple rules generate '{}'", &edge.output))
        }
    }
}


#[test]
fn cannot_add_duplicate_rule() {
    let mut graph = Graph::new();
    graph.rule(Rule::new("cc", HashMap::new())).unwrap();
    assert_eq!(
        graph.rule(Rule::new("cc", HashMap::new())),
        Err(String::from("duplicate rule 'cc'"))
    );
}

#[test]
fn cannot_add_duplicate_build_rule() {
    let mut graph = Graph::new();
    graph.build(Edge::new("foo.o", "cc", "foo.cpp")).unwrap();
    assert_eq!(
        graph.build(Edge::new("foo.o", "cxx", "bar.cpp")),
        Err(String::from("multiple rules generate 'foo.o'"))
    );
}