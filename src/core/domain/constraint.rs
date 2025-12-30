use std::collections::{HashMap, HashSet};
use serde::{Deserialize, Serialize};

use crate::core::domain::entity::*;
use crate::core::domain::relationships::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConstraintAnalysis {
    pub constraint_type: ConstraintType,
    pub table_name: String,
    pub constraint_name: Option<String>,
    pub columns: Vec<String>,
    pub is_valid: bool,
    pub issues: Vec<ConstraintIssue>,
    pub business_rules: Vec<BusinessRule>,
    pub dependencies: Vec<String>,
    pub metadata: ConstraintMetadata,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConstraintType {
    PrimaryKey,
    ForeignKey,
    Unique,
    Check,
    Default,
    NotNull,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConstraintIssue {
    pub issue_type: ConstraintIssueType,
    pub severity: IssueSeverity,
    pub description: String,
    pub suggested_fix: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConstraintIssueType {
    MissingConstraint,
    RedundantConstraint,
    ConflictingConstraint,
    PerformanceImpact,
    DataIntegrityRisk,
    NamingConvention,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum IssueSeverity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BusinessRule {
    pub rule_type: BusinessRuleType,
    pub description: String,
    pub examples: Vec<String>,
    pub enforcement_level: EnforcementLevel,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BusinessRuleType {
    Uniqueness,
    ReferentialIntegrity,
    DomainValidation,
    RequiredField,
    DefaultValue,
    CustomLogic,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EnforcementLevel {
    Database,
    Application,
    Manual,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConstraintMetadata {
    pub description: Option<String>,
    pub created_by: Option<String>,
    pub created_at: Option<String>,
    pub business_justification: Option<String>,
    pub tags: Vec<String>,
    pub custom_properties: HashMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConstraintRelationship {
    pub source_constraint: String,
    pub target_constraint: String,
    pub relationship_type: ConstraintRelationshipType,
    pub strength: RelationshipStrength,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConstraintRelationshipType {
    DependsOn,
    ConflictsWith,
    Complements,
    Overlaps,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RelationshipStrength {
    Weak,
    Moderate,
    Strong,
}

pub struct ConstraintValidationService;

impl ConstraintValidationService {
    pub fn validate_primary_key(pk: &PrimaryKey, table: &Table, database: &Database) -> ConstraintAnalysis {
        let mut issues = Vec::new();
        let mut business_rules = Vec::new();
        let mut dependencies = Vec::new();

        // Check if all columns exist
        for col_name in &pk.columns {
            if !table.has_column(col_name) {
                issues.push(ConstraintIssue {
                    issue_type: ConstraintIssueType::DataIntegrityRisk,
                    severity: IssueSeverity::Critical,
                    description: format!("Primary key column '{}' does not exist in table '{}'", col_name, table.name),
                    suggested_fix: Some(format!("Add column '{}' to table '{}' or remove it from primary key", col_name, table.name)),
                });
            }
        }

        // Check for nullability issues
        for col_name in &pk.columns {
            if let Some(column) = table.get_column(col_name) {
                if column.nullable {
                    issues.push(ConstraintIssue {
                        issue_type: ConstraintIssueType::DataIntegrityRisk,
                        severity: IssueSeverity::Critical,
                        description: format!("Primary key column '{}' cannot be nullable", col_name),
                        suggested_fix: Some(format!("ALTER TABLE {} MODIFY {} {} NOT NULL", table.name, col_name, column.get_data_type_name())),
                    });
                }
            }
        }

        // Check for uniqueness (primary keys are inherently unique)
        business_rules.push(BusinessRule {
            rule_type: BusinessRuleType::Uniqueness,
            description: format!("Each combination of ({}) must be unique across all rows", pk.columns.join(", ")),
            examples: vec![
                format!("Cannot have duplicate values in ({}) columns", pk.columns.join(", ")),
                "Primary key ensures entity identity uniqueness".to_string(),
            ],
            enforcement_level: EnforcementLevel::Database,
        });

        // Check for required fields
        business_rules.push(BusinessRule {
            rule_type: BusinessRuleType::RequiredField,
            description: format!("Columns ({}) are mandatory for all records", pk.columns.join(", ")),
            examples: vec![
                "Cannot insert records with NULL values in primary key columns".to_string(),
                "Primary key columns must have values during updates".to_string(),
            ],
            enforcement_level: EnforcementLevel::Database,
        });

        // Check naming convention
        if let Some(name) = &pk.constraint_name {
            if !name.to_lowercase().contains("pk") && !name.to_lowercase().contains("primary") {
                issues.push(ConstraintIssue {
                    issue_type: ConstraintIssueType::NamingConvention,
                    severity: IssueSeverity::Low,
                    description: format!("Primary key constraint name '{}' should follow naming convention (include 'PK' or 'PRIMARY')", name),
                    suggested_fix: Some(format!("Rename constraint to 'PK_{}_{}'", table.name, pk.columns.join("_"))),
                });
            }
        }

        ConstraintAnalysis {
            constraint_type: ConstraintType::PrimaryKey,
            table_name: table.name.clone(),
            constraint_name: pk.constraint_name.clone(),
            columns: pk.columns.clone(),
            is_valid: issues.iter().all(|issue| issue.severity != IssueSeverity::Critical),
            issues,
            business_rules,
            dependencies,
            metadata: ConstraintMetadata {
                description: Some(format!("Primary key constraint on columns ({})", pk.columns.join(", "))),
                created_by: None,
                created_at: None,
                business_justification: Some("Ensures entity identity and data integrity".to_string()),
                tags: vec!["primary".to_string(), "unique".to_string(), "required".to_string()],
                custom_properties: HashMap::new(),
            },
        }
    }

    pub fn validate_foreign_key(fk: &ForeignKey, table: &Table, database: &Database) -> ConstraintAnalysis {
        let mut issues = Vec::new();
        let mut business_rules = Vec::new();
        let mut dependencies = Vec::new();

        // Check if referenced table exists
        if database.get_table(None, &fk.referenced_table).is_none() {
            issues.push(ConstraintIssue {
                issue_type: ConstraintIssueType::DataIntegrityRisk,
                severity: IssueSeverity::Critical,
                description: format!("Referenced table '{}' does not exist", fk.referenced_table),
                suggested_fix: Some(format!("Create table '{}' or correct the foreign key reference", fk.referenced_table)),
            });
        }

        // Check if referenced columns exist
        if let Some(referenced_table) = database.get_table(None, &fk.referenced_table) {
            for col_name in &fk.referenced_columns {
                if !referenced_table.has_column(col_name) {
                    issues.push(ConstraintIssue {
                        issue_type: ConstraintIssueType::DataIntegrityRisk,
                        severity: IssueSeverity::Critical,
                        description: format!("Referenced column '{}' does not exist in table '{}'", col_name, fk.referenced_table),
                        suggested_fix: Some(format!("Add column '{}' to table '{}' or correct the foreign key reference", col_name, fk.referenced_table)),
                    });
                }
            }

            // Check if referenced columns have appropriate constraints
            for col_name in &fk.referenced_columns {
                if let Some(column) = referenced_table.get_column(col_name) {
                    // Referenced columns should typically be primary key or unique
                    let is_pk = referenced_table.primary_key.as_ref()
                        .map(|pk| pk.columns.contains(col_name))
                        .unwrap_or(false);

                    let is_unique = referenced_table.unique_constraints.iter()
                        .any(|uc| uc.columns.contains(col_name));

                    if !is_pk && !is_unique {
                        issues.push(ConstraintIssue {
                            issue_type: ConstraintIssueType::DataIntegrityRisk,
                            severity: IssueSeverity::High,
                            description: format!("Referenced column '{}' should be unique or part of primary key", col_name),
                            suggested_fix: Some(format!("Add unique constraint or include '{}' in primary key of table '{}'", col_name, fk.referenced_table)),
                        });
                    }
                }
            }

            // Add dependency
            dependencies.push(format!("Depends on {}.{}", fk.referenced_table, fk.referenced_columns.join(", ")));
        }

        // Check foreign key columns exist
        for col_name in &fk.columns {
            if !table.has_column(col_name) {
                issues.push(ConstraintIssue {
                    issue_type: ConstraintIssueType::DataIntegrityRisk,
                    severity: IssueSeverity::Critical,
                    description: format!("Foreign key column '{}' does not exist in table '{}'", col_name, table.name),
                    suggested_fix: Some(format!("Add column '{}' to table '{}' or remove it from foreign key", col_name, table.name)),
                });
            }
        }

        // Check data type compatibility
        if let (Some(source_table), Some(target_table)) = (Some(table), database.get_table(None, &fk.referenced_table)) {
            for (source_col, target_col) in fk.columns.iter().zip(fk.referenced_columns.iter()) {
                if let (Some(source_column), Some(target_column)) = (
                    source_table.get_column(source_col),
                    target_table.get_column(target_col)
                ) {
                    if !Self::are_data_types_compatible(&source_column.data_type, &target_column.data_type) {
                        issues.push(ConstraintIssue {
                            issue_type: ConstraintIssueType::DataIntegrityRisk,
                            severity: IssueSeverity::High,
                            description: format!("Data type mismatch: '{}' ({}) vs '{}' ({})",
                                source_col, source_column.get_data_type_name(),
                                target_col, target_column.get_data_type_name()),
                            suggested_fix: Some(format!("Change data type of '{}' to match '{}'", source_col, target_column.get_data_type_name())),
                        });
                    }
                }
            }
        }

        // Business rules for referential integrity
        business_rules.push(BusinessRule {
            rule_type: BusinessRuleType::ReferentialIntegrity,
            description: format!("Values in ({}) must exist in ({}) of table '{}'",
                fk.columns.join(", "), fk.referenced_columns.join(", "), fk.referenced_table),
            examples: vec![
                format!("Cannot insert into '{}' with values in ({}) that don't exist in '{}'.({}, {})",
                    table.name, fk.columns.join(", "), fk.referenced_table, fk.referenced_table, fk.referenced_columns.join(", ")),
                "Foreign key maintains referential integrity between tables".to_string(),
            ],
            enforcement_level: EnforcementLevel::Database,
        });

        // Check cascade actions
        match fk.on_delete {
            ReferentialAction::NoAction => {
                issues.push(ConstraintIssue {
                    issue_type: ConstraintIssueType::DataIntegrityRisk,
                    severity: IssueSeverity::Medium,
                    description: "Foreign key uses NO ACTION on delete - may cause constraint violations".to_string(),
                    suggested_fix: Some("Consider using CASCADE or SET NULL for better referential integrity".to_string()),
                });
            }
            ReferentialAction::Cascade => {
                business_rules.push(BusinessRule {
                    rule_type: BusinessRuleType::CustomLogic,
                    description: "Child records are automatically deleted when parent is deleted".to_string(),
                    examples: vec![
                        format!("Deleting from '{}' will automatically delete related records from '{}'", fk.referenced_table, table.name),
                    ],
                    enforcement_level: EnforcementLevel::Database,
                });
            }
            ReferentialAction::SetNull => {
                business_rules.push(BusinessRule {
                    rule_type: BusinessRuleType::CustomLogic,
                    description: "Foreign key values are set to NULL when referenced record is deleted".to_string(),
                    examples: vec![
                        format!("Deleting from '{}' will set ({}) to NULL in related '{}'", fk.referenced_table, fk.columns.join(", "), table.name),
                    ],
                    enforcement_level: EnforcementLevel::Database,
                });
            }
            ReferentialAction::SetDefault => {
                business_rules.push(BusinessRule {
                    rule_type: BusinessRuleType::DefaultValue,
                    description: "Foreign key values are set to default when referenced record is deleted".to_string(),
                    examples: vec![
                        format!("Deleting from '{}' will set ({}) to default values in related '{}'", fk.referenced_table, fk.columns.join(", "), table.name),
                    ],
                    enforcement_level: EnforcementLevel::Database,
                });
            }
            ReferentialAction::Restrict => {
                business_rules.push(BusinessRule {
                    rule_type: BusinessRuleType::CustomLogic,
                    description: "Cannot delete referenced records that have dependent records".to_string(),
                    examples: vec![
                        format!("Cannot delete from '{}' if related records exist in '{}'", fk.referenced_table, table.name),
                    ],
                    enforcement_level: EnforcementLevel::Database,
                });
            }
        }

        ConstraintAnalysis {
            constraint_type: ConstraintType::ForeignKey,
            table_name: table.name.clone(),
            constraint_name: fk.constraint_name.clone(),
            columns: fk.columns.clone(),
            is_valid: issues.iter().all(|issue| issue.severity != IssueSeverity::Critical),
            issues,
            business_rules,
            dependencies,
            metadata: ConstraintMetadata {
                description: Some(format!("Foreign key referencing {}.{}", fk.referenced_table, fk.referenced_columns.join(", "))),
                created_by: None,
                created_at: None,
                business_justification: Some("Maintains referential integrity between related tables".to_string()),
                tags: vec!["foreign".to_string(), "reference".to_string(), "relationship".to_string()],
                custom_properties: HashMap::new(),
            },
        }
    }

    pub fn validate_unique_constraint(uc: &UniqueConstraint, table: &Table, _database: &Database) -> ConstraintAnalysis {
        let mut issues = Vec::new();
        let mut business_rules = Vec::new();
        let dependencies = Vec::new();

        // Check if all columns exist
        for col_name in &uc.columns {
            if !table.has_column(col_name) {
                issues.push(ConstraintIssue {
                    issue_type: ConstraintIssueType::DataIntegrityRisk,
                    severity: IssueSeverity::Critical,
                    description: format!("Unique constraint column '{}' does not exist in table '{}'", col_name, table.name),
                    suggested_fix: Some(format!("Add column '{}' to table '{}' or remove it from unique constraint", col_name, table.name)),
                });
            }
        }

        // Business rule for uniqueness
        business_rules.push(BusinessRule {
            rule_type: BusinessRuleType::Uniqueness,
            description: format!("Each combination of ({}) must be unique across all rows (except NULL values)", uc.columns.join(", ")),
            examples: vec![
                format!("Cannot have duplicate non-NULL values in ({}) columns", uc.columns.join(", ")),
                "NULL values are allowed and don't violate uniqueness".to_string(),
            ],
            enforcement_level: EnforcementLevel::Database,
        });

        // Check naming convention
        if let Some(name) = &uc.constraint_name {
            if !name.to_lowercase().contains("uq") && !name.to_lowercase().contains("unique") {
                issues.push(ConstraintIssue {
                    issue_type: ConstraintIssueType::NamingConvention,
                    severity: IssueSeverity::Low,
                    description: format!("Unique constraint name '{}' should follow naming convention (include 'UQ' or 'UNIQUE')", name),
                    suggested_fix: Some(format!("Rename constraint to 'UQ_{}_{}'", table.name, uc.columns.join("_"))),
                });
            }
        }

        ConstraintAnalysis {
            constraint_type: ConstraintType::Unique,
            table_name: table.name.clone(),
            constraint_name: uc.constraint_name.clone(),
            columns: uc.columns.clone(),
            is_valid: issues.iter().all(|issue| issue.severity != IssueSeverity::Critical),
            issues,
            business_rules,
            dependencies,
            metadata: ConstraintMetadata {
                description: Some(format!("Unique constraint on columns ({})", uc.columns.join(", "))),
                created_by: None,
                created_at: None,
                business_justification: Some("Ensures no duplicate combinations of specified columns".to_string()),
                tags: vec!["unique".to_string(), "constraint".to_string()],
                custom_properties: HashMap::new(),
            },
        }
    }

    pub fn validate_check_constraint(cc: &CheckConstraint, table: &Table, _database: &Database) -> ConstraintAnalysis {
        let mut issues = Vec::new();
        let mut business_rules = Vec::new();
        let dependencies = Vec::new();

        // Parse the expression to extract column references
        let referenced_columns = Self::extract_columns_from_expression(&cc.expression);

        // Check if referenced columns exist
        for col_name in &referenced_columns {
            if !table.has_column(col_name) {
                issues.push(ConstraintIssue {
                    issue_type: ConstraintIssueType::DataIntegrityRisk,
                    severity: IssueSeverity::Critical,
                    description: format!("Check constraint references non-existent column '{}'", col_name),
                    suggested_fix: Some(format!("Add column '{}' to table '{}' or correct the check constraint expression", col_name, table.name)),
                });
            }
        }

        // Extract business rules from check expression
        if let Some(rule) = Self::extract_business_rule_from_check(&cc.expression) {
            business_rules.push(rule);
        }

        // Business rule for check constraint
        business_rules.push(BusinessRule {
            rule_type: BusinessRuleType::DomainValidation,
            description: format!("Records must satisfy the condition: {}", cc.expression),
            examples: vec![
                format!("Cannot insert/update records where {} is not satisfied", cc.expression),
                "Check constraint validates data against business rules".to_string(),
            ],
            enforcement_level: EnforcementLevel::Database,
        });

        // Check naming convention
        if let Some(name) = &cc.constraint_name {
            if !name.to_lowercase().contains("ck") && !name.to_lowercase().contains("check") {
                issues.push(ConstraintIssue {
                    issue_type: ConstraintIssueType::NamingConvention,
                    severity: IssueSeverity::Low,
                    description: format!("Check constraint name '{}' should follow naming convention (include 'CK' or 'CHECK')", name),
                    suggested_fix: Some(format!("Rename constraint to 'CK_{}_{}'", table.name, referenced_columns.join("_"))),
                });
            }
        }

        ConstraintAnalysis {
            constraint_type: ConstraintType::Check,
            table_name: table.name.clone(),
            constraint_name: cc.constraint_name.clone(),
            columns: referenced_columns,
            is_valid: issues.iter().all(|issue| issue.severity != IssueSeverity::Critical),
            issues,
            business_rules,
            dependencies,
            metadata: ConstraintMetadata {
                description: Some(format!("Check constraint: {}", cc.expression)),
                created_by: None,
                created_at: None,
                business_justification: Some("Validates data against business rules and domain constraints".to_string()),
                tags: vec!["check".to_string(), "validation".to_string(), "business_rule".to_string()],
                custom_properties: HashMap::new(),
            },
        }
    }

    fn are_data_types_compatible(source: &DataType, target: &DataType) -> bool {
        match (source, target) {
            // Exact matches
            (s, t) if s == t => true,

            // Numeric compatibility
            (DataType::TinyInt { .. }, DataType::SmallInt { .. }) => true,
            (DataType::SmallInt { .. }, DataType::Integer { .. }) => true,
            (DataType::Integer { .. }, DataType::BigInt { .. }) => true,

            // String compatibility (same length or target is longer/unlimited)
            (DataType::Char { length: s_len }, DataType::VarChar { length: t_len }) => {
                match (s_len, t_len) {
                    (Some(s), Some(t)) => s <= t,
                    (Some(_), None) => true,
                    (None, Some(_)) => false,
                    (None, None) => true,
                }
            }
            (DataType::VarChar { length: s_len }, DataType::Text) => true,
            (DataType::Char { .. }, DataType::Text) => true,

            // Default: not compatible
            _ => false,
        }
    }

    fn extract_columns_from_expression(expression: &str) -> Vec<String> {
        // Simple regex-based column extraction (in a real implementation, this would use proper SQL parsing)
        use regex::Regex;
        let column_regex = Regex::new(r"\b([a-zA-Z_][a-zA-Z0-9_]*)\b").unwrap();

        column_regex.captures_iter(expression)
            .filter_map(|cap| cap.get(1))
            .map(|m| m.as_str().to_string())
            .collect::<HashSet<String>>()
            .into_iter()
            .collect()
    }

    fn extract_business_rule_from_check(expression: &str) -> Option<BusinessRule> {
        // Simple pattern matching for common check constraint patterns
        if expression.contains(">=") || expression.contains("<=") || expression.contains(">") || expression.contains("<") {
            Some(BusinessRule {
                rule_type: BusinessRuleType::DomainValidation,
                description: format!("Value range validation: {}", expression),
                examples: vec![
                    format!("Values must satisfy: {}", expression),
                    "Range constraints ensure data falls within acceptable bounds".to_string(),
                ],
                enforcement_level: EnforcementLevel::Database,
            })
        } else if expression.to_lowercase().contains("in") {
            Some(BusinessRule {
                rule_type: BusinessRuleType::DomainValidation,
                description: format!("Value must be from allowed set: {}", expression),
                examples: vec![
                    format!("Values must be one of: {}", expression),
                    "Enumerated value constraints ensure data integrity".to_string(),
                ],
                enforcement_level: EnforcementLevel::Database,
            })
        } else if expression.to_lowercase().contains("like") {
            Some(BusinessRule {
                rule_type: BusinessRuleType::DomainValidation,
                description: format!("Value must match pattern: {}", expression),
                examples: vec![
                    format!("Values must match pattern: {}", expression),
                    "Pattern constraints ensure data format compliance".to_string(),
                ],
                enforcement_level: EnforcementLevel::Database,
            })
        } else {
            None
        }
    }
}

pub struct ConstraintAnalysisService;

impl ConstraintAnalysisService {
    pub fn analyze_table_constraints(table: &Table, database: &Database) -> Vec<ConstraintAnalysis> {
        let mut analyses = Vec::new();

        // Primary key
        if let Some(pk) = &table.primary_key {
            analyses.push(ConstraintValidationService::validate_primary_key(pk, table, database));
        }

        // Foreign keys
        for fk in &table.foreign_keys {
            analyses.push(ConstraintValidationService::validate_foreign_key(fk, table, database));
        }

        // Unique constraints
        for uc in &table.unique_constraints {
            analyses.push(ConstraintValidationService::validate_unique_constraint(uc, table, database));
        }

        // Check constraints
        for cc in &table.check_constraints {
            analyses.push(ConstraintValidationService::validate_check_constraint(cc, table, database));
        }

        // Analyze NOT NULL constraints
        for column in &table.columns {
            if !column.nullable {
                analyses.push(ConstraintAnalysis {
                    constraint_type: ConstraintType::NotNull,
                    table_name: table.name.clone(),
                    constraint_name: None,
                    columns: vec![column.name.clone()],
                    is_valid: true,
                    issues: Vec::new(),
                    business_rules: vec![BusinessRule {
                        rule_type: BusinessRuleType::RequiredField,
                        description: format!("Column '{}' cannot be NULL", column.name),
                        examples: vec![
                            format!("Cannot insert records with NULL values in '{}'", column.name),
                            format!("Cannot update '{}' to NULL", column.name),
                        ],
                        enforcement_level: EnforcementLevel::Database,
                    }],
                    dependencies: Vec::new(),
                    metadata: ConstraintMetadata {
                        description: Some(format!("NOT NULL constraint on column '{}'", column.name)),
                        created_by: None,
                        created_at: None,
                        business_justification: Some("Ensures required data is always provided".to_string()),
                        tags: vec!["required".to_string(), "not_null".to_string()],
                        custom_properties: HashMap::new(),
                    },
                });
            }
        }

        analyses
    }

    pub fn analyze_constraint_relationships(database: &Database) -> Vec<ConstraintRelationship> {
        let mut relationships = Vec::new();
        let tables = database.get_all_tables();

        for table in &tables {
            // Primary key -> Foreign key relationships
            if let Some(pk) = &table.primary_key {
                for other_table in &tables {
                    for fk in &other_table.foreign_keys {
                        if fk.referenced_table == table.name &&
                           fk.referenced_columns == pk.columns {
                            relationships.push(ConstraintRelationship {
                                source_constraint: format!("{}.{}", table.name, pk.constraint_name.as_deref().unwrap_or("PRIMARY KEY")),
                                target_constraint: format!("{}.{}", other_table.name, fk.constraint_name.as_deref().unwrap_or("FOREIGN KEY")),
                                relationship_type: ConstraintRelationshipType::Complements,
                                strength: RelationshipStrength::Strong,
                            });
                        }
                    }
                }
            }

            // Foreign key -> Unique constraint relationships
            for fk in &table.foreign_keys {
                if let Some(referenced_table) = database.get_table(None, &fk.referenced_table) {
                    for uc in &referenced_table.unique_constraints {
                        if uc.columns == fk.referenced_columns {
                            relationships.push(ConstraintRelationship {
                                source_constraint: format!("{}.{}", table.name, fk.constraint_name.as_deref().unwrap_or("FOREIGN KEY")),
                                target_constraint: format!("{}.{}", referenced_table.name, uc.constraint_name.as_deref().unwrap_or("UNIQUE")),
                                relationship_type: ConstraintRelationshipType::DependsOn,
                                strength: RelationshipStrength::Moderate,
                            });
                        }
                    }
                }
            }
        }

        relationships
    }

    pub fn detect_constraint_issues(database: &Database) -> Vec<ConstraintIssue> {
        let mut all_issues = Vec::new();

        for table in database.get_all_tables() {
            // Check for tables without primary keys
            if table.primary_key.is_none() {
                all_issues.push(ConstraintIssue {
                    issue_type: ConstraintIssueType::MissingConstraint,
                    severity: IssueSeverity::High,
                    description: format!("Table '{}' has no primary key", table.name),
                    suggested_fix: Some(format!("Add a primary key to table '{}' (consider using an auto-increment ID column)", table.name)),
                });
            }

            // Check for foreign keys without indexes
            for fk in &table.foreign_keys {
                let has_index = table.indexes.iter().any(|idx| {
                    idx.columns == fk.columns
                });

                if !has_index {
                    all_issues.push(ConstraintIssue {
                        issue_type: ConstraintIssueType::PerformanceImpact,
                        severity: IssueSeverity::Medium,
                        description: format!("Foreign key ({}) in table '{}' has no supporting index", fk.columns.join(", "), table.name),
                        suggested_fix: Some(format!("CREATE INDEX idx_{}_{} ON {} ({});",
                            table.name, fk.columns.join("_"), table.name, fk.columns.join(", "))),
                    });
                }
            }

            // Check for redundant unique constraints
            let unique_columns: HashSet<_> = table.unique_constraints.iter()
                .flat_map(|uc| &uc.columns)
                .collect();

            if let Some(pk) = &table.primary_key {
                let redundant_columns: Vec<_> = pk.columns.iter()
                    .filter(|col| unique_columns.contains(col))
                    .cloned()
                    .collect();

                if !redundant_columns.is_empty() {
                    all_issues.push(ConstraintIssue {
                        issue_type: ConstraintIssueType::RedundantConstraint,
                        severity: IssueSeverity::Low,
                        description: format!("Columns ({}) are both in primary key and unique constraints", redundant_columns.join(", ")),
                        suggested_fix: Some(format!("Remove redundant unique constraints on primary key columns in table '{}'", table.name)),
                    });
                }
            }
        }

        all_issues
    }
}

pub struct ConstraintDocumentationService;

impl ConstraintDocumentationService {
    pub fn generate_constraint_documentation(analysis: &ConstraintAnalysis) -> String {
        let mut doc = format!("### {} Constraint\n\n", Self::constraint_type_name(&analysis.constraint_type));

        if let Some(name) = &analysis.constraint_name {
            doc.push_str(&format!("**Name:** {}\n", name));
        }

        doc.push_str(&format!("**Table:** {}\n", analysis.table_name));
        doc.push_str(&format!("**Columns:** {}\n", analysis.columns.join(", ")));

        if let Some(description) = &analysis.metadata.description {
            doc.push_str(&format!("**Description:** {}\n", description));
        }

        if !analysis.business_rules.is_empty() {
            doc.push_str("\n#### Business Rules\n\n");
            for rule in &analysis.business_rules {
                doc.push_str(&format!("- **{}:** {}\n", Self::rule_type_name(&rule.rule_type), rule.description));

                if !rule.examples.is_empty() {
                    doc.push_str("  - Examples:\n");
                    for example in &rule.examples {
                        doc.push_str(&format!("    - {}\n", example));
                    }
                }
            }
        }

        if !analysis.issues.is_empty() {
            doc.push_str("\n#### Issues\n\n");
            for issue in &analysis.issues {
                let severity_icon = match issue.severity {
                    IssueSeverity::Critical => "🚨",
                    IssueSeverity::High => "⚠️",
                    IssueSeverity::Medium => "⚡",
                    IssueSeverity::Low => "ℹ️",
                };

                doc.push_str(&format!("- {} **{}:** {}\n", severity_icon, Self::issue_type_name(&issue.issue_type), issue.description));

                if let Some(fix) = &issue.suggested_fix {
                    doc.push_str(&format!("  - **Suggested Fix:** {}\n", fix));
                }
            }
        }

        doc.push_str("\n---\n\n");
        doc
    }

    pub fn generate_api_validation_rules(analyses: &[ConstraintAnalysis]) -> HashMap<String, Vec<String>> {
        let mut validation_rules = HashMap::new();

        for analysis in analyses {
            let table_rules = validation_rules.entry(analysis.table_name.clone()).or_insert(Vec::new());

            match &analysis.constraint_type {
                ConstraintType::PrimaryKey => {
                    for column in &analysis.columns {
                        table_rules.push(format!("{}: required, unique", column));
                    }
                }
                ConstraintType::ForeignKey => {
                    for column in &analysis.columns {
                        table_rules.push(format!("{}: references {}.{}", column, analysis.table_name, analysis.columns.join("_")));
                    }
                }
                ConstraintType::Unique => {
                    table_rules.push(format!("{}: unique combination", analysis.columns.join(", ")));
                }
                ConstraintType::Check => {
                    table_rules.push(format!("Check constraint: {}", analysis.metadata.description.as_deref().unwrap_or("custom validation")));
                }
                ConstraintType::NotNull => {
                    for column in &analysis.columns {
                        table_rules.push(format!("{}: required", column));
                    }
                }
                _ => {}
            }
        }

        validation_rules
    }

    fn constraint_type_name(constraint_type: &ConstraintType) -> &'static str {
        match constraint_type {
            ConstraintType::PrimaryKey => "Primary Key",
            ConstraintType::ForeignKey => "Foreign Key",
            ConstraintType::Unique => "Unique",
            ConstraintType::Check => "Check",
            ConstraintType::Default => "Default",
            ConstraintType::NotNull => "Not Null",
        }
    }

    fn rule_type_name(rule_type: &BusinessRuleType) -> &'static str {
        match rule_type {
            BusinessRuleType::Uniqueness => "Uniqueness",
            BusinessRuleType::ReferentialIntegrity => "Referential Integrity",
            BusinessRuleType::DomainValidation => "Domain Validation",
            BusinessRuleType::RequiredField => "Required Field",
            BusinessRuleType::DefaultValue => "Default Value",
            BusinessRuleType::CustomLogic => "Custom Logic",
        }
    }

    fn issue_type_name(issue_type: &ConstraintIssueType) -> &'static str {
        match issue_type {
            ConstraintIssueType::MissingConstraint => "Missing Constraint",
            ConstraintIssueType::RedundantConstraint => "Redundant Constraint",
            ConstraintIssueType::ConflictingConstraint => "Conflicting Constraint",
            ConstraintIssueType::PerformanceImpact => "Performance Impact",
            ConstraintIssueType::DataIntegrityRisk => "Data Integrity Risk",
            ConstraintIssueType::NamingConvention => "Naming Convention",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary_key_validation() {
        let mut table = Table::new("users");
        table.add_column(Column::new("id", DataType::Integer { unsigned: false }));
        table.add_column(Column::new("name", DataType::VarChar { length: Some(100) }));

        let pk = PrimaryKey {
            name: None,
            columns: vec!["id".to_string()],
            constraint_name: Some("PK_users".to_string()),
        };

        table.primary_key = Some(pk);

        let database = Database::new("test_db");
        let analysis = ConstraintValidationService::validate_primary_key(table.primary_key.as_ref().unwrap(), &table, &database);

        assert!(analysis.is_valid);
        assert_eq!(analysis.constraint_type, ConstraintType::PrimaryKey);
        assert_eq!(analysis.columns, vec!["id"]);
        assert!(!analysis.business_rules.is_empty());
    }

    #[test]
    fn test_foreign_key_validation() {
        let mut source_table = Table::new("posts");
        source_table.add_column(Column::new("id", DataType::Integer { unsigned: false }));
        source_table.add_column(Column::new("user_id", DataType::Integer { unsigned: false }));

        let mut target_table = Table::new("users");
        target_table.add_column(Column::new("id", DataType::Integer { unsigned: false }));

        let pk = PrimaryKey {
            name: None,
            columns: vec!["id".to_string()],
            constraint_name: Some("PK_users".to_string()),
        };
        target_table.primary_key = Some(pk);

        let fk = ForeignKey {
            name: Some("FK_posts_users".to_string()),
            table_name: "posts".to_string(),
            columns: vec!["user_id".to_string()],
            referenced_table: "users".to_string(),
            referenced_columns: vec!["id".to_string()],
            on_delete: ReferentialAction::Cascade,
            on_update: ReferentialAction::Restrict,
            constraint_name: None,
            metadata: ForeignKeyMetadata::default(),
        };

        source_table.add_foreign_key(fk);

        let mut database = Database::new("test_db");
        database.add_table(source_table);
        database.add_table(target_table);

        let analysis = ConstraintValidationService::validate_foreign_key(
            database.get_table(None, "posts").unwrap().foreign_keys.first().unwrap(),
            database.get_table(None, "posts").unwrap(),
            &database
        );

        assert!(analysis.is_valid);
        assert_eq!(analysis.constraint_type, ConstraintType::ForeignKey);
        assert_eq!(analysis.columns, vec!["user_id"]);
    }

    #[test]
    fn test_data_type_compatibility() {
        // Compatible types
        assert!(ConstraintValidationService::are_data_types_compatible(
            &DataType::Integer { unsigned: false },
            &DataType::BigInt { unsigned: false }
        ));

        assert!(ConstraintValidationService::are_data_types_compatible(
            &DataType::Char { length: Some(10) },
            &DataType::VarChar { length: Some(20) }
        ));

        // Incompatible types
        assert!(!ConstraintValidationService::are_data_types_compatible(
            &DataType::Boolean,
            &DataType::VarChar { length: None }
        ));
    }

    #[test]
    fn test_column_extraction_from_expression() {
        let expression = "age >= 18 AND status IN ('active', 'pending')";
        let columns = ConstraintValidationService::extract_columns_from_expression(expression);

        assert!(columns.contains(&"age".to_string()));
        assert!(columns.contains(&"status".to_string()));
        assert!(!columns.contains(&"active".to_string())); // Should not extract string literals
    }
}
