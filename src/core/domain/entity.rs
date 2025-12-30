use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Database {
    pub name: String,
    pub schemas: Vec<Schema>,
    pub metadata: DatabaseMetadata,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Schema {
    pub name: Option<String>,
    pub tables: Vec<Table>,
    pub views: Vec<View>,
    pub metadata: SchemaMetadata,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Table {
    pub name: String,
    pub schema: Option<String>,
    pub columns: Vec<Column>,
    pub primary_key: Option<PrimaryKey>,
    pub indexes: Vec<Index>,
    pub foreign_keys: Vec<ForeignKey>,
    pub unique_constraints: Vec<UniqueConstraint>,
    pub check_constraints: Vec<CheckConstraint>,
    pub metadata: TableMetadata,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct View {
    pub name: String,
    pub schema: Option<String>,
    pub definition: String,
    pub columns: Vec<Column>,
    pub dependencies: Vec<String>, // Referenced table/view names
    pub metadata: ViewMetadata,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Column {
    pub name: String,
    pub data_type: DataType,
    pub nullable: bool,
    pub default_value: Option<String>,
    pub auto_increment: bool,
    pub identity: Option<IdentitySpecification>,
    pub collation: Option<String>,
    pub metadata: ColumnMetadata,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DataType {
    // Numeric types
    Boolean,
    TinyInt { unsigned: bool },
    SmallInt { unsigned: bool },
    Integer { unsigned: bool },
    BigInt { unsigned: bool },
    Decimal { precision: Option<u32>, scale: Option<u32> },
    Float { precision: Option<u32> },
    Double,
    Real,

    // String types
    Char { length: Option<u32> },
    VarChar { length: Option<u32> },
    Text,
    NChar { length: Option<u32> },
    NVarChar { length: Option<u32> },
    NText,

    // Binary types
    Binary { length: Option<u32> },
    VarBinary { length: Option<u32> },
    Blob,
    Image,

    // Date/Time types
    Date,
    Time,
    DateTime,
    SmallDateTime,
    DateTime2,
    DateTimeOffset,
    Timestamp,

    // Other types
    Uuid,
    Json,
    JsonB,
    Xml,

    // Custom/Specialized types
    Custom(String),
    Array { element_type: Box<DataType>, dimensions: Option<u32> },
    Enum { values: Vec<String> },
    Set { values: Vec<String> },

    // Spatial types
    Geometry,
    Point,
    LineString,
    Polygon,
    MultiPoint,
    MultiLineString,
    MultiPolygon,
    GeometryCollection,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PrimaryKey {
    pub name: Option<String>,
    pub columns: Vec<String>,
    pub constraint_name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Index {
    pub name: String,
    pub table_name: String,
    pub columns: Vec<IndexColumn>,
    pub unique: bool,
    pub index_type: IndexType,
    pub where_clause: Option<String>,
    pub metadata: IndexMetadata,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IndexColumn {
    pub name: String,
    pub sort_order: SortOrder,
    pub nulls_position: NullsPosition,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum IndexType {
    BTree,
    Hash,
    Gist,
    Gin,
    SpGist,
    Brin,
    Custom(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SortOrder {
    Ascending,
    Descending,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NullsPosition {
    First,
    Last,
    Default,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ForeignKey {
    pub name: Option<String>,
    pub table_name: String,
    pub columns: Vec<String>,
    pub referenced_table: String,
    pub referenced_columns: Vec<String>,
    pub on_delete: ReferentialAction,
    pub on_update: ReferentialAction,
    pub constraint_name: Option<String>,
    pub metadata: ForeignKeyMetadata,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ReferentialAction {
    NoAction,
    Restrict,
    Cascade,
    SetNull,
    SetDefault,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UniqueConstraint {
    pub name: Option<String>,
    pub table_name: String,
    pub columns: Vec<String>,
    pub constraint_name: Option<String>,
    pub metadata: ConstraintMetadata,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CheckConstraint {
    pub name: Option<String>,
    pub table_name: String,
    pub expression: String,
    pub constraint_name: Option<String>,
    pub metadata: ConstraintMetadata,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IdentitySpecification {
    pub seed: i64,
    pub increment: i64,
    pub min_value: Option<i64>,
    pub max_value: Option<i64>,
    pub cycle: bool,
}

// Metadata structures for documentation and API generation

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DatabaseMetadata {
    pub description: Option<String>,
    pub version: Option<String>,
    pub created_at: Option<String>,
    pub author: Option<String>,
    pub tags: Vec<String>,
    pub custom_properties: HashMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SchemaMetadata {
    pub description: Option<String>,
    pub owner: Option<String>,
    pub created_at: Option<String>,
    pub custom_properties: HashMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TableMetadata {
    pub description: Option<String>,
    pub category: Option<String>,
    pub owner: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub row_count_estimate: Option<u64>,
    pub size_estimate: Option<String>,
    pub tags: Vec<String>,
    pub api_endpoints: Vec<ApiEndpoint>,
    pub custom_properties: HashMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ViewMetadata {
    pub description: Option<String>,
    pub category: Option<String>,
    pub owner: Option<String>,
    pub created_at: Option<String>,
    pub updatable: bool,
    pub tags: Vec<String>,
    pub custom_properties: HashMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ColumnMetadata {
    pub description: Option<String>,
    pub example_values: Vec<String>,
    pub validation_rules: Vec<ValidationRule>,
    pub sensitive: bool,
    pub deprecated: bool,
    pub tags: Vec<String>,
    pub api_field: Option<ApiField>,
    pub custom_properties: HashMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IndexMetadata {
    pub description: Option<String>,
    pub usage_estimate: Option<String>,
    pub maintenance_cost: Option<String>,
    pub custom_properties: HashMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ForeignKeyMetadata {
    pub description: Option<String>,
    pub relationship_type: RelationshipType,
    pub custom_properties: HashMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RelationshipType {
    OneToOne,
    OneToMany,
    ManyToOne,
    ManyToMany,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConstraintMetadata {
    pub description: Option<String>,
    pub business_rule: Option<String>,
    pub custom_properties: HashMap<String, String>,
}

// API-related metadata

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApiEndpoint {
    pub path: String,
    pub method: HttpMethod,
    pub description: Option<String>,
    pub parameters: Vec<ApiParameter>,
    pub response_schema: Option<String>,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Patch,
    Delete,
    Head,
    Options,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApiParameter {
    pub name: String,
    pub param_type: ParameterType,
    pub data_type: String,
    pub required: bool,
    pub description: Option<String>,
    pub default_value: Option<String>,
    pub validation: Option<ParameterValidation>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ParameterType {
    Path,
    Query,
    Body,
    Header,
    Form,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParameterValidation {
    pub min_length: Option<u32>,
    pub max_length: Option<u32>,
    pub min_value: Option<f64>,
    pub max_value: Option<f64>,
    pub pattern: Option<String>,
    pub enum_values: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApiField {
    pub name: String,
    pub data_type: String,
    pub required: bool,
    pub description: Option<String>,
    pub example: Option<String>,
    pub validation: Option<FieldValidation>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FieldValidation {
    pub min_length: Option<u32>,
    pub max_length: Option<u32>,
    pub min_value: Option<f64>,
    pub max_value: Option<f64>,
    pub pattern: Option<String>,
    pub format: Option<String>,
}

// Validation rules for data integrity

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ValidationRule {
    pub rule_type: ValidationRuleType,
    pub field: Option<String>,
    pub value: Option<String>,
    pub message: Option<String>,
    pub severity: ValidationSeverity,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ValidationRuleType {
    Required,
    Unique,
    Range,
    Pattern,
    Length,
    Custom,
    ForeignKey,
    Check,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ValidationSeverity {
    Error,
    Warning,
    Info,
}

// Domain services and utilities

impl Database {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            schemas: Vec::new(),
            metadata: DatabaseMetadata::default(),
        }
    }

    pub fn add_schema(&mut self, schema: Schema) {
        self.schemas.push(schema);
    }

    pub fn get_schema(&self, name: &str) -> Option<&Schema> {
        self.schemas.iter().find(|s| s.name.as_deref() == Some(name))
    }

    pub fn get_schema_mut(&mut self, name: &str) -> Option<&mut Schema> {
        self.schemas.iter_mut().find(|s| s.name.as_deref() == Some(name))
    }

    pub fn get_table(&self, schema_name: Option<&str>, table_name: &str) -> Option<&Table> {
        if let Some(schema) = schema_name.and_then(|name| self.get_schema(name)) {
            schema.get_table(table_name)
        } else {
            // Search in all schemas if no schema specified
            self.schemas.iter()
                .find_map(|schema| schema.get_table(table_name))
        }
    }

    pub fn get_all_tables(&self) -> Vec<&Table> {
        self.schemas.iter()
            .flat_map(|schema| &schema.tables)
            .collect()
    }

    pub fn get_foreign_key_relationships(&self) -> Vec<(&Table, &ForeignKey)> {
        self.get_all_tables()
            .into_iter()
            .flat_map(|table| table.foreign_keys.iter().map(move |fk| (table, fk)))
            .collect()
    }
}

impl Schema {
    pub fn new(name: Option<&str>) -> Self {
        Self {
            name: name.map(|s| s.to_string()),
            tables: Vec::new(),
            views: Vec::new(),
            metadata: SchemaMetadata::default(),
        }
    }

    pub fn add_table(&mut self, table: Table) {
        self.tables.push(table);
    }

    pub fn add_view(&mut self, view: View) {
        self.views.push(view);
    }

    pub fn get_table(&self, name: &str) -> Option<&Table> {
        self.tables.iter().find(|t| t.name == name)
    }

    pub fn get_table_mut(&mut self, name: &str) -> Option<&mut Table> {
        self.tables.iter_mut().find(|t| t.name == name)
    }

    pub fn get_view(&self, name: &str) -> Option<&View> {
        self.views.iter().find(|v| v.name == name)
    }

    pub fn get_view_mut(&mut self, name: &str) -> Option<&mut View> {
        self.views.iter_mut().find(|v| v.name == name)
    }
}

impl Table {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            schema: None,
            columns: Vec::new(),
            primary_key: None,
            indexes: Vec::new(),
            foreign_keys: Vec::new(),
            unique_constraints: Vec::new(),
            check_constraints: Vec::new(),
            metadata: TableMetadata::default(),
        }
    }

    pub fn with_schema(mut self, schema: &str) -> Self {
        self.schema = Some(schema.to_string());
        self
    }

    pub fn add_column(&mut self, column: Column) {
        self.columns.push(column);
    }

    pub fn get_column(&self, name: &str) -> Option<&Column> {
        self.columns.iter().find(|c| c.name == name)
    }

    pub fn get_column_mut(&mut self, name: &str) -> Option<&mut Column> {
        self.columns.iter_mut().find(|c| c.name == name)
    }

    pub fn set_primary_key(&mut self, columns: Vec<String>) {
        self.primary_key = Some(PrimaryKey {
            name: None,
            columns,
            constraint_name: None,
        });
    }

    pub fn add_foreign_key(&mut self, fk: ForeignKey) {
        self.foreign_keys.push(fk);
    }

    pub fn add_index(&mut self, index: Index) {
        self.indexes.push(index);
    }

    pub fn get_primary_key_columns(&self) -> Vec<&Column> {
        if let Some(pk) = &self.primary_key {
            pk.columns.iter()
                .filter_map(|col_name| self.get_column(col_name))
                .collect()
        } else {
            Vec::new()
        }
    }

    pub fn get_foreign_key_columns(&self) -> Vec<&Column> {
        self.foreign_keys.iter()
            .flat_map(|fk| &fk.columns)
            .filter_map(|col_name| self.get_column(col_name))
            .collect()
    }

    pub fn get_nullable_columns(&self) -> Vec<&Column> {
        self.columns.iter()
            .filter(|col| col.nullable)
            .collect()
    }

    pub fn get_required_columns(&self) -> Vec<&Column> {
        self.columns.iter()
            .filter(|col| !col.nullable)
            .collect()
    }

    pub fn has_column(&self, name: &str) -> bool {
        self.columns.iter().any(|c| c.name == name)
    }

    pub fn get_referenced_tables(&self) -> HashSet<String> {
        self.foreign_keys.iter()
            .map(|fk| fk.referenced_table.clone())
            .collect()
    }

    pub fn get_referencing_tables(&self, all_tables: &[&Table]) -> Vec<&Table> {
        all_tables.iter()
            .filter(|other_table| {
                other_table.foreign_keys.iter()
                    .any(|fk| fk.referenced_table == self.name)
            })
            .cloned()
            .collect()
    }
}

impl View {
    pub fn new(name: &str, definition: &str) -> Self {
        Self {
            name: name.to_string(),
            schema: None,
            definition: definition.to_string(),
            columns: Vec::new(),
            dependencies: Vec::new(),
            metadata: ViewMetadata::default(),
        }
    }

    pub fn with_schema(mut self, schema: &str) -> Self {
        self.schema = Some(schema.to_string());
        self
    }

    pub fn add_column(&mut self, column: Column) {
        self.columns.push(column);
    }

    pub fn add_dependency(&mut self, table_name: &str) {
        if !self.dependencies.contains(&table_name.to_string()) {
            self.dependencies.push(table_name.to_string());
        }
    }
}

impl Column {
    pub fn new(name: &str, data_type: DataType) -> Self {
        Self {
            name: name.to_string(),
            data_type,
            nullable: true,
            default_value: None,
            auto_increment: false,
            identity: None,
            collation: None,
            metadata: ColumnMetadata::default(),
        }
    }

    pub fn not_null(mut self) -> Self {
        self.nullable = false;
        self
    }

    pub fn with_default(mut self, value: &str) -> Self {
        self.default_value = Some(value.to_string());
        self
    }

    pub fn auto_increment(mut self) -> Self {
        self.auto_increment = true;
        self
    }

    pub fn with_identity(mut self, identity: IdentitySpecification) -> Self {
        self.identity = Some(identity);
        self
    }

    pub fn is_primary_key(&self, primary_key: &Option<PrimaryKey>) -> bool {
        if let Some(pk) = primary_key {
            pk.columns.contains(&self.name)
        } else {
            false
        }
    }

    pub fn is_foreign_key(&self, foreign_keys: &[ForeignKey]) -> bool {
        foreign_keys.iter()
            .any(|fk| fk.columns.contains(&self.name))
    }

    pub fn is_unique(&self, unique_constraints: &[UniqueConstraint]) -> bool {
        unique_constraints.iter()
            .any(|uc| uc.columns.contains(&self.name))
    }

    pub fn get_data_type_name(&self) -> String {
        match &self.data_type {
            DataType::Boolean => "BOOLEAN".to_string(),
            DataType::TinyInt { unsigned } => {
                if *unsigned { "TINYINT UNSIGNED" } else { "TINYINT" }.to_string()
            }
            DataType::SmallInt { unsigned } => {
                if *unsigned { "SMALLINT UNSIGNED" } else { "SMALLINT" }.to_string()
            }
            DataType::Integer { unsigned } => {
                if *unsigned { "INTEGER UNSIGNED" } else { "INTEGER" }.to_string()
            }
            DataType::BigInt { unsigned } => {
                if *unsigned { "BIGINT UNSIGNED" } else { "BIGINT" }.to_string()
            }
            DataType::Decimal { precision, scale } => {
                match (precision, scale) {
                    (Some(p), Some(s)) => format!("DECIMAL({}, {})", p, s),
                    (Some(p), None) => format!("DECIMAL({})", p),
                    _ => "DECIMAL".to_string(),
                }
            }
            DataType::Float { precision } => {
                precision.map_or("FLOAT".to_string(), |p| format!("FLOAT({})", p))
            }
            DataType::Double => "DOUBLE".to_string(),
            DataType::Real => "REAL".to_string(),
            DataType::Char { length } => {
                length.map_or("CHAR".to_string(), |l| format!("CHAR({})", l))
            }
            DataType::VarChar { length } => {
                length.map_or("VARCHAR".to_string(), |l| format!("VARCHAR({})", l))
            }
            DataType::Text => "TEXT".to_string(),
            DataType::NChar { length } => {
                length.map_or("NCHAR".to_string(), |l| format!("NCHAR({})", l))
            }
            DataType::NVarChar { length } => {
                length.map_or("NVARCHAR".to_string(), |l| format!("NVARCHAR({})", l))
            }
            DataType::NText => "NTEXT".to_string(),
            DataType::Binary { length } => {
                length.map_or("BINARY".to_string(), |l| format!("BINARY({})", l))
            }
            DataType::VarBinary { length } => {
                length.map_or("VARBINARY".to_string(), |l| format!("VARBINARY({})", l))
            }
            DataType::Blob => "BLOB".to_string(),
            DataType::Image => "IMAGE".to_string(),
            DataType::Date => "DATE".to_string(),
            DataType::Time => "TIME".to_string(),
            DataType::DateTime => "DATETIME".to_string(),
            DataType::SmallDateTime => "SMALLDATETIME".to_string(),
            DataType::DateTime2 => "DATETIME2".to_string(),
            DataType::DateTimeOffset => "DATETIMEOFFSET".to_string(),
            DataType::Timestamp => "TIMESTAMP".to_string(),
            DataType::Uuid => "UUID".to_string(),
            DataType::Json => "JSON".to_string(),
            DataType::JsonB => "JSONB".to_string(),
            DataType::Xml => "XML".to_string(),
            DataType::Custom(name) => name.clone(),
            DataType::Array { element_type, dimensions } => {
                let base_type = element_type.get_data_type_name();
                dimensions.map_or(
                    format!("{}[]", base_type),
                    |d| format!("{}{}", base_type, "[]".repeat(d as usize))
                )
            }
            DataType::Enum { values } => {
                format!("ENUM({})", values.join(", "))
            }
            DataType::Set { values } => {
                format!("SET({})", values.join(", "))
            }
            DataType::Geometry => "GEOMETRY".to_string(),
            DataType::Point => "POINT".to_string(),
            DataType::LineString => "LINESTRING".to_string(),
            DataType::Polygon => "POLYGON".to_string(),
            DataType::MultiPoint => "MULTIPOINT".to_string(),
            DataType::MultiLineString => "MULTILINESTRING".to_string(),
            DataType::MultiPolygon => "MULTIPOLYGON".to_string(),
            DataType::GeometryCollection => "GEOMETRYCOLLECTION".to_string(),
        }
    }
}

impl DataType {
    pub fn get_data_type_name(&self) -> String {
        match self {
            DataType::Array { element_type, .. } => element_type.get_data_type_name(),
            _ => self.get_simple_name(),
        }
    }

    pub fn get_simple_name(&self) -> String {
        match self {
            DataType::Boolean => "BOOLEAN",
            DataType::TinyInt { .. } => "TINYINT",
            DataType::SmallInt { .. } => "SMALLINT",
            DataType::Integer { .. } => "INTEGER",
            DataType::BigInt { .. } => "BIGINT",
            DataType::Decimal { .. } => "DECIMAL",
            DataType::Float { .. } => "FLOAT",
            DataType::Double => "DOUBLE",
            DataType::Real => "REAL",
            DataType::Char { .. } => "CHAR",
            DataType::VarChar { .. } => "VARCHAR",
            DataType::Text => "TEXT",
            DataType::NChar { .. } => "NCHAR",
            DataType::NVarChar { .. } => "NVARCHAR",
            DataType::NText => "NTEXT",
            DataType::Binary { .. } => "BINARY",
            DataType::VarBinary { .. } => "VARBINARY",
            DataType::Blob => "BLOB",
            DataType::Image => "IMAGE",
            DataType::Date => "DATE",
            DataType::Time => "TIME",
            DataType::DateTime => "DATETIME",
            DataType::SmallDateTime => "SMALLDATETIME",
            DataType::DateTime2 => "DATETIME2",
            DataType::DateTimeOffset => "DATETIMEOFFSET",
            DataType::Timestamp => "TIMESTAMP",
            DataType::Uuid => "UUID",
            DataType::Json => "JSON",
            DataType::JsonB => "JSONB",
            DataType::Xml => "XML",
            DataType::Custom(name) => name,
            DataType::Array { .. } => "ARRAY",
            DataType::Enum { .. } => "ENUM",
            DataType::Set { .. } => "SET",
            DataType::Geometry => "GEOMETRY",
            DataType::Point => "POINT",
            DataType::LineString => "LINESTRING",
            DataType::Polygon => "POLYGON",
            DataType::MultiPoint => "MULTIPOINT",
            DataType::MultiLineString => "MULTILINESTRING",
            DataType::MultiPolygon => "MULTIPOLYGON",
            DataType::GeometryCollection => "GEOMETRYCOLLECTION",
        }.to_string()
    }

    pub fn is_numeric(&self) -> bool {
        matches!(self,
            DataType::TinyInt { .. } | DataType::SmallInt { .. } | DataType::Integer { .. } |
            DataType::BigInt { .. } | DataType::Decimal { .. } | DataType::Float { .. } |
            DataType::Double | DataType::Real
        )
    }

    pub fn is_text(&self) -> bool {
        matches!(self,
            DataType::Char { .. } | DataType::VarChar { .. } | DataType::Text |
            DataType::NChar { .. } | DataType::NVarChar { .. } | DataType::NText
        )
    }

    pub fn is_temporal(&self) -> bool {
        matches!(self,
            DataType::Date | DataType::Time | DataType::DateTime | DataType::SmallDateTime |
            DataType::DateTime2 | DataType::DateTimeOffset | DataType::Timestamp
        )
    }

    pub fn is_spatial(&self) -> bool {
        matches!(self,
            DataType::Geometry | DataType::Point | DataType::LineString | DataType::Polygon |
            DataType::MultiPoint | DataType::MultiLineString | DataType::MultiPolygon |
            DataType::GeometryCollection
        )
    }
}

impl fmt::Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get_data_type_name())
    }
}

// Default implementations

impl Default for DatabaseMetadata {
    fn default() -> Self {
        Self {
            description: None,
            version: None,
            created_at: None,
            author: None,
            tags: Vec::new(),
            custom_properties: HashMap::new(),
        }
    }
}

impl Default for SchemaMetadata {
    fn default() -> Self {
        Self {
            description: None,
            owner: None,
            created_at: None,
            custom_properties: HashMap::new(),
        }
    }
}

impl Default for TableMetadata {
    fn default() -> Self {
        Self {
            description: None,
            category: None,
            owner: None,
            created_at: None,
            updated_at: None,
            row_count_estimate: None,
            size_estimate: None,
            tags: Vec::new(),
            api_endpoints: Vec::new(),
            custom_properties: HashMap::new(),
        }
    }
}

impl Default for ViewMetadata {
    fn default() -> Self {
        Self {
            description: None,
            category: None,
            owner: None,
            created_at: None,
            updatable: false,
            tags: Vec::new(),
            custom_properties: HashMap::new(),
        }
    }
}

impl Default for ColumnMetadata {
    fn default() -> Self {
        Self {
            description: None,
            example_values: Vec::new(),
            validation_rules: Vec::new(),
            sensitive: false,
            deprecated: false,
            tags: Vec::new(),
            api_field: None,
            custom_properties: HashMap::new(),
        }
    }
}

impl Default for IndexMetadata {
    fn default() -> Self {
        Self {
            description: None,
            usage_estimate: None,
            maintenance_cost: None,
            custom_properties: HashMap::new(),
        }
    }
}

impl Default for ForeignKeyMetadata {
    fn default() -> Self {
        Self {
            description: None,
            relationship_type: RelationshipType::ManyToOne,
            custom_properties: HashMap::new(),
        }
    }
}

impl Default for ConstraintMetadata {
    fn default() -> Self {
        Self {
            description: None,
            business_rule: None,
            custom_properties: HashMap::new(),
        }
    }
}
