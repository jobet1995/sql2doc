use std::collections::HashMap;
use std::iter::Peekable;
use std::str::Chars;

use crate::core::Position;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Keywords
    Select,
    From,
    Where,
    Join,
    Inner,
    Left,
    Right,
    Full,
    Outer,
    Cross,
    On,
    Using,
    Group,
    By,
    Having,
    Order,
    Asc,
    Desc,
    Limit,
    Offset,
    Insert,
    Into,
    Values,
    Update,
    Set,
    Delete,
    Create,
    Table,
    Index,
    Alter,
    Drop,
    Add,
    Modify,
    Rename,
    Column,
    Constraint,
    Primary,
    Foreign,
    Key,
    Unique,
    Check,
    Default,
    References,
    Not,
    NullLiteral,
    AutoIncrement,
    If,
    Exists,
    Distinct,
    As,
    And,
    Or,
    In,
    Between,
    Like,
    Ilike,
    Is,
    Union,
    All,
    Intersect,
    Except,
    With,
    Recursive,
    Case,
    When,
    Then,
    Else,
    End,
    Cast,

    // Literals
    Identifier(String),
    QuotedIdentifier(String),
    StringLiteral(String),
    IntegerLiteral(i64),
    FloatLiteral(f64),
    BooleanLiteral(bool),

    // Operators
    Plus,
    Minus,
    Asterisk,
    Slash,
    Percent,
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    Concat,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    BitwiseNot,
    LeftShift,
    RightShift,

    // Punctuation
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    Comma,
    Semicolon,
    Dot,
    Colon,
    DoubleColon,
    QuestionMark,
    AtSign,

    // Special tokens
    Eof,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub position: Position,
}


