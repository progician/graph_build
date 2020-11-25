use super::graph::Graph;
use logos::{Lexer, Logos};

pub type SyntaxError = String;
pub type Result = std::result::Result<Graph, Vec<SyntaxError>>;

#[derive(Logos, Debug, PartialEq)]
enum Token {
    #[regex(r"[a-zA-Z0-9_.-]+")]
    Identifer,

    #[regex(r"=.+")]
    AssignmentValue,

    #[regex("\n")]
    Newline,

    #[regex(r"[ ]+")]
    #[error]
    Whitespace,
}


pub fn parse(text: &str) -> Result {
    let mut graph_from_text = Graph::new();
    let mut lexer: Lexer<Token> = Lexer::new(text);
    
    while let Some(t) = lexer.next() {
        if t != Token::Identifer {
            return Err(vec!("identifier expected".to_string()));
        }
        let variable_name = lexer.slice().to_string();
        
        if let Some(next_token) = lexer.next() {
            if next_token != Token::AssignmentValue {
                return Err(vec!("value assignment expected".to_string()));
            }
            let mut assigned_value = lexer.slice().to_string();
            assigned_value.remove(0);
            graph_from_text.variables.insert(variable_name, assigned_value);
            if let Some(expected_line_ending) = lexer.next() {
                if expected_line_ending != Token::Newline {
                    return Err(vec!("newline expected after variable assignment".to_string()));
                }
            }
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