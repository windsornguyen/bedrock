use::bedrock::lexer::Token;
use::logos::Logos;


fn main() {
    let source = r#"
        struct MyStruct {
            let x: int = 42;
            let name: string = "example";
        }

        fn main() {
            let y = 3.14;
            let mut z = true;
            // This is a line comment
            /* This is a
               block comment */
        }
    "#;

    let mut lex = Token::lexer(source);
    while let Some(token) = lex.next() {
        match token {
            Ok(token) => println!("{:?}: {:?}", token, lex.slice()),
            Err(_) => println!("Lexing error at position {:?}", lex.span()),
        }
    }
}
