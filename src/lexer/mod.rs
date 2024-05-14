use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")] // Skip whitespace and newlines
pub enum Token {
    #[regex(r"//[^\n]*", logos::skip)]
    LineComment,
    #[regex(r"/\*", handle_nested_block_comment)]
    NestedBlockComment,
    #[regex(r"#.*", logos::skip)]
    HashComment,

    #[token("struct")]
    Struct,
    #[token("null")]
    Null,
    #[token("void")]
    TVoid,
    #[token("int")]
    TInt,
    #[token("float")]
    TFloat,
    #[token("string")]
    TString,
    #[token("boolean")]
    TBoolean,
    #[token("else")]
    Else,
    #[token("if")]
    If,
    #[token("while")]
    While,
    #[token("for")]
    For,
    #[token("return")]
    Return,
    #[token("let")]
    Let,
    #[token("mut")]
    Mut,
    #[token("fn")]
    Fn,
    #[token("true")]
    True,
    #[token("false")]
    False,

    #[token(".")]
    Dot,
    #[token(";")]
    Semi,
    #[token(",")]
    Comma,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
    #[token("+")]
    Plus,
    #[token("-")]
    Dash,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,
    #[token("%")]
    Percent,
    #[token("=")]
    Eq,
    #[token("==")]
    EqEq,
    #[token("!")]
    Bang,
    #[token("!=")]
    BangEq,
    #[token("<")]
    LT,
    #[token("<=")]
    LTEQ,
    #[token(">")]
    GT,
    #[token(">=")]
    GTEQ,
    #[token("&&")]
    AndAnd,
    #[token("||")]
    OrOr,
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("[")]
    LBracket,
    #[token("]")]
    RBracket,
    #[token("->")]
    Arrow,

    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Ident(String),

    #[regex(r"[0-9]+(\.[0-9]+)?", |lex| lex.slice().parse().unwrap_or(0.0))]
    Number(f64),

    #[regex(r#""([^"\\]|\\.)*""#, |lex| parse_string(lex.slice()))]
    String(String),
}

fn handle_nested_block_comment(lex: &mut logos::Lexer<Token>) -> logos::Skip {
    let mut depth = 1;

    while let Some(c) = lex.remainder().chars().next() {
        lex.bump(1);

        match c {
            '/' => {
                if let Some('*') = lex.remainder().chars().next() {
                    lex.bump(1);
                    depth += 1;
                }
            }
            '*' => {
                if let Some('/') = lex.remainder().chars().next() {
                    lex.bump(1);
                    depth -= 1;
                    if depth == 0 {
                        break;
                    }
                }
            }
            _ => {}
        }
    }

    logos::Skip
}

fn parse_string(lexeme: &str) -> String {
    let mut result = String::new();
    let mut chars = lexeme.chars();
    chars.next(); // Skip the opening quote
    while let Some(c) = chars.next() {
        match c {
            '\\' => {
                if let Some(escaped) = chars.next() {
                    match escaped {
                        'n' => result.push('\n'),
                        't' => result.push('\t'),
                        '\\' => result.push('\\'),
                        '"' => result.push('"'),
                        _ => result.push(escaped),
                    }
                }
            }
            '"' => break, // End of string
            _ => result.push(c),
        }
    }
    result
}
