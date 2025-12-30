use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

use crate::core::parse::SqlDialect;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Identifier {
    pub name: String,
    pub quoted: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QualifiedName {
    pub parts: Vec<Identifier>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SchemaReference {
    pub catalog: Option<String>,
    pub schema: Option<String>,
    pub object: String,
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Position {
    pub line: usize,
    pub column: usize,
    pub index: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Location {
    pub start: Position,
    pub end: Position,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AstNode<T> {
    pub node: T,
    pub location: Option<Location>,
    pub metadata: HashMap<String, String>,
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParseOptions {
    pub dialect: SqlDialect,
    pub strict_mode: bool,
    pub preserve_comments: bool,
    pub case_sensitive: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DocumentationMetadata {
    pub title: Option<String>,
    pub description: Option<String>,
    pub tags: Vec<String>,
    pub examples: Vec<String>,
    pub deprecated: bool,
    pub version: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApiMetadata {
    pub http_method: Option<String>,
    pub endpoint: Option<String>,
    pub parameters: Vec<ApiParameter>,
    pub response_schema: Option<String>,
    pub authentication_required: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApiParameter {
    pub name: String,
    pub param_type: String,
    pub required: bool,
    pub description: Option<String>,
    pub default_value: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ValidationRule {
    pub rule_type: ValidationRuleType,
    pub field: String,
    pub value: Option<String>,
    pub message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ValidationRuleType {
    Required,
    Unique,
    Range,
    Pattern,
    Custom,
    ForeignKey,
    Check,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ColumnMetadata {
    pub name: String,
    pub data_type: String,
    pub nullable: bool,
    pub default_value: Option<String>,
    pub description: Option<String>,
    pub validation_rules: Vec<ValidationRule>,
    pub api_metadata: Option<ApiMetadata>,
    pub documentation: Option<DocumentationMetadata>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TableMetadata {
    pub name: String,
    pub schema: Option<String>,
    pub description: Option<String>,
    pub columns: Vec<ColumnMetadata>,
    pub primary_key: Vec<String>,
    pub foreign_keys: Vec<ForeignKeyMetadata>,
    pub indexes: Vec<IndexMetadata>,
    pub api_endpoints: Vec<ApiMetadata>,
    pub documentation: Option<DocumentationMetadata>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ForeignKeyMetadata {
    pub name: Option<String>,
    pub columns: Vec<String>,
    pub referenced_table: String,
    pub referenced_columns: Vec<String>,
    pub on_delete: Option<String>,
    pub on_update: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IndexMetadata {
    pub name: String,
    pub columns: Vec<String>,
    pub unique: bool,
    pub index_type: Option<String>,
    pub where_clause: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SchemaMetadata {
    pub name: Option<String>,
    pub tables: Vec<TableMetadata>,
    pub views: Vec<ViewMetadata>,
    pub documentation: Option<DocumentationMetadata>,
    pub version: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ViewMetadata {
    pub name: String,
    pub schema: Option<String>,
    pub definition: String,
    pub columns: Vec<ColumnMetadata>,
    pub documentation: Option<DocumentationMetadata>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QueryMetadata {
    pub query_type: QueryType,
    pub tables_accessed: Vec<String>,
    pub columns_accessed: Vec<String>,
    pub estimated_complexity: QueryComplexity,
    pub documentation: Option<DocumentationMetadata>,
    pub api_metadata: Option<ApiMetadata>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum QueryType {
    Select,
    Insert,
    Update,
    Delete,
    DDL,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum QueryComplexity {
    Simple,
    Medium,
    Complex,
    VeryComplex,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AstParseError {
    pub message: String,
    pub location: Option<Location>,
    pub error_type: AstParseErrorType,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AstParseErrorType {
    SyntaxError,
    SemanticError,
    UnsupportedFeature,
    TypeMismatch,
    MissingReference,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AstValidationError {
    pub message: String,
    pub location: Option<Location>,
    pub severity: ValidationSeverity,
    pub rule: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ValidationSeverity {
    Error,
    Warning,
    Info,
}

impl Identifier {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            quoted: false,
        }
    }

    pub fn quoted(name: &str) -> Self {
        Self {
            name: name.to_string(),
            quoted: true,
        }
    }
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.quoted {
            write!(f, "\"{}\"", self.name)
        } else {
            write!(f, "{}", self.name)
        }
    }
}

impl QualifiedName {
    pub fn new(parts: Vec<Identifier>) -> Self {
        Self { parts }
    }

    pub fn len(&self) -> usize {
        self.parts.len()
    }

    pub fn is_empty(&self) -> bool {
        self.parts.is_empty()
    }

    pub fn last(&self) -> Option<&Identifier> {
        self.parts.last()
    }
}

impl FromStr for QualifiedName {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s
            .split('.')
            .map(Identifier::new)
            .collect();
        Ok(Self { parts })
    }
}

impl fmt::Display for QualifiedName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let parts: Vec<String> = self.parts.iter().map(|part| part.to_string()).collect();
        write!(f, "{}", parts.join("."))
    }

}

impl SchemaReference {
    pub fn new(object: &str) -> Self {
        Self {
            catalog: None,
            schema: None,
            object: object.to_string(),
        }
    }

    pub fn with_schema(schema: &str, object: &str) -> Self {
        Self {
            catalog: None,
            schema: Some(schema.to_string()),
            object: object.to_string(),
        }
    }

    pub fn with_catalog(catalog: &str, schema: &str, object: &str) -> Self {
        Self {
            catalog: Some(catalog.to_string()),
            schema: Some(schema.to_string()),
            object: object.to_string(),
        }
    }
}

impl fmt::Display for SchemaReference {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut parts = Vec::new();
        if let Some(catalog) = &self.catalog {
            parts.push(catalog.clone());
        }
        if let Some(schema) = &self.schema {
            parts.push(schema.clone());
        }
        parts.push(self.object.clone());
        write!(f, "{}", parts.join("."))
    }

}

impl<T> AstNode<T> {
    pub fn new(node: T) -> Self {
        Self {
            node,
            location: None,
            metadata: HashMap::new(),
        }
    }

    pub fn with_location(mut self, location: Location) -> Self {
        self.location = Some(location);
        self
    }

    pub fn with_metadata(mut self, key: &str, value: &str) -> Self {
        self.metadata.insert(key.to_string(), value.to_string());
        self
    }

    pub fn get_metadata(&self, key: &str) -> Option<&String> {
        self.metadata.get(key)
    }
}

impl Default for ParseOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl ParseOptions {
    pub fn new() -> Self {
        Self {
            dialect: SqlDialect::Standard,
            strict_mode: false,
            preserve_comments: false,
            case_sensitive: false,
        }
    }

    pub fn with_dialect(mut self, dialect: SqlDialect) -> Self {
        self.dialect = dialect;
        self
    }

    pub fn strict(mut self) -> Self {
        self.strict_mode = true;
        self
    }

    pub fn case_sensitive(mut self) -> Self {
        self.case_sensitive = true;
        self
    }
}

impl Default for DocumentationMetadata {
    fn default() -> Self {
        Self::new()
    }
}

impl DocumentationMetadata {
    pub fn new() -> Self {
        Self {
            title: None,
            description: None,
            tags: Vec::new(),
            examples: Vec::new(),
            deprecated: false,
            version: None,
        }
    }

    pub fn with_title(mut self, title: &str) -> Self {
        self.title = Some(title.to_string());
        self
    }

    pub fn with_description(mut self, description: &str) -> Self {
        self.description = Some(description.to_string());
        self
    }

    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }
}

impl ColumnMetadata {
    pub fn new(name: &str, data_type: &str) -> Self {
        Self {
            name: name.to_string(),
            data_type: data_type.to_string(),
            nullable: true,
            default_value: None,
            description: None,
            validation_rules: Vec::new(),
            api_metadata: None,
            documentation: None,
        }
    }

    pub fn not_null(mut self) -> Self {
        self.nullable = false;
        self
    }

    pub fn with_description(mut self, description: &str) -> Self {
        self.description = Some(description.to_string());
        self
    }

    pub fn add_validation_rule(mut self, rule: ValidationRule) -> Self {
        self.validation_rules.push(rule);
        self
    }
}

impl TableMetadata {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            schema: None,
            description: None,
            columns: Vec::new(),
            primary_key: Vec::new(),
            foreign_keys: Vec::new(),
            indexes: Vec::new(),
            api_endpoints: Vec::new(),
            documentation: None,
        }
    }

    pub fn with_schema(mut self, schema: &str) -> Self {
        self.schema = Some(schema.to_string());
        self
    }

    pub fn with_columns(mut self, columns: Vec<ColumnMetadata>) -> Self {
        self.columns = columns;
        self
    }

    pub fn with_primary_key(mut self, columns: Vec<String>) -> Self {
        self.primary_key = columns;
        self
    }

    pub fn get_column(&self, name: &str) -> Option<&ColumnMetadata> {
        self.columns.iter().find(|col| col.name == name)
    }

    pub fn get_column_mut(&mut self, name: &str) -> Option<&mut ColumnMetadata> {
        self.columns.iter_mut().find(|col| col.name == name)
    }
}

impl Default for SchemaMetadata {
    fn default() -> Self {
        Self::new()
    }
}

impl SchemaMetadata {
    pub fn new() -> Self {
        Self {
            name: None,
            tables: Vec::new(),
            views: Vec::new(),
            documentation: None,
            version: None,
        }
    }

    pub fn with_name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn add_table(mut self, table: TableMetadata) -> Self {
        self.tables.push(table);
        self
    }

    pub fn get_table(&self, name: &str) -> Option<&TableMetadata> {
        self.tables.iter().find(|table| table.name == name)
    }

    pub fn get_table_mut(&mut self, name: &str) -> Option<&mut TableMetadata> {
        self.tables.iter_mut().find(|table| table.name == name)
    }
}

impl QueryMetadata {
    pub fn new(query_type: QueryType) -> Self {
        Self {
            query_type,
            tables_accessed: Vec::new(),
            columns_accessed: Vec::new(),
            estimated_complexity: QueryComplexity::Simple,
            documentation: None,
            api_metadata: None,
        }
    }

    pub fn with_tables(mut self, tables: Vec<String>) -> Self {
        self.tables_accessed = tables;
        self
    }

    pub fn with_columns(mut self, columns: Vec<String>) -> Self {
        self.columns_accessed = columns;
        self
    }

    pub fn with_complexity(mut self, complexity: QueryComplexity) -> Self {
        self.estimated_complexity = complexity;
        self
    }
}

impl fmt::Display for AstParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for AstParseError {}

impl fmt::Display for AstValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for AstValidationError {}

impl Default for Position {
    fn default() -> Self {
        Self {
            line: 1,
            column: 1,
            index: 0,
        }
    }
}
