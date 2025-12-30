use sql2doc::core::parse::lexer::*;

#[test]
fn test_basic_keywords() {
    let mut lexer = Lexer::new("SELECT FROM WHERE");
    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens.len(), 3);
    assert_eq!(tokens[0].token_type, TokenType::Select);
    assert_eq!(tokens[1].token_type, TokenType::From);
    assert_eq!(tokens[2].token_type, TokenType::Where);
}

#[test]
fn test_identifiers() {
    let mut lexer = Lexer::new("users user_id \"quoted table\" `backtick table`");
    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens.len(), 4);
    assert_eq!(tokens[0].token_type, TokenType::Identifier("users".to_string()));
    assert_eq!(tokens[1].token_type, TokenType::Identifier("user_id".to_string()));
    assert_eq!(tokens[2].token_type, TokenType::QuotedIdentifier("quoted table".to_string()));
    assert_eq!(tokens[3].token_type, TokenType::QuotedIdentifier("backtick table".to_string()));
}

#[test]
fn test_literals() {
    let mut lexer = Lexer::new("123 45.67 'hello world' TRUE FALSE NULL");
    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens.len(), 6);
    assert_eq!(tokens[0].token_type, TokenType::IntegerLiteral(123));
    assert_eq!(tokens[1].token_type, TokenType::FloatLiteral(45.67));
    assert_eq!(tokens[2].token_type, TokenType::StringLiteral("hello world".to_string()));
    assert_eq!(tokens[3].token_type, TokenType::BooleanLiteral(true));
    assert_eq!(tokens[4].token_type, TokenType::BooleanLiteral(false));
    assert_eq!(tokens[5].token_type, TokenType::NullLiteral);
}

#[test]
fn test_operators() {
    let mut lexer = Lexer::new("= <> < <= > >= + - * / % || & | ^ ~ << >>");
    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens.len(), 18);
    assert_eq!(tokens[0].token_type, TokenType::Equal);
    assert_eq!(tokens[1].token_type, TokenType::NotEqual);
    assert_eq!(tokens[2].token_type, TokenType::LessThan);
    assert_eq!(tokens[3].token_type, TokenType::LessThanOrEqual);
    assert_eq!(tokens[4].token_type, TokenType::GreaterThan);
    assert_eq!(tokens[5].token_type, TokenType::GreaterThanOrEqual);
    assert_eq!(tokens[6].token_type, TokenType::Plus);
    assert_eq!(tokens[7].token_type, TokenType::Minus);
    assert_eq!(tokens[8].token_type, TokenType::Asterisk);
    assert_eq!(tokens[9].token_type, TokenType::Slash);
    assert_eq!(tokens[10].token_type, TokenType::Percent);
    assert_eq!(tokens[11].token_type, TokenType::Concat);
    assert_eq!(tokens[12].token_type, TokenType::BitwiseAnd);
    assert_eq!(tokens[13].token_type, TokenType::BitwiseOr);
    assert_eq!(tokens[14].token_type, TokenType::BitwiseXor);
    assert_eq!(tokens[15].token_type, TokenType::BitwiseNot);
    assert_eq!(tokens[16].token_type, TokenType::LeftShift);
    assert_eq!(tokens[17].token_type, TokenType::RightShift);
}

#[test]
fn test_punctuation() {
    let mut lexer = Lexer::new("( ) [ ] , ; . : :: ? @");
    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens.len(), 11);
    assert_eq!(tokens[0].token_type, TokenType::LeftParen);
    assert_eq!(tokens[1].token_type, TokenType::RightParen);
    assert_eq!(tokens[2].token_type, TokenType::LeftBracket);
    assert_eq!(tokens[3].token_type, TokenType::RightBracket);
    assert_eq!(tokens[4].token_type, TokenType::Comma);
    assert_eq!(tokens[5].token_type, TokenType::Semicolon);
    assert_eq!(tokens[6].token_type, TokenType::Dot);
    assert_eq!(tokens[7].token_type, TokenType::Colon);
    assert_eq!(tokens[8].token_type, TokenType::DoubleColon);
    assert_eq!(tokens[9].token_type, TokenType::QuestionMark);
    assert_eq!(tokens[10].token_type, TokenType::AtSign);
}

