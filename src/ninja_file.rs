use super::graph::{Graph, Rule, Bindings};
use logos::{Lexer, Logos};
use std::collections::VecDeque;

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
    Identifier,

    #[regex(r"=.+")]
    AssignmentValue,

    #[regex("\n")]
    Newline,
}


#[derive(PartialEq, Clone)]
enum TokenKind {
    Error,
    Whitespace,
    Rule,
    Identifier,
    AssignmentValue,
    Newline,
}


impl From<Token> for TokenKind {
    fn from(token: Token) -> Self {
        match token {
            Token::Error => TokenKind::Error,
            Token::Whitespace => TokenKind::Whitespace,
            Token::Rule => TokenKind::Rule,
            Token::Identifier => TokenKind::Identifier,
            Token::AssignmentValue => TokenKind::AssignmentValue,
            Token::Newline => TokenKind::Newline,
        }
    }
}


struct FullToken {
    kind: TokenKind,
    span: logos::Span,
}


struct NinjaLexer<'a> {
    inner: Lexer<'a, Token>,
    peeked: VecDeque<FullToken>,
    span: Option<logos::Span>,
    done: bool,
}


impl<'a> NinjaLexer<'a> {
    fn new(text: &'a str) -> Self {
        Self {
            inner: Lexer::new(text),
            peeked: VecDeque::new(),
            span: None,
            done: false,
        }
    }

    fn slice(&self) -> &str {
        unsafe {
            let sp = self.span.as_ref().unwrap();
            self.inner.source().get_unchecked( sp.start..sp.end)
        }
    }

    fn peek(&mut self) -> Option<TokenKind> {
        self.peek_nth(0)
    }

    fn peek_nth(&mut self, n: usize) -> Option<TokenKind> {
        while self.peeked.len() <= n && !self.done {
            if let Some(token) = self.inner.next() {
                self.peeked.push_back(FullToken {
                    kind: TokenKind::from(token),
                    span: self.inner.span(),
                });
            }
            else {
                self.done = true;
            }
        }

        match self.peeked.get(n) {
            Some(full_token) => Some(full_token.kind.clone()),
            None => None,
        }
    }
}


impl<'a> Iterator for NinjaLexer<'a> {
    type Item = TokenKind;
    
    fn next(&mut self) -> Option<Self::Item> {
        match self.peeked.pop_front() {
            Some(token) => {
                self.span = Some(token.span);
                Some(token.kind)
            },
            None => {
                let result = match self.inner.next() {
                    Some(token) => Some(TokenKind::from(token)),
                    None => {
                        self.done = true;
                        None
                    },
                };
                self.span = Some(self.inner.span());
                result
            },
        }
    }
}


fn expect_token(lexer: &mut NinjaLexer, expectation: TokenKind) -> std::result::Result<(), Vec<SyntaxError>> {
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


fn parse_global_variable_assignment(lexer: &mut NinjaLexer, mut graph: Graph) -> Result {
    let variable_name = lexer.slice().to_string();

    if let Some(next_token) = lexer.next() {
        if next_token != TokenKind::AssignmentValue {
            return Err(vec!("value assignment expected".to_string()));
        }
        let mut assigned_value = lexer.slice().to_string();
        assigned_value.remove(0);
        graph.variables.insert(variable_name, assigned_value);
        if let Some(expected_line_ending) = lexer.next() {
            if expected_line_ending != TokenKind::Newline {
                return Err(vec!("newline expected after variable assignment".to_string()));
            }
        }
    }
    return Ok(graph);
}


fn parse_indented_variable_block(lexer: &mut NinjaLexer) -> std::result::Result<Bindings, Vec<SyntaxError>> {
    let mut variables = Bindings::new();
    while let Some(token) = lexer.peek() {
        if token != TokenKind::Whitespace {
            return Ok(variables);
        }

        expect_token(lexer, TokenKind::Whitespace)?;
        expect_token(lexer, TokenKind::Identifier)?;
        let variable_name = lexer.slice().to_string();

        expect_token(lexer, TokenKind::AssignmentValue)?;
        let mut assigned_value = lexer.slice().to_string();
        assigned_value.remove(0);
        variables.insert(variable_name, assigned_value);

        expect_token(lexer, TokenKind::Newline)?;
    }
    Ok(variables)
}


fn parse_rule(lexer: &mut NinjaLexer, mut graph: Graph) -> Result {
    expect_token(lexer, TokenKind::Whitespace)?;
    expect_token(lexer, TokenKind::Identifier)?;

    let rule_name = lexer.slice().to_string();
    expect_token(lexer, TokenKind::Newline)?;

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
    let mut lexer = NinjaLexer::new(text);
    
    while let Some(token) = lexer.next() {
        match token {
            TokenKind::Identifier => graph_from_text = parse_global_variable_assignment(&mut lexer, graph_from_text)?,
            TokenKind::Rule => graph_from_text = parse_rule(&mut lexer, graph_from_text)?,
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


#[test]
fn variable_block_for_rule_end_when_no_indented_line_follows() {
    const VARIABLE_VALUE: &str = "gcc $in -o $out";
    let rule_text =
format!("rule cc
    command={}
command={}
", VARIABLE_VALUE, VARIABLE_VALUE);
    let graph_for_rule = parse(&rule_text).unwrap();
    assert_eq!(
        graph_for_rule.rules["cc"].variables["command"],
        VARIABLE_VALUE
    );
    assert_eq!(
        graph_for_rule.variables["command"],
        VARIABLE_VALUE
    );
}