#[derive(Debug, Clone)]
pub struct LexerError {
    pub message: String,
    pub position: Position,
}

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
    position: Position,
    keywords: HashMap<String, TokenType>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut keywords = HashMap::new();

        // SQL keywords (case-insensitive)
        let keyword_map = vec![
            ("SELECT", TokenType::Select),
            ("FROM", TokenType::From),
            ("WHERE", TokenType::Where),
            ("JOIN", TokenType::Join),
            ("INNER", TokenType::Inner),
            ("LEFT", TokenType::Left),
            ("RIGHT", TokenType::Right),
            ("FULL", TokenType::Full),
            ("OUTER", TokenType::Outer),
            ("CROSS", TokenType::Cross),
            ("ON", TokenType::On),
            ("USING", TokenType::Using),
            ("GROUP", TokenType::Group),
            ("BY", TokenType::By),
            ("HAVING", TokenType::Having),
            ("ORDER", TokenType::Order),
            ("ASC", TokenType::Asc),
            ("DESC", TokenType::Desc),
            ("LIMIT", TokenType::Limit),
            ("OFFSET", TokenType::Offset),
            ("INSERT", TokenType::Insert),
            ("INTO", TokenType::Into),
            ("VALUES", TokenType::Values),
            ("UPDATE", TokenType::Update),
            ("SET", TokenType::Set),
            ("DELETE", TokenType::Delete),
            ("CREATE", TokenType::Create),
            ("TABLE", TokenType::Table),
            ("INDEX", TokenType::Index),
            ("ALTER", TokenType::Alter),
            ("DROP", TokenType::Drop),
            ("ADD", TokenType::Add),
            ("MODIFY", TokenType::Modify),
            ("RENAME", TokenType::Rename),
            ("COLUMN", TokenType::Column),
            ("CONSTRAINT", TokenType::Constraint),
            ("PRIMARY", TokenType::Primary),
            ("FOREIGN", TokenType::Foreign),
            ("KEY", TokenType::Key),
            ("UNIQUE", TokenType::Unique),
            ("CHECK", TokenType::Check),
            ("DEFAULT", TokenType::Default),
            ("REFERENCES", TokenType::References),
            ("NOT", TokenType::Not),
            ("NULL", TokenType::NullLiteral),
            ("AUTO_INCREMENT", TokenType::AutoIncrement),
            ("AUTOINCREMENT", TokenType::AutoIncrement),
            ("IF", TokenType::If),
            ("EXISTS", TokenType::Exists),
            ("DISTINCT", TokenType::Distinct),
            ("AS", TokenType::As),
            ("AND", TokenType::And),
            ("OR", TokenType::Or),
            ("IN", TokenType::In),
            ("BETWEEN", TokenType::Between),
            ("LIKE", TokenType::Like),
            ("ILIKE", TokenType::Ilike),
            ("IS", TokenType::Is),
            ("UNION", TokenType::Union),
            ("ALL", TokenType::All),
            ("INTERSECT", TokenType::Intersect),
            ("EXCEPT", TokenType::Except),
            ("WITH", TokenType::With),
            ("RECURSIVE", TokenType::Recursive),
            ("CASE", TokenType::Case),
            ("WHEN", TokenType::When),
            ("THEN", TokenType::Then),
            ("ELSE", TokenType::Else),
            ("END", TokenType::End),
            ("CAST", TokenType::Cast),
            ("TRUE", TokenType::BooleanLiteral(true)),
            ("FALSE", TokenType::BooleanLiteral(false)),
        ];

        for (keyword, token_type) in keyword_map {
            keywords.insert(keyword.to_lowercase(), token_type.clone());
        }

        Self {
            input: input.chars().peekable(),
            position: Position { line: 1, column: 1, index: 0 },
            keywords,
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens = Vec::new();

        while let Some(token) = self.next_token()? {
            tokens.push(token);
        }

        Ok(tokens)
    }

    pub fn next_token(&mut self) -> Result<Option<Token>, LexerError> {
        self.skip_whitespace_and_comments()?;

        match self.peek() {
            Some(&ch) => {
                let start_pos = self.position.clone();

                match ch {
                    // Single character tokens
                    '(' => { self.advance(); Ok(Some(Token::new(TokenType::LeftParen, "(", start_pos))) }
                    ')' => { self.advance(); Ok(Some(Token::new(TokenType::RightParen, ")", start_pos))) }
                    '[' => { self.advance(); Ok(Some(Token::new(TokenType::LeftBracket, "[", start_pos))) }
                    ']' => { self.advance(); Ok(Some(Token::new(TokenType::RightBracket, "]", start_pos))) }
                    ',' => { self.advance(); Ok(Some(Token::new(TokenType::Comma, ",", start_pos))) }
                    ';' => { self.advance(); Ok(Some(Token::new(TokenType::Semicolon, ";", start_pos))) }
                    '.' => { self.advance(); Ok(Some(Token::new(TokenType::Dot, ".", start_pos))) }
                    ':' => {
                        self.advance();
                        if self.peek() == Some(&':') {
                            self.advance();
                            Ok(Some(Token::new(TokenType::DoubleColon, "::", start_pos)))
                        } else {
                            Ok(Some(Token::new(TokenType::Colon, ":", start_pos)))
                        }
                    }
                    '?' => { self.advance(); Ok(Some(Token::new(TokenType::QuestionMark, "?", start_pos))) }
                    '@' => { self.advance(); Ok(Some(Token::new(TokenType::AtSign, "@", start_pos))) }

                    // Operators
                    '+' => { self.advance(); Ok(Some(Token::new(TokenType::Plus, "+", start_pos))) }
                    '*' => { self.advance(); Ok(Some(Token::new(TokenType::Asterisk, "*", start_pos))) }
                    '%' => { self.advance(); Ok(Some(Token::new(TokenType::Percent, "%", start_pos))) }
                    '&' => { self.advance(); Ok(Some(Token::new(TokenType::BitwiseAnd, "&", start_pos))) }
                    '|' => {
                        self.advance();
                        if self.peek() == Some(&'|') {
                            self.advance();
                            Ok(Some(Token::new(TokenType::Concat, "||", start_pos)))
                        } else {
                            Ok(Some(Token::new(TokenType::BitwiseOr, "|", start_pos)))
                        }
                    }
                    '^' => { self.advance(); Ok(Some(Token::new(TokenType::BitwiseXor, "^", start_pos))) }
                    '~' => { self.advance(); Ok(Some(Token::new(TokenType::BitwiseNot, "~", start_pos))) }

                    // Complex operators
                    '=' => { self.advance(); Ok(Some(Token::new(TokenType::Equal, "=", start_pos))) }
                    '!' => {
                        self.advance();
                        if self.peek() == Some(&'=') {
                            self.advance();
                            Ok(Some(Token::new(TokenType::NotEqual, "!=", start_pos)))
                        } else {
                            Err(LexerError::new("Unexpected '!'", start_pos))
                        }
                    }
                    '<' => {
                        self.advance();
                        match self.peek() {
                            Some(&'=') => { self.advance(); Ok(Some(Token::new(TokenType::LessThanOrEqual, "<=", start_pos))) }
                            Some(&'>') => { self.advance(); Ok(Some(Token::new(TokenType::NotEqual, "<>", start_pos))) }
                            Some(&'<') => { self.advance(); Ok(Some(Token::new(TokenType::LeftShift, "<<", start_pos))) }
                            _ => Ok(Some(Token::new(TokenType::LessThan, "<", start_pos)))
                        }
                    }
                    '>' => {
                        self.advance();
                        match self.peek() {
                            Some(&'=') => { self.advance(); Ok(Some(Token::new(TokenType::GreaterThanOrEqual, ">=", start_pos))) }
                            Some(&'>') => { self.advance(); Ok(Some(Token::new(TokenType::RightShift, ">>", start_pos))) }
                            _ => Ok(Some(Token::new(TokenType::GreaterThan, ">", start_pos)))
                        }
                    }
                    '-' => {
                        self.advance();
                        if self.peek() == Some(&'-') {
                            self.skip_single_line_comment()?;
                            self.next_token()
                        } else {
                            Ok(Some(Token::new(TokenType::Minus, "-", start_pos)))
                        }
                    }
                    '/' => {
                        self.advance();
                        if self.peek() == Some(&'*') {
                            self.skip_multi_line_comment()?;
                            self.next_token()
                        } else {
                            Ok(Some(Token::new(TokenType::Slash, "/", start_pos)))
                        }
                    }

                    // Literals and identifiers
                    '"' => self.lex_quoted_identifier(start_pos),
                    '`' => self.lex_backtick_identifier(start_pos),
                    '\'' => self.lex_string_literal(start_pos),

                    ch if ch.is_ascii_digit() => self.lex_number(start_pos),
                    ch if ch.is_alphabetic() || ch == '_' => self.lex_identifier_or_keyword(start_pos),

                    _ => Err(LexerError::new(format!("Unexpected character: {}", ch), start_pos)),
                }
            }
            None => Ok(None),
        }
    }

    fn lex_identifier_or_keyword(&mut self, start_pos: Position) -> Result<Option<Token>, LexerError> {
        let mut identifier = String::new();

        while let Some(&ch) = self.peek() {
            if ch.is_alphanumeric() || ch == '_' {
                #[allow(clippy::disallowed_methods)]
                identifier.push(self.advance().unwrap());
            } else {
                break;
            }
        }

        let lower_identifier = identifier.to_lowercase();
        let token_type = self.keywords.get(&lower_identifier)
            .cloned()
            .unwrap_or_else(|| TokenType::Identifier(identifier.clone()));

        Ok(Some(Token::new(token_type, &identifier, start_pos)))
    }

    fn lex_quoted_identifier(&mut self, start_pos: Position) -> Result<Option<Token>, LexerError> {
        self.advance(); // consume opening quote
        let mut identifier = String::new();

        while let Some(ch) = self.peek() {
            match ch {
                '"' => {
                    self.advance();
                    if self.peek() == Some(&'"') {
                        // Escaped quote
                        self.advance();
                        identifier.push('"');
                    } else {
                        // End of quoted identifier
                        break;
                    }
                }
                '\n' | '\r' => return Err(LexerError::new("Unterminated quoted identifier", self.position.clone())),
                _ => {
                    #[allow(clippy::disallowed_methods)]
                    identifier.push(self.advance().unwrap());
                }
            }
        }

        Ok(Some(Token::new(TokenType::QuotedIdentifier(identifier.clone()), &format!("\"{}\"", identifier), start_pos)))
    }

    fn lex_backtick_identifier(&mut self, start_pos: Position) -> Result<Option<Token>, LexerError> {
        self.advance(); // consume opening backtick
        let mut identifier = String::new();

        while let Some(ch) = self.peek() {
            match ch {
                '`' => {
                    self.advance();
                    break;
                }
                '\n' | '\r' => return Err(LexerError::new("Unterminated backtick identifier", self.position.clone())),
                _ => {
                    #[allow(clippy::disallowed_methods)]
                    identifier.push(self.advance().unwrap());
                }
            }
        }

        Ok(Some(Token::new(TokenType::QuotedIdentifier(identifier.clone()), &format!("`{}`", identifier), start_pos)))
    }

    fn lex_string_literal(&mut self, start_pos: Position) -> Result<Option<Token>, LexerError> {
        self.advance(); // consume opening quote
        let mut literal = String::new();

        while let Some(ch) = self.peek() {
            match ch {
                '\'' => {
                    self.advance();
                    if self.peek() == Some(&'\'') {
                        // Escaped quote
                        self.advance();
                        literal.push('\'');
                    } else {
                        // End of string literal
                        break;
                    }
                }
                '\n' | '\r' => return Err(LexerError::new("Unterminated string literal", self.position.clone())),
                _ => {
                    #[allow(clippy::disallowed_methods)]
                    literal.push(self.advance().unwrap());
                }
            }
        }

        Ok(Some(Token::new(TokenType::StringLiteral(literal.clone()), &format!("'{}'", literal), start_pos)))
    }

    fn lex_number(&mut self, start_pos: Position) -> Result<Option<Token>, LexerError> {
        let mut number_str = String::new();
        let mut has_dot = false;

        while let Some(&ch) = self.peek() {
            if ch.is_ascii_digit() {
                #[allow(clippy::disallowed_methods)]
                number_str.push(self.advance().unwrap());
            } else if ch == '.' && !has_dot {
                has_dot = true;
                #[allow(clippy::disallowed_methods)]
                number_str.push(self.advance().unwrap());
            } else {
                break;
            }
        }

        if has_dot {
            match number_str.parse::<f64>() {
                Ok(value) => Ok(Some(Token::new(TokenType::FloatLiteral(value), &number_str, start_pos))),
                Err(_) => Err(LexerError::new(format!("Invalid float literal: {}", number_str), start_pos)),
            }
        } else {
            match number_str.parse::<i64>() {
                Ok(value) => Ok(Some(Token::new(TokenType::IntegerLiteral(value), &number_str, start_pos))),
                Err(_) => Err(LexerError::new(format!("Invalid integer literal: {}", number_str), start_pos)),
            }
        }
    }

    fn skip_whitespace_and_comments(&mut self) -> Result<(), LexerError> {
        while let Some(&ch) = self.peek() {
            match ch {
                ' ' | '\t' | '\r' => { self.advance(); }
                '\n' => {
                    self.position.line += 1;
                    self.position.column = 1;
                    self.advance();
                }
                '-' => {
                    if let Some('-') = self.input.clone().nth(1) {
                        self.skip_single_line_comment()?;
                    } else {
                        break;
                    }
                }
                '/' => {
                    if let Some('*') = self.input.clone().nth(1) {
                        self.skip_multi_line_comment()?;
                    } else {
                        break;
                    }
                }
                _ => break,
            }
        }
        Ok(())
    }

    fn skip_single_line_comment(&mut self) -> Result<(), LexerError> {
        self.advance(); // consume first '-'
        self.advance(); // consume second '-'

        while let Some(ch) = self.advance() {
            if ch == '\n' {
                self.position.line += 1;
                self.position.column = 1;
                break;
            }
        }
        Ok(())
    }

    fn skip_multi_line_comment(&mut self) -> Result<(), LexerError> {
        self.advance(); // consume '/'
        self.advance(); // consume '*'

        while let Some(ch) = self.advance() {
            if ch == '*' {
                if let Some('/') = self.peek() {
                    self.advance(); // consume '/'
                    break;
                }
            } else if ch == '\n' {
                self.position.line += 1;
                self.position.column = 1;
            }
        }
        Ok(())
    }

    fn peek(&mut self) -> Option<&char> {
        self.input.peek()
    }

    fn advance(&mut self) -> Option<char> {
        match self.input.next() {
            Some(ch) => {
                self.position.index += 1;
                self.position.column += 1;
                Some(ch)
            }
            None => None,
        }
    }
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: &str, position: Position) -> Self {
        Self {
            token_type,
            lexeme: lexeme.to_string(),
            position,
        }
    }

    pub fn eof(position: Position) -> Self {
        Self {
            token_type: TokenType::Eof,
            lexeme: "".to_string(),
            position,
        }
    }
}

impl LexerError {
    pub fn new(message: impl Into<String>, position: Position) -> Self {
        Self {
            message: message.into(),
            position,
        }
    }
}

impl std::fmt::Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} at line {}, column {}", self.message, self.position.line, self.position.column)
    }
}

impl std::error::Error for LexerError {}
