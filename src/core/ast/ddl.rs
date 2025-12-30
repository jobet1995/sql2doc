use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DataType {
    Boolean,
    Integer { size: Option<u32>, unsigned: bool },
    BigInt { unsigned: bool },
    SmallInt { unsigned: bool },
    TinyInt { unsigned: bool },
    Float { precision: Option<u32> },
    Double,
    Decimal { precision: Option<u32>, scale: Option<u32> },
    Varchar { length: Option<u32> },
    Char { length: Option<u32> },
    Text,
    Binary { length: Option<u32> },
    Varbinary { length: Option<u32> },
    Blob,
    Date,
    Time,
    DateTime,
    Timestamp,
    Json,
    Uuid,
    Custom(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ColumnConstraint {
    NotNull,
    Null,
    Default(String),
    Unique,
    PrimaryKey,
    AutoIncrement,
    Check(String),
    ForeignKey {
        table: String,
        column: String,
        on_delete: Option<ReferentialAction>,
        on_update: Option<ReferentialAction>,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ReferentialAction {
    Cascade,
    Restrict,
    SetNull,
    SetDefault,
    NoAction,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ColumnDefinition {
    pub name: String,
    pub data_type: DataType,
    pub constraints: Vec<ColumnConstraint>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TableConstraint {
    PrimaryKey { columns: Vec<String> },
    Unique { name: Option<String>, columns: Vec<String> },
    ForeignKey {
        name: Option<String>,
        columns: Vec<String>,
        referenced_table: String,
        referenced_columns: Vec<String>,
        on_delete: Option<ReferentialAction>,
        on_update: Option<ReferentialAction>,
    },
    Check { name: Option<String>, expression: String },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateTableStatement {
    pub table_name: String,
    pub if_not_exists: bool,
    pub columns: Vec<ColumnDefinition>,
    pub constraints: Vec<TableConstraint>,
    pub options: HashMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AlterTableAction {
    AddColumn { column: ColumnDefinition },
    DropColumn { column_name: String, if_exists: bool },
    AlterColumn {
        column_name: String,
        new_data_type: Option<DataType>,
        add_constraints: Vec<ColumnConstraint>,
        drop_constraints: Vec<String>,
    },
    RenameColumn { old_name: String, new_name: String },
    AddConstraint { constraint: TableConstraint },
    DropConstraint { constraint_name: String, if_exists: bool },
    RenameTable { new_name: String },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AlterTableStatement {
    pub table_name: String,
    pub if_exists: bool,
    pub actions: Vec<AlterTableAction>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DropTableStatement {
    pub table_names: Vec<String>,
    pub if_exists: bool,
    pub cascade: bool,
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
pub struct CreateIndexStatement {
    pub index_name: String,
    pub table_name: String,
    pub if_not_exists: bool,
    pub unique: bool,
    pub index_type: Option<IndexType>,
    pub columns: Vec<String>,
    pub where_clause: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DropIndexStatement {
    pub index_names: Vec<String>,
    pub if_exists: bool,
    pub cascade: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DdlStatement {
    CreateTable(CreateTableStatement),
    AlterTable(AlterTableStatement),
    DropTable(DropTableStatement),
    CreateIndex(CreateIndexStatement),
    DropIndex(DropIndexStatement),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SchemaDefinition {
    pub name: Option<String>,
    pub statements: Vec<DdlStatement>,
}

impl DataType {
    pub fn is_numeric(&self) -> bool {
        matches!(
            self,
            DataType::Integer { .. }
                | DataType::BigInt { .. }
                | DataType::SmallInt { .. }
                | DataType::TinyInt { .. }
                | DataType::Float { .. }
                | DataType::Double
                | DataType::Decimal { .. }
        )
    }

    pub fn is_text(&self) -> bool {
        matches!(
            self,
            DataType::Varchar { .. } | DataType::Char { .. } | DataType::Text
        )
    }

    pub fn is_temporal(&self) -> bool {
        matches!(
            self,
            DataType::Date | DataType::Time | DataType::DateTime | DataType::Timestamp
        )
    }
}

impl ColumnDefinition {
    pub fn new(name: String, data_type: DataType) -> Self {
        Self {
            name,
            data_type,
            constraints: Vec::new(),
        }
    }

    pub fn with_constraints(mut self, constraints: Vec<ColumnConstraint>) -> Self {
        self.constraints = constraints;
        self
    }

    pub fn is_primary_key(&self) -> bool {
        self.constraints
            .iter()
            .any(|c| matches!(c, ColumnConstraint::PrimaryKey))
    }

    pub fn is_nullable(&self) -> bool {
        !self.constraints.iter().any(|c| matches!(c, ColumnConstraint::NotNull))
    }

    pub fn has_default(&self) -> bool {
        self.constraints
            .iter()
            .any(|c| matches!(c, ColumnConstraint::Default(_)))
    }
}

impl CreateTableStatement {
    pub fn new(table_name: String) -> Self {
        Self {
            table_name,
            if_not_exists: false,
            columns: Vec::new(),
            constraints: Vec::new(),
            options: HashMap::new(),
        }
    }

    pub fn with_columns(mut self, columns: Vec<ColumnDefinition>) -> Self {
        self.columns = columns;
        self
    }

    pub fn with_constraints(mut self, constraints: Vec<TableConstraint>) -> Self {
        self.constraints = constraints;
        self
    }

    pub fn primary_key_columns(&self) -> Vec<&ColumnDefinition> {
        self.columns
            .iter()
            .filter(|col| col.is_primary_key())
            .collect()
    }

    pub fn foreign_keys(&self) -> Vec<&TableConstraint> {
        self.constraints
            .iter()
            .filter(|c| matches!(c, TableConstraint::ForeignKey { .. }))
            .collect()
    }
}

impl AlterTableStatement {
    pub fn new(table_name: String) -> Self {
        Self {
            table_name,
            if_exists: false,
            actions: Vec::new(),
        }
    }

    pub fn with_actions(mut self, actions: Vec<AlterTableAction>) -> Self {
        self.actions = actions;
        self
    }
}

impl CreateIndexStatement {
    pub fn new(index_name: String, table_name: String) -> Self {
        Self {
            index_name,
            table_name,
            if_not_exists: false,
            unique: false,
            index_type: None,
            columns: Vec::new(),
            where_clause: None,
        }
    }

    pub fn with_columns(mut self, columns: Vec<String>) -> Self {
        self.columns = columns;
        self
    }
}

impl Default for SchemaDefinition {
    fn default() -> Self {
        Self::new()
    }
}

impl SchemaDefinition {
    pub fn new() -> Self {
        Self {
            name: None,
            statements: Vec::new(),
        }
    }

    pub fn with_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn add_statement(&mut self, statement: DdlStatement) {
        self.statements.push(statement);
    }

    pub fn tables(&self) -> Vec<&CreateTableStatement> {
        self.statements
            .iter()
            .filter_map(|stmt| match stmt {
                DdlStatement::CreateTable(table) => Some(table),
                _ => None,
            })
            .collect()
    }

    pub fn indexes(&self) -> Vec<&CreateIndexStatement> {
        self.statements
            .iter()
            .filter_map(|stmt| match stmt {
                DdlStatement::CreateIndex(index) => Some(index),
                _ => None,
            })
            .collect()
    }
}
