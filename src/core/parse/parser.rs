use std::collections::HashMap;
use std::iter::Peekable;

use crate::core::ast::*;
use crate::core::parse::lexer::{Token, TokenType};
use crate::core::Position;

#[derive(Debug, Clone)]
pub struct ParserError {
    pub message: String,
    pub position: Position,
}

pub struct Parser {
    tokens: Peekable<std::vec::IntoIter<Token>>,
    current_token: Option<Token>,
    peek_token: Option<Token>,
    errors: Vec<ParserError>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        let mut token_iter = tokens.into_iter().peekable();
        let current_token = token_iter.next();
        let peek_token = token_iter.next();

        Self {
            tokens: token_iter,
            current_token,
            peek_token,
            errors: Vec::new(),
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Statement>, Vec<ParserError>> {
        let mut statements = Vec::new();

        while self.current_token.is_some() {
            match self.parse_statement() {
                Ok(stmt) => statements.push(stmt),
                Err(err) => {
                    self.errors.push(err);
                    self.synchronize();
                }
            }

            if let Some(Token { token_type: TokenType::Semicolon, .. }) = &self.current_token {
                self.advance();
            }
        }

        if self.errors.is_empty() {
            Ok(statements)
        } else {
            Err(self.errors.clone())
        }
    }

    fn parse_statement(&mut self) -> Result<Statement, ParserError> {
        match &self.current_token {
            Some(Token { token_type: TokenType::Select, .. }) => {
                let select = self.parse_select_statement()?;
                Ok(Statement::Dml(Box::new(DmlStatement::Select(Box::new(select)))))
            }
            Some(Token { token_type: TokenType::Insert, .. }) => {
                let insert = self.parse_insert_statement()?;
                Ok(Statement::Dml(Box::new(DmlStatement::Insert(Box::new(insert)))))
            }
            Some(Token { token_type: TokenType::Update, .. }) => {
                let update = self.parse_update_statement()?;
                Ok(Statement::Dml(Box::new(DmlStatement::Update(Box::new(update)))))
            }
            Some(Token { token_type: TokenType::Delete, .. }) => {
                let delete = self.parse_delete_statement()?;
                Ok(Statement::Dml(Box::new(DmlStatement::Delete(Box::new(delete)))))
            }
            Some(Token { token_type: TokenType::Create, .. }) => {
                let ddl = self.parse_ddl_statement()?;
                Ok(Statement::Ddl(ddl))
            }
            Some(Token { token_type: TokenType::Alter, .. }) => {
                let ddl = self.parse_ddl_statement()?;
                Ok(Statement::Ddl(ddl))
            }
            Some(Token { token_type: TokenType::Drop, .. }) => {
                let ddl = self.parse_ddl_statement()?;
                Ok(Statement::Ddl(ddl))
            }
            Some(token) => Err(ParserError::new(
                format!("Unexpected token: {:?}", token.token_type),
                token.position.clone(),
            )),
            None => Err(ParserError::new("Unexpected end of input".to_string(), Position::default())),
        }
    }

    fn parse_select_statement(&mut self) -> Result<SelectStatement, ParserError> {
        self.expect_token(TokenType::Select)?;

        let mut select = SelectStatement::new();

        // Handle WITH clause
        if let Some(Token { token_type: TokenType::With, .. }) = &self.current_token {
            select.with = self.parse_with_clause()?;
        }

        // Handle DISTINCT
        if let Some(Token { token_type: TokenType::Distinct, .. }) = &self.current_token {
            select.distinct = true;
            self.advance();
        }

        // Parse SELECT list
        select.select_list = self.parse_select_list()?;

        // Parse FROM clause
        if let Some(Token { token_type: TokenType::From, .. }) = &self.current_token {
            self.advance();
            select.from = self.parse_from_clause()?;
        }

        // Parse WHERE clause
        if let Some(Token { token_type: TokenType::Where, .. }) = &self.current_token {
            self.advance();
            select.where_clause = Some(self.parse_expression()?);
        }

        // Parse GROUP BY
        if let Some(Token { token_type: TokenType::Group, .. }) = &self.current_token {
            self.expect_token(TokenType::Group)?;
            self.expect_token(TokenType::By)?;
            select.group_by = Some(self.parse_group_by()?);
        }

        // Parse HAVING
        if let Some(Token { token_type: TokenType::Having, .. }) = &self.current_token {
            self.advance();
            select.having = Some(self.parse_expression()?);
        }

        // Parse ORDER BY
        if let Some(Token { token_type: TokenType::Order, .. }) = &self.current_token {
            self.expect_token(TokenType::Order)?;
            self.expect_token(TokenType::By)?;
            select.order_by = self.parse_order_by()?;
        }

        // Parse LIMIT
        if let Some(Token { token_type: TokenType::Limit, .. }) = &self.current_token {
            self.advance();
            select.limit = Some(self.parse_limit()?);
        }

        // Parse OFFSET
        if let Some(Token { token_type: TokenType::Offset, .. }) = &self.current_token {
            self.advance();
            select.offset = Some(self.parse_offset()?);
        }

        // Parse UNION
        select.unions = self.parse_unions()?;

        Ok(select)
    }

    fn parse_with_clause(&mut self) -> Result<Vec<CommonTableExpression>, ParserError> {
        self.expect_token(TokenType::With)?;
        let mut ctes = Vec::new();

        loop {
            let mut recursive = false;
            if let Some(Token { token_type: TokenType::Recursive, .. }) = &self.current_token {
                recursive = true;
                self.advance();
            }

            let name = self.parse_identifier()?;
            let columns = if let Some(Token { token_type: TokenType::LeftParen, .. }) = &self.current_token {
                self.advance();
                let cols = self.parse_identifier_list()?;
                self.expect_token(TokenType::RightParen)?;
                cols
            } else {
                Vec::new()
            };

            self.expect_token(TokenType::As)?;
            self.expect_token(TokenType::LeftParen)?;
            let query = self.parse_select_statement()?;
            self.expect_token(TokenType::RightParen)?;

            ctes.push(CommonTableExpression {
                name,
                columns,
                query,
                recursive,
            });

            if let Some(Token { token_type: TokenType::Comma, .. }) = &self.current_token {
                self.advance();
            } else {
                break;
            }
        }

        Ok(ctes)
    }

    fn parse_select_list(&mut self) -> Result<Vec<SelectItem>, ParserError> {
        let mut items = Vec::new();

        loop {
            if let Some(Token { token_type: TokenType::Asterisk, .. }) = &self.current_token {
                self.advance();
                if let Some(Token { token_type: TokenType::Dot, .. }) = &self.current_token {
                    self.advance();
                    let qualifier = items.last()
                        .and_then(|item| match item {
                            SelectItem::QualifiedWildcard { qualifier } => Some(qualifier.clone()),
                            _ => None,
                        })
                        .unwrap_or_default();
                    items.push(SelectItem::QualifiedWildcard { qualifier });
                } else {
                    items.push(SelectItem::Wildcard);
                }
            } else {
                let expr = self.parse_expression()?;
                let alias = if let Some(Token { token_type: TokenType::As, .. }) = &self.current_token {
                    self.advance();
                    Some(self.parse_identifier()?)
                } else if let Some(Token { token_type: TokenType::Identifier(_), .. }) = &self.current_token {
                    // Check if it's a potential alias (not a keyword)
                    Some(self.parse_identifier()?)
                } else {
                    None
                };

                items.push(SelectItem::Expression { expr, alias });
            }

            if let Some(Token { token_type: TokenType::Comma, .. }) = &self.current_token {
                self.advance();
            } else {
                break;
            }
        }

        Ok(items)
    }

    fn parse_from_clause(&mut self) -> Result<Vec<TableReference>, ParserError> {
        let mut tables = vec![self.parse_table_reference()?];

        // Parse JOINs
        while let Some(token) = &self.current_token {
            match token.token_type {
                TokenType::Inner | TokenType::Left | TokenType::Right | TokenType::Full |
                TokenType::Join | TokenType::Cross => {
                    let join = self.parse_join()?;
                    if let Some(last_table) = tables.last_mut() {
                        match last_table {
                            TableReference::Join { joins, .. } => joins.push(join),
                            _ => {
                                // Convert single table to join
                                let left = Box::new(last_table.clone());
                                let JoinClause { join_type: _, ref table, condition: _ } = join;
                                *last_table = TableReference::Join {
                                    left,
                                    right: Box::new(table.clone()),
                                    joins: vec![join],
                                };
                            }
                        }
                    }
                }
                _ => break,
            }
        }

        Ok(tables)
    }

    fn parse_table_reference(&mut self) -> Result<TableReference, ParserError> {
        let name = self.parse_identifier()?;
        let alias = if let Some(Token { token_type: TokenType::As, .. }) = &self.current_token {
            self.advance();
            Some(self.parse_identifier()?)
        } else if let Some(Token { token_type: TokenType::Identifier(_), .. }) = &self.current_token {
            Some(self.parse_identifier()?)
        } else {
            None
        };

        Ok(TableReference::Table { name, alias })
    }

    fn parse_join(&mut self) -> Result<JoinClause, ParserError> {
        let join_type = match &self.current_token {
            Some(Token { token_type: TokenType::Inner, .. }) => {
                self.advance();
                self.expect_token(TokenType::Join)?;
                JoinType::Inner
            }
            Some(Token { token_type: TokenType::Left, .. }) => {
                self.advance();
                if let Some(Token { token_type: TokenType::Outer, .. }) = &self.current_token {
                    self.advance();
                }
                self.expect_token(TokenType::Join)?;
                JoinType::Left
            }
            Some(Token { token_type: TokenType::Right, .. }) => {
                self.advance();
                if let Some(Token { token_type: TokenType::Outer, .. }) = &self.current_token {
                    self.advance();
                }
                self.expect_token(TokenType::Join)?;
                JoinType::Right
            }
            Some(Token { token_type: TokenType::Full, .. }) => {
                self.advance();
                if let Some(Token { token_type: TokenType::Outer, .. }) = &self.current_token {
                    self.advance();
                }
                self.expect_token(TokenType::Join)?;
                JoinType::Full
            }
            Some(Token { token_type: TokenType::Cross, .. }) => {
                self.advance();
                self.expect_token(TokenType::Join)?;
                JoinType::Cross
            }
            Some(Token { token_type: TokenType::Join, .. }) => {
                self.advance();
                JoinType::Inner
            }
            _ => return Err(self.unexpected_token_error("join type")),
        };

        let table = self.parse_table_reference()?;
        let condition = if let Some(Token { token_type: TokenType::On, .. }) = &self.current_token {
            self.advance();
            Some(JoinCondition::On(self.parse_expression()?))
        } else if let Some(Token { token_type: TokenType::Using, .. }) = &self.current_token {
            self.advance();
            self.expect_token(TokenType::LeftParen)?;
            let columns = self.parse_identifier_list()?;
            self.expect_token(TokenType::RightParen)?;
            Some(JoinCondition::Using(columns))
        } else {
            None
        };

        Ok(JoinClause {
            join_type,
            table,
            condition,
        })
    }

    fn parse_expression(&mut self) -> Result<Expression, ParserError> {
        self.parse_logical_or()
    }

    fn parse_logical_or(&mut self) -> Result<Expression, ParserError> {
        let mut left = self.parse_logical_and()?;

        while let Some(Token { token_type: TokenType::Or, .. }) = &self.current_token {
            self.advance();
            let right = self.parse_logical_and()?;
            left = Expression::BinaryOp {
                left: Box::new(left),
                op: BinaryOperator::Or,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_logical_and(&mut self) -> Result<Expression, ParserError> {
        let mut left = self.parse_equality()?;

        while let Some(Token { token_type: TokenType::And, .. }) = &self.current_token {
            self.advance();
            let right = self.parse_equality()?;
            left = Expression::BinaryOp {
                left: Box::new(left),
                op: BinaryOperator::And,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_equality(&mut self) -> Result<Expression, ParserError> {
        let mut left = self.parse_comparison()?;

        while let Some(token) = &self.current_token {
            let op = match token.token_type {
                TokenType::Equal => BinaryOperator::Eq,
                TokenType::NotEqual => BinaryOperator::Neq,
                _ => break,
            };
            self.advance();
            let right = self.parse_comparison()?;
            left = Expression::BinaryOp {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_comparison(&mut self) -> Result<Expression, ParserError> {
        let mut left = self.parse_term()?;

        while let Some(token) = &self.current_token {
            let op = match token.token_type {
                TokenType::LessThan => BinaryOperator::Lt,
                TokenType::LessThanOrEqual => BinaryOperator::Lte,
                TokenType::GreaterThan => BinaryOperator::Gt,
                TokenType::GreaterThanOrEqual => BinaryOperator::Gte,
                TokenType::Like => BinaryOperator::Like,
                TokenType::Ilike => BinaryOperator::ILike,
                TokenType::In => {
                    self.advance();
                    if let Some(Token { token_type: TokenType::LeftParen, .. }) = &self.current_token {
                        self.advance();
                        let values = self.parse_expression_list()?;
                        self.expect_token(TokenType::RightParen)?;
                        return Ok(Expression::In {
                            expr: Box::new(left),
                            values,
                        });
                    } else {
                        return Err(self.unexpected_token_error("left parenthesis after IN"));
                    }
                }
                TokenType::Between => {
                    self.advance();
                    let min = self.parse_term()?;
                    self.expect_token(TokenType::And)?;
                    let max = self.parse_term()?;
                    return Ok(Expression::Between {
                        expr: Box::new(left),
                        min: Box::new(min),
                        max: Box::new(max),
                    });
                }
                TokenType::Is => {
                    self.advance();
                    if let Some(Token { token_type: TokenType::Not, .. }) = &self.current_token {
                        self.advance();
                        let right = self.parse_term()?;
                        return Ok(Expression::BinaryOp {
                            left: Box::new(left),
                            op: BinaryOperator::Neq,
                            right: Box::new(right),
                        });
                    } else {
                        let right = self.parse_term()?;
                        return Ok(Expression::BinaryOp {
                            left: Box::new(left),
                            op: BinaryOperator::Eq,
                            right: Box::new(right),
                        });
                    }
                }
                _ => break,
            };
            self.advance();
            let right = self.parse_term()?;
            left = Expression::BinaryOp {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_term(&mut self) -> Result<Expression, ParserError> {
        let mut left = self.parse_factor()?;

        while let Some(token) = &self.current_token {
            let op = match token.token_type {
                TokenType::Plus => BinaryOperator::Plus,
                TokenType::Minus => BinaryOperator::Minus,
                TokenType::Concat => BinaryOperator::Concat,
                _ => break,
            };
            self.advance();
            let right = self.parse_factor()?;
            left = Expression::BinaryOp {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_factor(&mut self) -> Result<Expression, ParserError> {
        let mut left = self.parse_unary()?;

        while let Some(token) = &self.current_token {
            let op = match token.token_type {
                TokenType::Asterisk => BinaryOperator::Multiply,
                TokenType::Slash => BinaryOperator::Divide,
                TokenType::Percent => BinaryOperator::Modulo,
                TokenType::BitwiseAnd => BinaryOperator::BitwiseAnd,
                TokenType::BitwiseOr => BinaryOperator::BitwiseOr,
                TokenType::BitwiseXor => BinaryOperator::BitwiseXor,
                _ => break,
            };
            self.advance();
            let right = self.parse_unary()?;
            left = Expression::BinaryOp {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_unary(&mut self) -> Result<Expression, ParserError> {
        if let Some(token) = &self.current_token {
            match token.token_type {
                TokenType::Not => {
                    self.advance();
                    let expr = self.parse_unary()?;
                    return Ok(Expression::UnaryOp {
                        op: UnaryOperator::Not,
                        expr: Box::new(expr),
                    });
                }
                TokenType::Minus => {
                    self.advance();
                    let expr = self.parse_unary()?;
                    return Ok(Expression::UnaryOp {
                        op: UnaryOperator::Minus,
                        expr: Box::new(expr),
                    });
                }
                TokenType::Plus => {
                    self.advance();
                    return self.parse_unary();
                }
                TokenType::BitwiseNot => {
                    self.advance();
                    let expr = self.parse_unary()?;
                    return Ok(Expression::UnaryOp {
                        op: UnaryOperator::BitwiseNot,
                        expr: Box::new(expr),
                    });
                }
                _ => {}
            }
        }

        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Result<Expression, ParserError> {
        match &self.current_token {
            Some(Token { token_type: TokenType::IntegerLiteral(value), .. }) => {
                let expr = Expression::Literal(LiteralValue::Integer(*value));
                self.advance();
                Ok(expr)
            }
            Some(Token { token_type: TokenType::FloatLiteral(value), .. }) => {
                let expr = Expression::Literal(LiteralValue::Float(*value));
                self.advance();
                Ok(expr)
            }
            Some(Token { token_type: TokenType::StringLiteral(value), .. }) => {
                let expr = Expression::Literal(LiteralValue::String(value.clone()));
                self.advance();
                Ok(expr)
            }
            Some(Token { token_type: TokenType::BooleanLiteral(value), .. }) => {
                let expr = Expression::Literal(LiteralValue::Boolean(*value));
                self.advance();
                Ok(expr)
            }
            Some(Token { token_type: TokenType::NullLiteral, .. }) => {
                let expr = Expression::Literal(LiteralValue::Null);
                self.advance();
                Ok(expr)
            }
            Some(Token { token_type: TokenType::Identifier(name), .. }) => {
                let identifier = name.clone();
                self.advance();

                if let Some(Token { token_type: TokenType::LeftParen, .. }) = &self.current_token {
                    // Function call
                    self.advance();
                    let args = if let Some(Token { token_type: TokenType::RightParen, .. }) = &self.current_token {
                        self.advance();
                        Vec::new()
                    } else {
                        let args = self.parse_expression_list()?;
                        self.expect_token(TokenType::RightParen)?;
                        args
                    };
                    Ok(Expression::Function {
                        name: identifier,
                        args,
                    })
                } else if let Some(Token { token_type: TokenType::Dot, .. }) = &self.current_token {
                    // Qualified identifier
                    self.advance();
                    let column = self.parse_identifier()?;
                    Ok(Expression::QualifiedColumn {
                        table: identifier,
                        column,
                    })
                } else {
                    Ok(Expression::Column(identifier))
                }
            }
            Some(Token { token_type: TokenType::QuotedIdentifier(name), .. }) => {
                let identifier = name.clone();
                self.advance();

                if let Some(Token { token_type: TokenType::Dot, .. }) = &self.current_token {
                    self.advance();
                    let column = self.parse_identifier()?;
                    Ok(Expression::QualifiedColumn {
                        table: identifier,
                        column,
                    })
                } else {
                    Ok(Expression::Column(identifier))
                }
            }
            Some(Token { token_type: TokenType::LeftParen, .. }) => {
                self.advance();
                let expr = self.parse_expression()?;
                self.expect_token(TokenType::RightParen)?;
                Ok(expr)
            }
            Some(Token { token_type: TokenType::Case, .. }) => {
                self.parse_case_expression()
            }
            Some(Token { token_type: TokenType::Cast, .. }) => {
                self.parse_cast_expression()
            }
            Some(token) => Err(ParserError::new(
                format!("Unexpected token in expression: {:?}", token.token_type),
                token.position.clone(),
            )),
            None => Err(ParserError::new("Unexpected end of input in expression".to_string(), Position::default())),
        }
    }

    fn parse_case_expression(&mut self) -> Result<Expression, ParserError> {
        self.expect_token(TokenType::Case)?;
        let operand = if let Some(Token { token_type: TokenType::When, .. }) = &self.current_token {
            None
        } else {
            Some(Box::new(self.parse_expression()?))
        };

        let mut when_clauses = Vec::new();
        while let Some(Token { token_type: TokenType::When, .. }) = &self.current_token {
            self.advance();
            let condition = self.parse_expression()?;
            self.expect_token(TokenType::Then)?;
            let result = self.parse_expression()?;
            when_clauses.push(WhenClause { condition, result });
        }

        let else_expr = if let Some(Token { token_type: TokenType::Else, .. }) = &self.current_token {
            self.advance();
            Some(Box::new(self.parse_expression()?))
        } else {
            None
        };

        self.expect_token(TokenType::End)?;
        Ok(Expression::Case {
            operand,
            when_clauses,
            else_expr,
        })
    }

    fn parse_cast_expression(&mut self) -> Result<Expression, ParserError> {
        self.expect_token(TokenType::Cast)?;
        self.expect_token(TokenType::LeftParen)?;
        let expr = self.parse_expression()?;
        self.expect_token(TokenType::As)?;
        let data_type = self.parse_data_type()?;
        self.expect_token(TokenType::RightParen)?;

        Ok(Expression::Cast {
            expr: Box::new(expr),
            data_type,
        })
    }

    fn parse_data_type(&mut self) -> Result<String, ParserError> {
        // Simple implementation - just parse identifiers
        match &self.current_token {
            Some(Token { token_type: TokenType::Identifier(name), .. }) => {
                let type_name = name.clone();
                self.advance();
                Ok(type_name)
            }
            Some(token) => Err(ParserError::new(
                format!("Expected data type, found {:?}", token.token_type),
                token.position.clone(),
            )),
            None => Err(ParserError::new("Unexpected end of input in data type".to_string(), Position::default())),
        }
    }

    fn parse_identifier(&mut self) -> Result<String, ParserError> {
        match &self.current_token {
            Some(Token { token_type: TokenType::Identifier(name), .. }) => {
                let id = name.clone();
                self.advance();
                Ok(id)
            }
            Some(Token { token_type: TokenType::QuotedIdentifier(name), .. }) => {
                let id = name.clone();
                self.advance();
                Ok(id)
            }
            Some(token) => Err(ParserError::new(
                format!("Expected identifier, found {:?}", token.token_type),
                token.position.clone(),
            )),
            None => Err(ParserError::new("Unexpected end of input, expected identifier".to_string(), Position::default())),
        }
    }

    fn parse_identifier_list(&mut self) -> Result<Vec<String>, ParserError> {
        let mut identifiers = Vec::new();

        loop {
            identifiers.push(self.parse_identifier()?);
            if let Some(Token { token_type: TokenType::Comma, .. }) = &self.current_token {
                self.advance();
            } else {
                break;
            }
        }

        Ok(identifiers)
    }

    fn parse_expression_list(&mut self) -> Result<Vec<Expression>, ParserError> {
        let mut expressions = Vec::new();

        loop {
            expressions.push(self.parse_expression()?);
            if let Some(Token { token_type: TokenType::Comma, .. }) = &self.current_token {
                self.advance();
            } else {
                break;
            }
        }

        Ok(expressions)
    }

    fn parse_group_by(&mut self) -> Result<GroupByClause, ParserError> {
        let mut expressions = Vec::new();

        loop {
            expressions.push(self.parse_expression()?);
            if let Some(Token { token_type: TokenType::Comma, .. }) = &self.current_token {
                self.advance();
            } else {
                break;
            }
        }

        Ok(GroupByClause { expressions })
    }

    fn parse_order_by(&mut self) -> Result<Vec<OrderByClause>, ParserError> {
        let mut order_by = Vec::new();

        loop {
            let expr = self.parse_expression()?;
            let ascending = match &self.current_token {
                Some(Token { token_type: TokenType::Asc, .. }) => {
                    self.advance();
                    true
                }
                Some(Token { token_type: TokenType::Desc, .. }) => {
                    self.advance();
                    false
                }
                _ => true, // Default to ascending
            };

            order_by.push(OrderByClause {
                expr,
                ascending,
                nulls_first: None, // Not implemented yet
            });

            if let Some(Token { token_type: TokenType::Comma, .. }) = &self.current_token {
                self.advance();
            } else {
                break;
            }
        }

        Ok(order_by)
    }

    fn parse_limit(&mut self) -> Result<u64, ParserError> {
        match &self.current_token {
            Some(Token { token_type: TokenType::IntegerLiteral(value), .. }) => {
                let limit = *value as u64;
                self.advance();
                Ok(limit)
            }
            Some(token) => Err(ParserError::new(
                format!("Expected integer for LIMIT, found {:?}", token.token_type),
                token.position.clone(),
            )),
            None => Err(ParserError::new("Unexpected end of input in LIMIT".to_string(), Position::default())),
        }
    }

    fn parse_offset(&mut self) -> Result<u64, ParserError> {
        match &self.current_token {
            Some(Token { token_type: TokenType::IntegerLiteral(value), .. }) => {
                let offset = *value as u64;
                self.advance();
                Ok(offset)
            }
            Some(token) => Err(ParserError::new(
                format!("Expected integer for OFFSET, found {:?}", token.token_type),
                token.position.clone(),
            )),
            None => Err(ParserError::new("Unexpected end of input in OFFSET".to_string(), Position::default())),
        }
    }

    fn parse_unions(&mut self) -> Result<Vec<UnionClause>, ParserError> {
        let mut unions = Vec::new();

        while let Some(token) = &self.current_token {
            let all = match token.token_type {
                TokenType::Union => {
                    self.advance();
                    if let Some(Token { token_type: TokenType::All, .. }) = &self.current_token {
                        self.advance();
                        true
                    } else {
                        false
                    }
                }
                _ => break,
            };

            let select = self.parse_select_statement()?;
            unions.push(UnionClause { select, all });
        }

        Ok(unions)
    }

    fn parse_insert_statement(&mut self) -> Result<InsertStatement, ParserError> {
        self.expect_token(TokenType::Insert)?;
        self.expect_token(TokenType::Into)?;

        let table_name = self.parse_identifier()?;
        let columns = if let Some(Token { token_type: TokenType::LeftParen, .. }) = &self.current_token {
            self.advance();
            let cols = self.parse_identifier_list()?;
            self.expect_token(TokenType::RightParen)?;
            cols
        } else {
            Vec::new()
        };

        let (values, select) = if let Some(Token { token_type: TokenType::Values, .. }) = &self.current_token {
            self.advance();
            (self.parse_values_clause()?, None)
        } else if let Some(Token { token_type: TokenType::Select, .. }) = &self.current_token {
            (Vec::new(), Some(self.parse_select_statement()?))
        } else {
            return Err(self.unexpected_token_error("VALUES or SELECT"));
        };

        Ok(InsertStatement {
            table_name,
            columns,
            values,
            select,
            on_conflict: None, // Not implemented yet
            returning: Vec::new(), // Not implemented yet
        })
    }

    fn parse_values_clause(&mut self) -> Result<Vec<Vec<Expression>>, ParserError> {
        let mut value_lists = Vec::new();

        loop {
            self.expect_token(TokenType::LeftParen)?;
            let values = self.parse_expression_list()?;
            self.expect_token(TokenType::RightParen)?;
            value_lists.push(values);

            if let Some(Token { token_type: TokenType::Comma, .. }) = &self.current_token {
                self.advance();
            } else {
                break;
            }
        }

        Ok(value_lists)
    }

    fn parse_update_statement(&mut self) -> Result<UpdateStatement, ParserError> {
        self.expect_token(TokenType::Update)?;
        let table_name = self.parse_identifier()?;
        let alias = if let Some(Token { token_type: TokenType::As, .. }) = &self.current_token {
            self.advance();
            Some(self.parse_identifier()?)
        } else if let Some(Token { token_type: TokenType::Identifier(_), .. }) = &self.current_token {
            Some(self.parse_identifier()?)
        } else {
            None
        };

        self.expect_token(TokenType::Set)?;
        let assignments = self.parse_assignments()?;

        let from = Vec::new(); // Not implemented yet
        let where_clause = if let Some(Token { token_type: TokenType::Where, .. }) = &self.current_token {
            self.advance();
            Some(self.parse_expression()?)
        } else {
            None
        };

        Ok(UpdateStatement {
            table_name,
            alias,
            assignments,
            from,
            where_clause,
            returning: Vec::new(), // Not implemented yet
        })
    }

    fn parse_assignments(&mut self) -> Result<HashMap<String, Expression>, ParserError> {
        let mut assignments = HashMap::new();

        loop {
            let column = self.parse_identifier()?;
            self.expect_token(TokenType::Equal)?;
            let value = self.parse_expression()?;
            assignments.insert(column, value);

            if let Some(Token { token_type: TokenType::Comma, .. }) = &self.current_token {
                self.advance();
            } else {
                break;
            }
        }

        Ok(assignments)
    }

    fn parse_delete_statement(&mut self) -> Result<DeleteStatement, ParserError> {
        self.expect_token(TokenType::Delete)?;
        self.expect_token(TokenType::From)?;
        let table_name = self.parse_identifier()?;
        let alias = if let Some(Token { token_type: TokenType::As, .. }) = &self.current_token {
            self.advance();
            Some(self.parse_identifier()?)
        } else if let Some(Token { token_type: TokenType::Identifier(_), .. }) = &self.current_token {
            Some(self.parse_identifier()?)
        } else {
            None
        };

        let where_clause = if let Some(Token { token_type: TokenType::Where, .. }) = &self.current_token {
            self.advance();
            Some(self.parse_expression()?)
        } else {
            None
        };

        Ok(DeleteStatement {
            table_name,
            alias,
            using: Vec::new(), // Not implemented yet
            where_clause,
            returning: Vec::new(), // Not implemented yet
        })
    }

    fn parse_ddl_statement(&mut self) -> Result<DdlStatement, ParserError> {
        match &self.current_token {
            Some(Token { token_type: TokenType::Create, .. }) => {
                self.advance();
                if let Some(Token { token_type: TokenType::Table, .. }) = &self.current_token {
                    self.advance();
                    let table = self.parse_create_table()?;
                    Ok(DdlStatement::CreateTable(table))
                } else {
                    Err(self.unexpected_token_error("TABLE after CREATE"))
                }
            }
            _ => Err(self.unexpected_token_error("DDL statement")),
        }
    }

    fn parse_create_table(&mut self) -> Result<CreateTableStatement, ParserError> {
        let if_not_exists = if let Some(Token { token_type: TokenType::If, .. }) = &self.current_token {
            self.advance();
            self.expect_token(TokenType::Not)?;
            self.expect_token(TokenType::Exists)?;
            true
        } else {
            false
        };

        let table_name = self.parse_identifier()?;
        self.expect_token(TokenType::LeftParen)?;

        let mut columns = Vec::new();
        let mut constraints = Vec::new();

        loop {
            if let Some(Token { token_type: TokenType::RightParen, .. }) = &self.current_token {
                self.advance();
                break;
            }

            if let Some(Token { token_type: TokenType::Primary, .. }) = &self.current_token {
                self.advance();
                self.expect_token(TokenType::Key)?;
                self.expect_token(TokenType::LeftParen)?;
                let cols = self.parse_identifier_list()?;
                self.expect_token(TokenType::RightParen)?;
                constraints.push(TableConstraint::PrimaryKey { columns: cols });
            } else if let Some(Token { token_type: TokenType::Foreign, .. }) = &self.current_token {
                let constraint = self.parse_foreign_key_constraint()?;
                constraints.push(constraint);
            } else {
                // Parse column definition
                let column = self.parse_column_definition()?;
                columns.push(column);
            }

            if let Some(Token { token_type: TokenType::Comma, .. }) = &self.current_token {
                self.advance();
            } else {
                self.expect_token(TokenType::RightParen)?;
                break;
            }
        }

        Ok(CreateTableStatement {
            table_name,
            if_not_exists,
            columns,
            constraints,
            options: std::collections::HashMap::new(), // Not implemented yet
        })
    }

    fn parse_column_definition(&mut self) -> Result<ColumnDefinition, ParserError> {
        let name = self.parse_identifier()?;
        let data_type = self.parse_data_type()?;
        let data_type = match data_type.as_str() {
            "INT" | "INTEGER" => DataType::Integer { size: Some(32), unsigned: false },
            "BIGINT" => DataType::BigInt { unsigned: false },
            "SMALLINT" => DataType::SmallInt { unsigned: false },
            "TINYINT" => DataType::TinyInt { unsigned: false },
            "VARCHAR" => DataType::Varchar { length: None },
            "TEXT" => DataType::Text,
            "BOOLEAN" | "BOOL" => DataType::Boolean,
            "FLOAT" => DataType::Float { precision: None },
            "DOUBLE" => DataType::Double,
            "DATE" => DataType::Date,
            "TIME" => DataType::Time,
            "DATETIME" => DataType::DateTime,
            "TIMESTAMP" => DataType::Timestamp,
            "JSON" => DataType::Json,
            "UUID" => DataType::Uuid,
            _ => DataType::Custom(data_type),
        };

        let mut constraints = Vec::new();

        // Parse column constraints
        loop {
            match &self.current_token {
                Some(Token { token_type: TokenType::Not, .. }) => {
                    self.advance();
                    self.expect_token(TokenType::NullLiteral)?;
                    constraints.push(ColumnConstraint::NotNull);
                }
                Some(Token { token_type: TokenType::NullLiteral, .. }) => {
                    self.advance();
                    constraints.push(ColumnConstraint::Null);
                }
                Some(Token { token_type: TokenType::Primary, .. }) => {
                    self.advance();
                    self.expect_token(TokenType::Key)?;
                    constraints.push(ColumnConstraint::PrimaryKey);
                }
                Some(Token { token_type: TokenType::Unique, .. }) => {
                    self.advance();
                    constraints.push(ColumnConstraint::Unique);
                }
                Some(Token { token_type: TokenType::AutoIncrement, .. }) => {
                    self.advance();
                    constraints.push(ColumnConstraint::AutoIncrement);
                }
                Some(Token { token_type: TokenType::Default, .. }) => {
                    self.advance();
                    let value = self.parse_default_value()?;
                    constraints.push(ColumnConstraint::Default(value));
                }
                _ => break,
            }
        }

        Ok(ColumnDefinition {
            name,
            data_type,
            constraints,
        })
    }

    fn parse_foreign_key_constraint(&mut self) -> Result<TableConstraint, ParserError> {
        self.expect_token(TokenType::Foreign)?;
        self.expect_token(TokenType::Key)?;
        self.expect_token(TokenType::LeftParen)?;
        let columns = self.parse_identifier_list()?;
        self.expect_token(TokenType::RightParen)?;

        self.expect_token(TokenType::References)?;
        let referenced_table = self.parse_identifier()?;
        self.expect_token(TokenType::LeftParen)?;
        let referenced_columns = self.parse_identifier_list()?;
        self.expect_token(TokenType::RightParen)?;

        Ok(TableConstraint::ForeignKey {
            name: None,
            columns,
            referenced_table,
            referenced_columns,
            on_delete: None,
            on_update: None,
        })
    }

    fn parse_default_value(&mut self) -> Result<String, ParserError> {
        match &self.current_token {
            Some(Token { token_type: TokenType::StringLiteral(value), .. }) => {
                let val = value.clone();
                self.advance();
                Ok(format!("'{}'", val))
            }
            Some(Token { token_type: TokenType::IntegerLiteral(value), .. }) => {
                let val = value.to_string();
                self.advance();
                Ok(val)
            }
            Some(Token { token_type: TokenType::FloatLiteral(value), .. }) => {
                let val = value.to_string();
                self.advance();
                Ok(val)
            }
            Some(Token { token_type: TokenType::BooleanLiteral(value), .. }) => {
                let val = value.to_string().to_uppercase();
                self.advance();
                Ok(val)
            }
            Some(Token { token_type: TokenType::NullLiteral, .. }) => {
                self.advance();
                Ok("NULL".to_string())
            }
            _ => Err(self.unexpected_token_error("default value")),
        }
    }

    fn expect_token(&mut self, expected: TokenType) -> Result<(), ParserError> {
        match &self.current_token {
            Some(token) if token.token_type == expected => {
                self.advance();
                Ok(())
            }
            Some(token) => Err(ParserError::new(
                format!("Expected {:?}, found {:?}", expected, token.token_type),
                token.position.clone(),
            )),
            None => Err(ParserError::new(
                format!("Expected {:?}, found end of input", expected),
                Position::default(),
            )),
        }
    }

    fn advance(&mut self) {
        self.current_token = self.peek_token.take();
        self.peek_token = self.tokens.next();
    }

    fn synchronize(&mut self) {
        self.advance();

        while let Some(token) = &self.current_token {
            match token.token_type {
                TokenType::Semicolon => {
                    self.advance();
                    break;
                }
                TokenType::Select | TokenType::Insert | TokenType::Update |
                TokenType::Delete | TokenType::Create | TokenType::Alter |
                TokenType::Drop => break,
                _ => self.advance(),
            }
        }
    }

    fn unexpected_token_error(&self, expected: &str) -> ParserError {
        match &self.current_token {
            Some(token) => ParserError::new(
                format!("Expected {}, found {:?}", expected, token.token_type),
                token.position.clone(),
            ),
            None => ParserError::new(
                format!("Expected {}, found end of input", expected),
                Position::default(),
            ),
        }
    }
}

impl ParserError {
    pub fn new(message: impl Into<String>, position: Position) -> Self {
        Self {
            message: message.into(),
            position,
        }
    }
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} at line {}, column {}", self.message, self.position.line, self.position.column)
    }
}

impl std::error::Error for ParserError {}
