use super::graph::{Graph, Rule, Bindings};
use logos::{Lexer, Logos};

pub type SyntaxError = String;
pub type Result = std::result::Result<Graph, Vec<SyntaxError>>;

#[derive(Logos, Debug, PartialEq)]
enum Token {
    #[error]
    Error,

    #[regex(r"\s+", logos::skip)]
    Whitespace,

    #[token("rule")]
    Rule,

    #[regex(r"[a-zA-Z0-9_.-]+")]
    Identifer,

    #[regex(r"=.+")]
    AssignmentValue,

    #[regex("\n")]
    Newline,
}


fn parse_variable_assignment(lexer: &mut Lexer<Token>, mut graph: Graph) -> Result {
    let variable_name = lexer.slice().to_string();

    if let Some(next_token) = lexer.next() {
        if next_token != Token::AssignmentValue {
            return Err(vec!("value assignment expected".to_string()));
        }
        let mut assigned_value = lexer.slice().to_string();
        assigned_value.remove(0);
        graph.variables.insert(variable_name, assigned_value);
        if let Some(expected_line_ending) = lexer.next() {
            if expected_line_ending != Token::Newline {
                return Err(vec!("newline expected after variable assignment".to_string()));
            }
        }
    }
    return Ok(graph);
}


fn parse_rule(lexer: &mut Lexer<Token>, mut graph: Graph) -> Result {
    if let Some(token) = lexer.next() {
        if token != Token::Identifer {
            return Err(vec!("identifier expected".to_string()));
        }
        let rule_name = lexer.slice().to_string();
        let new_rule = Rule {
            name: rule_name,
            variables: Bindings::new(),
        };
        if let Err(err) = graph.rule(new_rule) {
            return Err(vec!(err));
        }
        if let Some(next_token) = lexer.next() {
            if next_token != Token::Newline {
                return Err(vec!("newline expected after rule".to_string()));
            }
        }
        Ok(graph)
    }
    else {
        Err(vec!("identifier expected".to_string()))
    }
}


pub fn parse(text: &str) -> Result {
    let mut graph_from_text = Graph::new();
    let mut lexer: Lexer<Token> = Lexer::new(text);
    
    while let Some(token) = lexer.next() {
        match token {
            Token::Identifer => graph_from_text = parse_variable_assignment(&mut lexer, graph_from_text)?,
            Token::Rule => graph_from_text = parse_rule(&mut lexer, graph_from_text)?,
            _ => return Err(vec!("unexpected token".to_string())),
        }
    }

    Ok(graph_from_text)
}



#[test]
fn an_empty_file_is_a_valid_but_empty_graph() {
    let graph_from_empty_string = parse("").unwrap();
    assert!(graph_from_empty_string.is_empty());
}


#[test]
fn a_single_assignment_is_a_global_constant() {
    let graph_from_single_assignment = parse("x=1").unwrap();
    assert_eq!(
        graph_from_single_assignment.variables["x"],
        "1"
    );
}


#[test]
fn uniquely_named_assignments_separated_by_newlines_are_global_variables() {
    let graph_from_single_assignment = parse("x=1\ny=2").unwrap();
    assert_eq!(
        graph_from_single_assignment.variables["x"],
        "1"
    );
    assert_eq!(
        graph_from_single_assignment.variables["y"],
        "2"
    );
}


#[test]
fn rules_are_defined_by_keyword_name_and_an_indented_block_of_variables() {
    let rule_text = "rule cxx";
    let graph_for_rule = parse(rule_text).unwrap();
    assert_eq!(
        graph_for_rule.rules.contains_key("cxx"),
        true
    );
}