#[test]
fn test_comments() {
    let mut lexer = Lexer::new("SELECT -- this is a comment\nFROM /* multi\nline\ncomment */ WHERE");
    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens.len(), 3);
    assert_eq!(tokens[0].token_type, TokenType::Select);
    assert_eq!(tokens[1].token_type, TokenType::From);
    assert_eq!(tokens[2].token_type, TokenType::Where);
}

#[test]
fn test_case_insensitive_keywords() {
    let mut lexer = Lexer::new("select Select SELECT from From FROM");
    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens.len(), 6);
    for i in 0..3 {
        assert_eq!(tokens[i].token_type, TokenType::Select);
    }
    for i in 3..6 {
        assert_eq!(tokens[i].token_type, TokenType::From);
    }
}

#[test]
fn test_complex_sql() {
    let sql = r#"
        SELECT u.name, u.email, p.title
        FROM users u
        INNER JOIN posts p ON u.id = p.user_id
        WHERE u.active = TRUE AND p.created_at > '2023-01-01'
        ORDER BY p.created_at DESC
        LIMIT 10
    "#;

    let mut lexer = Lexer::new(sql);
    let tokens = lexer.tokenize().unwrap();

    // Should have many tokens, let's check a few key ones
    let select_tokens: Vec<_> = tokens.iter().filter(|t| matches!(t.token_type, TokenType::Select)).collect();
    assert_eq!(select_tokens.len(), 1);

    let from_tokens: Vec<_> = tokens.iter().filter(|t| matches!(t.token_type, TokenType::From)).collect();
    assert_eq!(from_tokens.len(), 1);

    let where_tokens: Vec<_> = tokens.iter().filter(|t| matches!(t.token_type, TokenType::Where)).collect();
    assert_eq!(where_tokens.len(), 1);

    let inner_tokens: Vec<_> = tokens.iter().filter(|t| matches!(t.token_type, TokenType::Inner)).collect();
    assert_eq!(inner_tokens.len(), 1);

    let join_tokens: Vec<_> = tokens.iter().filter(|t| matches!(t.token_type, TokenType::Join)).collect();
    assert_eq!(join_tokens.len(), 1);

    let and_tokens: Vec<_> = tokens.iter().filter(|t| matches!(t.token_type, TokenType::And)).collect();
    assert_eq!(and_tokens.len(), 1);

    let order_tokens: Vec<_> = tokens.iter().filter(|t| matches!(t.token_type, TokenType::Order)).collect();
    assert_eq!(order_tokens.len(), 1);

    let desc_tokens: Vec<_> = tokens.iter().filter(|t| matches!(t.token_type, TokenType::Desc)).collect();
    assert_eq!(desc_tokens.len(), 1);

    let limit_tokens: Vec<_> = tokens.iter().filter(|t| matches!(t.token_type, TokenType::Limit)).collect();
    assert_eq!(limit_tokens.len(), 1);
}

#[test]
fn test_position_tracking() {
    let mut lexer = Lexer::new("SELECT\n  name\nFROM users");
    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens.len(), 4);


    assert_eq!(tokens[0].position.line, 1);
    assert_eq!(tokens[0].position.column, 1);

    assert_eq!(tokens[1].position.line, 2);
    assert_eq!(tokens[1].position.column, 4);

    assert_eq!(tokens[2].position.line, 3);
    assert_eq!(tokens[2].position.column, 2);

    assert_eq!(tokens[3].position.line, 3);
    assert_eq!(tokens[3].position.column, 7);
}

#[test]
fn test_error_handling() {
    let mut lexer = Lexer::new("SELECT !");
    let result = lexer.tokenize();

    assert!(result.is_err());
    if let Err(error) = result {
        assert!(error.message.contains("Unexpected"));
    }
}
