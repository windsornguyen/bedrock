use bedrock::lexer::Token;
use logos::Logos;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keywords() {
        let source = "struct null void int float string boolean else if while for return let mut fn true false";
        let mut lex = Token::lexer(source);

        assert_eq!(lex.next(), Some(Ok(Token::Struct)));
        assert_eq!(lex.next(), Some(Ok(Token::Null)));
        assert_eq!(lex.next(), Some(Ok(Token::TVoid)));
        assert_eq!(lex.next(), Some(Ok(Token::TInt)));
        assert_eq!(lex.next(), Some(Ok(Token::TFloat)));
        assert_eq!(lex.next(), Some(Ok(Token::TString)));
        assert_eq!(lex.next(), Some(Ok(Token::TBoolean)));
        assert_eq!(lex.next(), Some(Ok(Token::Else)));
        assert_eq!(lex.next(), Some(Ok(Token::If)));
        assert_eq!(lex.next(), Some(Ok(Token::While)));
        assert_eq!(lex.next(), Some(Ok(Token::For)));
        assert_eq!(lex.next(), Some(Ok(Token::Return)));
        assert_eq!(lex.next(), Some(Ok(Token::Let)));
        assert_eq!(lex.next(), Some(Ok(Token::Mut)));
        assert_eq!(lex.next(), Some(Ok(Token::Fn)));
        assert_eq!(lex.next(), Some(Ok(Token::True)));
        assert_eq!(lex.next(), Some(Ok(Token::False)));
        assert_eq!(lex.next(), None);
    }

    #[test]
    fn test_identifiers() {
        let source = "myVar another_var _hidden";
        let mut lex = Token::lexer(source);

        assert_eq!(lex.next(), Some(Ok(Token::Ident("myVar".to_string()))));
        assert_eq!(
            lex.next(),
            Some(Ok(Token::Ident("another_var".to_string())))
        );
        assert_eq!(lex.next(), Some(Ok(Token::Ident("_hidden".to_string()))));
        assert_eq!(lex.next(), None);
    }

    #[test]
    fn test_numbers() {
        let source = "42 3.14 0 12345 1.0";
        let mut lex = Token::lexer(source);

        assert_eq!(lex.next(), Some(Ok(Token::Number(42.0))));
        assert_eq!(lex.next(), Some(Ok(Token::Number(3.14))));
        assert_eq!(lex.next(), Some(Ok(Token::Number(0.0))));
        assert_eq!(lex.next(), Some(Ok(Token::Number(12345.0))));
        assert_eq!(lex.next(), Some(Ok(Token::Number(1.0))));
        assert_eq!(lex.next(), None);
    }

    #[test]
    fn test_strings() {
        let source = r#""hello" "world" "escaped \n \t \\""#;
        let mut lex = Token::lexer(source);

        assert_eq!(lex.next(), Some(Ok(Token::String("hello".to_string()))));
        assert_eq!(lex.next(), Some(Ok(Token::String("world".to_string()))));
        assert_eq!(
            lex.next(),
            Some(Ok(Token::String("escaped \n \t \\".to_string())))
        );
        assert_eq!(lex.next(), None);
    }

    #[test]
    fn test_line_comments() {
        let source = r#"// this is a comment
        let x = 42;"#;
        let mut lex = Token::lexer(source);

        assert_eq!(lex.next(), Some(Ok(Token::Let)));
        assert_eq!(lex.next(), Some(Ok(Token::Ident("x".to_string()))));
        assert_eq!(lex.next(), Some(Ok(Token::Eq)));
        assert_eq!(lex.next(), Some(Ok(Token::Number(42.0))));
        assert_eq!(lex.next(), Some(Ok(Token::Semi)));
        assert_eq!(lex.next(), None);
    }

    #[test]
    fn test_block_comments() {
        let source = r#"/* this is a block comment */
        let x = 42;"#;
        let mut lex = Token::lexer(source);

        assert_eq!(lex.next(), Some(Ok(Token::Let)));
        assert_eq!(lex.next(), Some(Ok(Token::Ident("x".to_string()))));
        assert_eq!(lex.next(), Some(Ok(Token::Eq)));
        assert_eq!(lex.next(), Some(Ok(Token::Number(42.0))));
        assert_eq!(lex.next(), Some(Ok(Token::Semi)));
        assert_eq!(lex.next(), None);
    }

    #[test]
    fn test_nested_block_comments() {
        let source = r#"/* this is a /* nested */ block comment */
        let x = 42;"#;
        let mut lex = Token::lexer(source);

        assert_eq!(lex.next(), Some(Ok(Token::Let)));
        assert_eq!(lex.next(), Some(Ok(Token::Ident("x".to_string()))));
        assert_eq!(lex.next(), Some(Ok(Token::Eq)));
        assert_eq!(lex.next(), Some(Ok(Token::Number(42.0))));
        assert_eq!(lex.next(), Some(Ok(Token::Semi)));
        assert_eq!(lex.next(), None);
    }

    #[test]
    fn test_mixed_comments() {
        let source = r#"
        // This is a single-line comment
        /* This is a block comment */
        let x = 42; // Inline comment after a statement
        let y = 3.14; /* Inline block comment */
        # This is a hash comment
        /* Nested comments example /* nested */ still in block comment */
        let z = x + y; // Another inline comment
        "#;
        let mut lex = Token::lexer(source);

        assert_eq!(lex.next(), Some(Ok(Token::Let)));
        assert_eq!(lex.next(), Some(Ok(Token::Ident("x".to_string()))));
        assert_eq!(lex.next(), Some(Ok(Token::Eq)));
        assert_eq!(lex.next(), Some(Ok(Token::Number(42.0))));
        assert_eq!(lex.next(), Some(Ok(Token::Semi)));
        assert_eq!(lex.next(), Some(Ok(Token::Let)));
        assert_eq!(lex.next(), Some(Ok(Token::Ident("y".to_string()))));
        assert_eq!(lex.next(), Some(Ok(Token::Eq)));
        assert_eq!(lex.next(), Some(Ok(Token::Number(3.14))));
        assert_eq!(lex.next(), Some(Ok(Token::Semi)));
        assert_eq!(lex.next(), Some(Ok(Token::Let)));
        assert_eq!(lex.next(), Some(Ok(Token::Ident("z".to_string()))));
        assert_eq!(lex.next(), Some(Ok(Token::Eq)));
        assert_eq!(lex.next(), Some(Ok(Token::Ident("x".to_string()))));
        assert_eq!(lex.next(), Some(Ok(Token::Plus)));
        assert_eq!(lex.next(), Some(Ok(Token::Ident("y".to_string()))));
        assert_eq!(lex.next(), Some(Ok(Token::Semi)));
        assert_eq!(lex.next(), None);
    }
    
    #[test]
    fn test_interleaved_mixed_comments() {
        let source = r#"
        /* outer // line comment inside block
        /* level 1 // another line comment
        # hash comment in nested block
        /* level 2 */
        still level 1 */
        outer */
        let x = 42; // code after comments
        "#;
        let mut lex = Token::lexer(source);

        assert_eq!(lex.next(), Some(Ok(Token::Let)));
        assert_eq!(lex.next(), Some(Ok(Token::Ident("x".to_string()))));
        assert_eq!(lex.next(), Some(Ok(Token::Eq)));
        assert_eq!(lex.next(), Some(Ok(Token::Number(42.0))));
        assert_eq!(lex.next(), Some(Ok(Token::Semi)));
        assert_eq!(lex.next(), None);
    }

    #[test]
    fn test_operators() {
        let source = "+ - * / % = == ! != < <= > >= && || . ; , { } ( ) [ ] ->";
        let mut lex = Token::lexer(source);

        assert_eq!(lex.next(), Some(Ok(Token::Plus)));
        assert_eq!(lex.next(), Some(Ok(Token::Dash)));
        assert_eq!(lex.next(), Some(Ok(Token::Star)));
        assert_eq!(lex.next(), Some(Ok(Token::Slash)));
        assert_eq!(lex.next(), Some(Ok(Token::Percent)));
        assert_eq!(lex.next(), Some(Ok(Token::Eq)));
        assert_eq!(lex.next(), Some(Ok(Token::EqEq)));
        assert_eq!(lex.next(), Some(Ok(Token::Bang)));
        assert_eq!(lex.next(), Some(Ok(Token::BangEq)));
        assert_eq!(lex.next(), Some(Ok(Token::LT)));
        assert_eq!(lex.next(), Some(Ok(Token::LTEQ)));
        assert_eq!(lex.next(), Some(Ok(Token::GT)));
        assert_eq!(lex.next(), Some(Ok(Token::GTEQ)));
        assert_eq!(lex.next(), Some(Ok(Token::AndAnd)));
        assert_eq!(lex.next(), Some(Ok(Token::OrOr)));
        assert_eq!(lex.next(), Some(Ok(Token::Dot)));
        assert_eq!(lex.next(), Some(Ok(Token::Semi)));
        assert_eq!(lex.next(), Some(Ok(Token::Comma)));
        assert_eq!(lex.next(), Some(Ok(Token::LBrace)));
        assert_eq!(lex.next(), Some(Ok(Token::RBrace)));
        assert_eq!(lex.next(), Some(Ok(Token::LParen)));
        assert_eq!(lex.next(), Some(Ok(Token::RParen)));
        assert_eq!(lex.next(), Some(Ok(Token::LBracket)));
        assert_eq!(lex.next(), Some(Ok(Token::RBracket)));
        assert_eq!(lex.next(), Some(Ok(Token::Arrow)));
        assert_eq!(lex.next(), None);
    }

    #[test]
    fn test_statements_with_newlines_and_semicolons() {
        let source = r#"
        let x = 42;
        let y = 3.14
        return x + y;
        "#;
        let mut lex = Token::lexer(source);

        assert_eq!(lex.next(), Some(Ok(Token::Let)));
        assert_eq!(lex.next(), Some(Ok(Token::Ident("x".to_string()))));
        assert_eq!(lex.next(), Some(Ok(Token::Eq)));
        assert_eq!(lex.next(), Some(Ok(Token::Number(42.0))));
        assert_eq!(lex.next(), Some(Ok(Token::Semi)));
        assert_eq!(lex.next(), Some(Ok(Token::Let)));
        assert_eq!(lex.next(), Some(Ok(Token::Ident("y".to_string()))));
        assert_eq!(lex.next(), Some(Ok(Token::Eq)));
        assert_eq!(lex.next(), Some(Ok(Token::Number(3.14))));
        assert_eq!(lex.next(), Some(Ok(Token::Return)));
        assert_eq!(lex.next(), Some(Ok(Token::Ident("x".to_string()))));
        assert_eq!(lex.next(), Some(Ok(Token::Plus)));
        assert_eq!(lex.next(), Some(Ok(Token::Ident("y".to_string()))));
        assert_eq!(lex.next(), Some(Ok(Token::Semi)));
        assert_eq!(lex.next(), None);
    }

    #[test]
    fn test_complex_nested_expressions() {
        let source = r#"
        let result = ((1 + 2) * (3 / 4)) % 5 == 0 && !true || false;
        "#;
        let mut lex = Token::lexer(source);

        assert_eq!(lex.next(), Some(Ok(Token::Let)));
        assert_eq!(lex.next(), Some(Ok(Token::Ident("result".to_string()))));
        assert_eq!(lex.next(), Some(Ok(Token::Eq)));
        assert_eq!(lex.next(), Some(Ok(Token::LParen)));
        assert_eq!(lex.next(), Some(Ok(Token::LParen)));
        assert_eq!(lex.next(), Some(Ok(Token::Number(1.0))));
        assert_eq!(lex.next(), Some(Ok(Token::Plus)));
        assert_eq!(lex.next(), Some(Ok(Token::Number(2.0))));
        assert_eq!(lex.next(), Some(Ok(Token::RParen)));
        assert_eq!(lex.next(), Some(Ok(Token::Star)));
        assert_eq!(lex.next(), Some(Ok(Token::LParen)));
        assert_eq!(lex.next(), Some(Ok(Token::Number(3.0))));
        assert_eq!(lex.next(), Some(Ok(Token::Slash)));
        assert_eq!(lex.next(), Some(Ok(Token::Number(4.0))));
        assert_eq!(lex.next(), Some(Ok(Token::RParen)));
        assert_eq!(lex.next(), Some(Ok(Token::RParen)));
        assert_eq!(lex.next(), Some(Ok(Token::Percent)));
        assert_eq!(lex.next(), Some(Ok(Token::Number(5.0))));
        assert_eq!(lex.next(), Some(Ok(Token::EqEq)));
        assert_eq!(lex.next(), Some(Ok(Token::Number(0.0))));
        assert_eq!(lex.next(), Some(Ok(Token::AndAnd)));
        assert_eq!(lex.next(), Some(Ok(Token::Bang)));
        assert_eq!(lex.next(), Some(Ok(Token::True)));
        assert_eq!(lex.next(), Some(Ok(Token::OrOr)));
        assert_eq!(lex.next(), Some(Ok(Token::False)));
        assert_eq!(lex.next(), Some(Ok(Token::Semi)));
        assert_eq!(lex.next(), None);
    }

    #[test]
    fn test_combined_syntax() {
        let source = r#"
    struct MyStruct {
        let a = 1;
        let b = 2;
        let sum = a + b;
        if (sum == 3) {
            return true;
        } else {
            return false;
        }
    }

    fn main() {
        let x = MyStruct;
        return x.sum;
    }
    "#;
        let mut lex = Token::lexer(source);

        assert_eq!(lex.next(), Some(Ok(Token::Struct)));
        assert_eq!(lex.next(), Some(Ok(Token::Ident("MyStruct".to_string()))));
        assert_eq!(lex.next(), Some(Ok(Token::LBrace)));
        assert_eq!(lex.next(), Some(Ok(Token::Let)));
        assert_eq!(lex.next(), Some(Ok(Token::Ident("a".to_string()))));
        assert_eq!(lex.next(), Some(Ok(Token::Eq)));
        assert_eq!(lex.next(), Some(Ok(Token::Number(1.0))));
        assert_eq!(lex.next(), Some(Ok(Token::Semi)));
        assert_eq!(lex.next(), Some(Ok(Token::Let)));
        assert_eq!(lex.next(), Some(Ok(Token::Ident("b".to_string()))));
        assert_eq!(lex.next(), Some(Ok(Token::Eq)));
        assert_eq!(lex.next(), Some(Ok(Token::Number(2.0))));
        assert_eq!(lex.next(), Some(Ok(Token::Semi)));
        assert_eq!(lex.next(), Some(Ok(Token::Let)));
        assert_eq!(lex.next(), Some(Ok(Token::Ident("sum".to_string()))));
        assert_eq!(lex.next(), Some(Ok(Token::Eq)));
        assert_eq!(lex.next(), Some(Ok(Token::Ident("a".to_string()))));
        assert_eq!(lex.next(), Some(Ok(Token::Plus)));
        assert_eq!(lex.next(), Some(Ok(Token::Ident("b".to_string()))));
        assert_eq!(lex.next(), Some(Ok(Token::Semi)));
        assert_eq!(lex.next(), Some(Ok(Token::If)));
        assert_eq!(lex.next(), Some(Ok(Token::LParen)));
        assert_eq!(lex.next(), Some(Ok(Token::Ident("sum".to_string()))));
        assert_eq!(lex.next(), Some(Ok(Token::EqEq)));
        assert_eq!(lex.next(), Some(Ok(Token::Number(3.0))));
        assert_eq!(lex.next(), Some(Ok(Token::RParen)));
        assert_eq!(lex.next(), Some(Ok(Token::LBrace)));
        assert_eq!(lex.next(), Some(Ok(Token::Return)));
        assert_eq!(lex.next(), Some(Ok(Token::True)));
        assert_eq!(lex.next(), Some(Ok(Token::Semi)));
        assert_eq!(lex.next(), Some(Ok(Token::RBrace))); // End of if block
        assert_eq!(lex.next(), Some(Ok(Token::Else)));
        assert_eq!(lex.next(), Some(Ok(Token::LBrace)));
        assert_eq!(lex.next(), Some(Ok(Token::Return)));
        assert_eq!(lex.next(), Some(Ok(Token::False)));
        assert_eq!(lex.next(), Some(Ok(Token::Semi)));
        assert_eq!(lex.next(), Some(Ok(Token::RBrace))); // End of else block
        assert_eq!(lex.next(), Some(Ok(Token::RBrace))); // End of struct block
        assert_eq!(lex.next(), Some(Ok(Token::Fn)));
        assert_eq!(lex.next(), Some(Ok(Token::Ident("main".to_string()))));
        assert_eq!(lex.next(), Some(Ok(Token::LParen)));
        assert_eq!(lex.next(), Some(Ok(Token::RParen)));
        assert_eq!(lex.next(), Some(Ok(Token::LBrace)));
        assert_eq!(lex.next(), Some(Ok(Token::Let)));
        assert_eq!(lex.next(), Some(Ok(Token::Ident("x".to_string()))));
        assert_eq!(lex.next(), Some(Ok(Token::Eq)));
        assert_eq!(lex.next(), Some(Ok(Token::Ident("MyStruct".to_string()))));
        assert_eq!(lex.next(), Some(Ok(Token::Semi)));
        assert_eq!(lex.next(), Some(Ok(Token::Return)));
        assert_eq!(lex.next(), Some(Ok(Token::Ident("x".to_string()))));
        assert_eq!(lex.next(), Some(Ok(Token::Dot)));
        assert_eq!(lex.next(), Some(Ok(Token::Ident("sum".to_string()))));
        assert_eq!(lex.next(), Some(Ok(Token::Semi)));
        assert_eq!(lex.next(), Some(Ok(Token::RBrace))); // End of main function block
        assert_eq!(lex.next(), None);
    }
}
