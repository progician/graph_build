use super::graph::{Graph, Rule, Bindings};
use logos::{Lexer, Logos};

pub type SyntaxError = String;
pub type Result = std::result::Result<Graph, Vec<SyntaxError>>;

#[derive(Logos, Debug, PartialEq)]
enum Token {
    #[error]
    Error,

    #[regex(r"[ ]+")]
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


fn expect_token(lexer: &mut Lexer<Token>, expectation: Token) -> std::result::Result<(), Vec<SyntaxError>> {
    if let Some(token) = lexer.next() {
        if token == expectation {
            Ok(())
        }
        else {
            Err(vec!(format!(
                "unexpected token {}", lexer.slice()
            )))
        }
    }
    else {
        Err(vec!("unexpected end of file".to_string()))
    }
}


fn parse_global_variable_assignment(lexer: &mut Lexer<Token>, mut graph: Graph) -> Result {
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


fn parse_indented_variable_block(lexer: &mut Lexer<Token>) -> std::result::Result<Bindings, Vec<SyntaxError>> {
    let mut variables = Bindings::new();
    while let Some(token) = lexer.next() {
        if token != Token::Whitespace {
            return Ok(variables);
        }

        expect_token(lexer, Token::Identifer)?;
        let variable_name = lexer.slice().to_string();

        expect_token(lexer, Token::AssignmentValue)?;
        let mut assigned_value = lexer.slice().to_string();
        assigned_value.remove(0);
        variables.insert(variable_name, assigned_value);

        expect_token(lexer, Token::Newline)?;
    }
    Ok(variables)
}


fn parse_rule(lexer: &mut Lexer<Token>, mut graph: Graph) -> Result {
    expect_token(lexer, Token::Whitespace)?;
    expect_token(lexer, Token::Identifer)?;

    let rule_name = lexer.slice().to_string();
    expect_token(lexer, Token::Newline)?;

    let rule_bindings = parse_indented_variable_block(lexer)?;

    let new_rule = Rule {
        name: rule_name,
        variables: rule_bindings,
    };
    if let Err(err) = graph.rule(new_rule) {
        return Err(vec!(err));
    }
    
    Ok(graph)
}


pub fn parse(text: &str) -> Result {
    let mut graph_from_text = Graph::new();
    let mut lexer: Lexer<Token> = Lexer::new(text);
    
    while let Some(token) = lexer.next() {
        match token {
            Token::Identifer => graph_from_text = parse_global_variable_assignment(&mut lexer, graph_from_text)?,
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
fn rules_are_defined_by_keyword_and_identifier() {
    const RULE_TEXT: &str =
"rule cc
";
    let graph_for_rule = parse(RULE_TEXT).unwrap();
    assert_eq!(
        graph_for_rule.rules.contains_key("cc"),
        true
    );
}


#[test]
fn indented_variable_block_after_rule_are_variables_of_the_rule() {
    const VARIABLE_VALUE: &str = "gcc $in -o $out";
    let rule_text =
format!("rule cc
    command={}
", VARIABLE_VALUE);
    let graph_for_rule = parse(&rule_text).unwrap();
    assert_eq!(
        graph_for_rule.rules["cc"].variables["command"],
        VARIABLE_VALUE
    );
}
