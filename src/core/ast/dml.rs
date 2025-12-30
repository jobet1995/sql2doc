use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LiteralValue {
    Null,
    Boolean(bool),
    Integer(i64),
    Float(f64),
    String(String),
    Binary(Vec<u8>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Expression {
    Literal(LiteralValue),
    Column(String),
    QualifiedColumn { table: String, column: String },
    Function { name: String, args: Vec<Expression> },
    BinaryOp { left: Box<Expression>, op: BinaryOperator, right: Box<Expression> },
    UnaryOp { op: UnaryOperator, expr: Box<Expression> },
    Between { expr: Box<Expression>, min: Box<Expression>, max: Box<Expression> },
    In { expr: Box<Expression>, values: Vec<Expression> },
    NotIn { expr: Box<Expression>, values: Vec<Expression> },
    Exists(Box<SelectStatement>),
    Subquery(Box<SelectStatement>),
    Case { operand: Option<Box<Expression>>, when_clauses: Vec<WhenClause>, else_expr: Option<Box<Expression>> },
    Cast { expr: Box<Expression>, data_type: String },
    WindowFunction { function: Box<Expression>, window: WindowSpecification },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WhenClause {
    pub condition: Expression,
    pub result: Expression,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BinaryOperator {
    Eq,
    Neq,
    Lt,
    Lte,
    Gt,
    Gte,
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    And,
    Or,
    Like,
    NotLike,
    ILike,
    NotILike,
    Concat,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UnaryOperator {
    Not,
    Plus,
    Minus,
    BitwiseNot,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum JoinType {
    Inner,
    Left,
    Right,
    Full,
    Cross,
    LeftOuter,
    RightOuter,
    FullOuter,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JoinClause {
    pub join_type: JoinType,
    pub table: TableReference,
    pub condition: Option<JoinCondition>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum JoinCondition {
    On(Expression),
    Using(Vec<String>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TableReference {
    Table { name: String, alias: Option<String> },
    Subquery { query: Box<SelectStatement>, alias: String },
    Join { left: Box<TableReference>, right: Box<TableReference>, joins: Vec<JoinClause> },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderByClause {
    pub expr: Expression,
    pub ascending: bool,
    pub nulls_first: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GroupByClause {
    pub expressions: Vec<Expression>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FrameBound {
    UnboundedPreceding,
    Preceding(u64),
    CurrentRow,
    Following(u64),
    UnboundedFollowing,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WindowFrame {
    pub start: FrameBound,
    pub end: Option<FrameBound>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WindowSpecification {
    pub partition_by: Vec<Expression>,
    pub order_by: Vec<OrderByClause>,
    pub frame: Option<WindowFrame>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommonTableExpression {
    pub name: String,
    pub columns: Vec<String>,
    pub query: SelectStatement,
    pub recursive: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SelectStatement {
    pub with: Vec<CommonTableExpression>,
    pub distinct: bool,
    pub select_list: Vec<SelectItem>,
    pub from: Vec<TableReference>,
    pub where_clause: Option<Expression>,
    pub group_by: Option<GroupByClause>,
    pub having: Option<Expression>,
    pub order_by: Vec<OrderByClause>,
    pub limit: Option<u64>,
    pub offset: Option<u64>,
    pub unions: Vec<UnionClause>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SelectItem {
    Expression { expr: Expression, alias: Option<String> },
    Wildcard,
    QualifiedWildcard { qualifier: String },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnionClause {
    pub select: SelectStatement,
    pub all: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InsertStatement {
    pub table_name: String,
    pub columns: Vec<String>,
    pub values: Vec<Vec<Expression>>,
    pub select: Option<SelectStatement>,
    pub on_conflict: Option<OnConflictClause>,
    pub returning: Vec<SelectItem>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OnConflictClause {
    pub target: ConflictTarget,
    pub action: ConflictAction,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConflictTarget {
    Columns(Vec<String>),
    OnConstraint(String),
    Where(Expression),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConflictAction {
    DoNothing,
    DoUpdate { assignments: HashMap<String, Expression>, where_clause: Option<Expression> },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateStatement {
    pub table_name: String,
    pub alias: Option<String>,
    pub assignments: HashMap<String, Expression>,
    pub from: Vec<TableReference>,
    pub where_clause: Option<Expression>,
    pub returning: Vec<SelectItem>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteStatement {
    pub table_name: String,
    pub alias: Option<String>,
    pub using: Vec<TableReference>,
    pub where_clause: Option<Expression>,
    pub returning: Vec<SelectItem>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DmlStatement {
    Select(Box<SelectStatement>),
    Insert(Box<InsertStatement>),
    Update(Box<UpdateStatement>),
    Delete(Box<DeleteStatement>),
}

impl Expression {
    pub fn column(name: &str) -> Self {
        Expression::Column(name.to_string())
    }

    pub fn qualified_column(table: &str, column: &str) -> Self {
        Expression::QualifiedColumn {
            table: table.to_string(),
            column: column.to_string(),
        }
    }

    pub fn literal(value: LiteralValue) -> Self {
        Expression::Literal(value)
    }

    pub fn function(name: &str, args: Vec<Expression>) -> Self {
        Expression::Function {
            name: name.to_string(),
            args,
        }
    }

    pub fn binary_op(left: Expression, op: BinaryOperator, right: Expression) -> Self {
        Expression::BinaryOp {
            left: Box::new(left),
            op,
            right: Box::new(right),
        }
    }

    pub fn unary_op(op: UnaryOperator, expr: Expression) -> Self {
        Expression::UnaryOp {
            op,
            expr: Box::new(expr),
        }
    }

    pub fn is_aggregate(&self) -> bool {
        match self {
            Expression::Function { name, .. } => matches!(
                name.to_uppercase().as_str(),
                "COUNT" | "SUM" | "AVG" | "MIN" | "MAX" | "STDDEV" | "VARIANCE"
            ),
            Expression::WindowFunction { .. } => true,
            _ => false,
        }
    }

    pub fn contains_aggregates(&self) -> bool {
        match self {
            Expression::Function { args, .. } => {
                args.iter().any(|arg| arg.contains_aggregates()) || self.is_aggregate()
            }
            Expression::BinaryOp { left, right, .. } => {
                left.contains_aggregates() || right.contains_aggregates()
            }
            Expression::UnaryOp { expr, .. } => expr.contains_aggregates(),
            Expression::WindowFunction { .. } => true,
            _ => false,
        }
    }
}

impl Default for SelectStatement {
    fn default() -> Self {
        Self::new()
    }
}

impl SelectStatement {
    pub fn new() -> Self {
        Self {
            with: Vec::new(),
            distinct: false,
            select_list: Vec::new(),
            from: Vec::new(),
            where_clause: None,
            group_by: None,
            having: None,
            order_by: Vec::new(),
            limit: None,
            offset: None,
            unions: Vec::new(),
        }
    }

    pub fn with_select_list(mut self, select_list: Vec<SelectItem>) -> Self {
        self.select_list = select_list;
        self
    }

    pub fn with_from(mut self, from: Vec<TableReference>) -> Self {
        self.from = from;
        self
    }

    pub fn with_where(mut self, where_clause: Expression) -> Self {
        self.where_clause = Some(where_clause);
        self
    }

    pub fn with_order_by(mut self, order_by: Vec<OrderByClause>) -> Self {
        self.order_by = order_by;
        self
    }

    pub fn has_aggregates(&self) -> bool {
        self.select_list.iter().any(|item| match item {
            SelectItem::Expression { expr, .. } => expr.contains_aggregates(),
            _ => false,
        }) || self.having.as_ref().is_some_and(|h| h.contains_aggregates())
    }

    pub fn has_group_by(&self) -> bool {
        self.group_by.is_some()
    }

    pub fn requires_group_by(&self) -> bool {
        self.has_aggregates() && !self.select_list.iter().all(|item| match item {
            SelectItem::Expression { expr, .. } => expr.is_aggregate() || matches!(expr, Expression::Column(_) | Expression::QualifiedColumn { .. }),
            SelectItem::Wildcard | SelectItem::QualifiedWildcard { .. } => false,
        })
    }
}

impl InsertStatement {
    pub fn new(table_name: &str) -> Self {
        Self {
            table_name: table_name.to_string(),
            columns: Vec::new(),
            values: Vec::new(),
            select: None,
            on_conflict: None,
            returning: Vec::new(),
        }
    }

    pub fn with_columns(mut self, columns: Vec<String>) -> Self {
        self.columns = columns;
        self
    }

    pub fn with_values(mut self, values: Vec<Vec<Expression>>) -> Self {
        self.values = values;
        self
    }

    pub fn with_select(mut self, select: SelectStatement) -> Self {
        self.select = Some(select);
        self
    }

    pub fn value_count(&self) -> usize {
        if let Some(select) = &self.select {
            select.select_list.len()
        } else {
            self.values.first().map(|row| row.len()).unwrap_or(0)
        }
    }

    pub fn is_bulk_insert(&self) -> bool {
        self.values.len() > 1 || self.select.is_some()
    }
}

impl UpdateStatement {
    pub fn new(table_name: &str) -> Self {
        Self {
            table_name: table_name.to_string(),
            alias: None,
            assignments: HashMap::new(),
            from: Vec::new(),
            where_clause: None,
            returning: Vec::new(),
        }
    }

    pub fn with_assignments(mut self, assignments: HashMap<String, Expression>) -> Self {
        self.assignments = assignments;
        self
    }

    pub fn with_where(mut self, where_clause: Expression) -> Self {
        self.where_clause = Some(where_clause);
        self
    }

    pub fn assignment_count(&self) -> usize {
        self.assignments.len()
    }

    pub fn has_where_clause(&self) -> bool {
        self.where_clause.is_some()
    }
}

impl DeleteStatement {
    pub fn new(table_name: &str) -> Self {
        Self {
            table_name: table_name.to_string(),
            alias: None,
            using: Vec::new(),
            where_clause: None,
            returning: Vec::new(),
        }
    }

    pub fn with_where(mut self, where_clause: Expression) -> Self {
        self.where_clause = Some(where_clause);
        self
    }

    pub fn has_where_clause(&self) -> bool {
        self.where_clause.is_some()
    }

    pub fn uses_other_tables(&self) -> bool {
        !self.using.is_empty()
    }
}

impl TableReference {
    pub fn table(name: &str) -> Self {
        TableReference::Table {
            name: name.to_string(),
            alias: None,
        }
    }

    pub fn table_with_alias(name: &str, alias: &str) -> Self {
        TableReference::Table {
            name: name.to_string(),
            alias: Some(alias.to_string()),
        }
    }

    pub fn subquery(query: SelectStatement, alias: &str) -> Self {
        TableReference::Subquery {
            query: Box::new(query),
            alias: alias.to_string(),
        }
    }

    pub fn join(left: TableReference, right: TableReference, joins: Vec<JoinClause>) -> Self {
        TableReference::Join {
            left: Box::new(left),
            right: Box::new(right),
            joins,
        }
    }

    pub fn get_name(&self) -> Option<&str> {
        match self {
            TableReference::Table { name, .. } => Some(name),
            TableReference::Subquery { alias, .. } => Some(alias),
            TableReference::Join { .. } => None,
        }
    }

    pub fn get_alias(&self) -> Option<&str> {
        match self {
            TableReference::Table { alias, .. } => alias.as_deref(),
            TableReference::Subquery { alias, .. } => Some(alias),
            TableReference::Join { .. } => None,
        }
    }
}
