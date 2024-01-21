// Module contains the lexer for the ninja build file format.
use logos::Logos;


// The main lexer is based on this grammar:
//
// nul = "\000";
// simple_varname = [a-zA-Z0-9_-]+;
// varname = [a-zA-Z0-9_.-]+;
// 
// [ ]*"#"[^\000\n]*"\n" { continue; }
// [ ]*"\r\n" { token = NEWLINE;  break; }
// [ ]*"\n"   { token = NEWLINE;  break; }
// [ ]+       { token = INDENT;   break; }
// "build"    { token = BUILD;    break; }
// "pool"     { token = POOL;     break; }
// "rule"     { token = RULE;     break; }
// "default"  { token = DEFAULT;  break; }
// "="        { token = EQUALS;   break; }
// ":"        { token = COLON;    break; }
// "|@"       { token = PIPEAT;   break; }
// "||"       { token = PIPE2;    break; }
// "|"        { token = PIPE;     break; }
// "include"  { token = INCLUDE;  break; }
// "subninja" { token = SUBNINJA; break; }
// varname    { token = IDENT;    break; }
// nul        { token = TEOF;     break; }
// [^]        { token = ERROR;    break; }
#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t]*#[^\n]*\n")]
#[logos(skip r"[ \t]*")]
enum NinjaFileToken {
    #[regex(r"[ ]*\r\n")]
    #[regex(r"[ ]*\n")]
    Newline,

    #[regex(r"\n[ ]+")]
    Indent,

    #[token("build")]
    Build,

    #[regex(r"[a-zA-Z0-9_.-]+")]
    Identifier,

    #[token("pool")]
    Pool,

    #[token("rule")]
    Rule,

    #[token("default")]
    Default,

    #[regex("=[ ]*[^\n]+")]
    AssignmentValue,

    #[token(":")]
    Colon,

    #[token("|@")]
    PipeAt,

    #[token("||")]
    Pipe2,

    #[token("|")]
    Pipe,

    #[token("include")]
    Include,

    #[token("subninja")]
    Subninja,
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert2::assert;

    fn assert_token_stream(input: &str, expected: &[NinjaFileToken]) {
        let lexer = NinjaFileToken::lexer(input);

        let token_stream: Vec<NinjaFileToken> = lexer
            .map(|v| v.unwrap())
            .collect();

        assert!(token_stream == expected);
    }

    #[test]
    fn test_single_build_line() {
        assert_token_stream("build foo: bar", &[
            NinjaFileToken::Build,
            NinjaFileToken::Identifier,
            NinjaFileToken::Colon,
            NinjaFileToken::Identifier,
        ]);
    }

    #[test]
    fn test_global_assignment() {
        assert_token_stream("x=1", &[
            NinjaFileToken::Identifier,
            NinjaFileToken::AssignmentValue,
        ]);
    }
}
