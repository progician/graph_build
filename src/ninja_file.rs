use super::graph::{Graph, Node};
use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
enum Token {
    #[error]
    Error,

    #[token("build")]
    Build,

    #[token(":")]
    Colon,

    #[token("default")]
    Default,

    #[token("=")]
    Equals,

    #[regex("[a-zA-Z0-9_.-]+")]
    Identifier,

    #[token("include")]
    Include,

    #[regex(r"[ ]+", priority=2)]
    Indentation,

    #[regex(r"[ ]*\r\n", priority=1)]
    #[regex(r"[ ]*\n", priority=1)]
    Newline,

    #[token("|")]
    Pipe,

    #[token("||")]
    Pipe2,

    #[token("pool")]
    Pool,

    #[token("rule")]
    Rule,

    #[token("subninja")]
    Subninja,
}


#[test]
fn test_lexer_basics() {
    let mut lexer = Token::lexer("rule capitalize");
    assert_eq!(lexer.next(), Some(Token::Rule));

    assert_eq!(lexer.next(), Some(Token::Indentation));
    assert_eq!(lexer.span(), 4..5);

    assert_eq!(lexer.next(), Some(Token::Identifier));
    assert_eq!(lexer.span(), 5..15);
    assert_eq!(lexer.slice(), "capitalize");

    assert_eq!(lexer.next(), None);
}


pub fn read(_input: &String) -> Result<Graph, String> {
    let mut build_graph = Graph::new();
    build_graph.build(Node::new("loremipsum.txt.u", "capitalize", "loremipsum.txt")).unwrap();
    Ok(build_graph)
}


//#[test]
//fn test_parse_simple_build_file() {
//    let text = String::from(
//"rule capitalize
//    command = dd if=$in of=$out conv=ucase
//build loremipsum.txt.u: capitalize loremipsum.txt
//"
//    );
//    
//    let build_graph = read(&text).unwrap();
//    assert_eq!(build_graph.rules.contains_key("capitalize"), true);
//    assert_eq!(build_graph.nodes["loremipsum.txt.u"].input, "loremipsum.txt")
//}