use std::collections::{HashMap, HashSet};
use serde::{Deserialize, Serialize};

use crate::core::domain::entity::*;
use crate::core::parse::SqlDialect;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FieldValidationResult {
    pub field_name: String,
    pub is_valid: bool,
    pub errors: Vec<FieldValidationError>,
    pub warnings: Vec<FieldValidationWarning>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FieldValidationError {
    pub rule: String,
    pub message: String,
    pub severity: ValidationSeverity,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FieldValidationWarning {
    pub rule: String,
    pub message: String,
    pub suggestion: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FieldCompatibilityResult {
    pub source_type: DataType,
    pub target_type: DataType,
    pub is_compatible: bool,
    pub conversion_required: bool,
    pub data_loss_possible: bool,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FieldDocumentation {
    pub field_name: String,
    pub data_type: String,
    pub description: Option<String>,
    pub nullable: bool,
    pub default_value: Option<String>,
    pub constraints: Vec<String>,
    pub relationships: Vec<String>,
    pub examples: Vec<String>,
    pub validation_rules: Vec<String>,
    pub api_info: Option<FieldApiInfo>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FieldApiInfo {
    pub json_name: String,
    pub required: bool,
    pub validation: Vec<String>,
    pub example: Option<String>,
}

pub struct FieldService {
    dialect: SqlDialect,
    type_mappings: HashMap<String, DataType>,
}

impl FieldService {
    pub fn new(dialect: SqlDialect) -> Self {
        let type_mappings = Self::build_type_mappings(&dialect);
        Self {
            dialect,
            type_mappings,
        }
    }

    pub fn validate_field(&self, field: &Column, table: &Table) -> FieldValidationResult {
        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        // Check field name validity
        if field.name.is_empty() {
            errors.push(FieldValidationError {
                rule: "field_name_not_empty".to_string(),
                message: "Field name cannot be empty".to_string(),
                severity: ValidationSeverity::Error,
            });
        }

        // Check for reserved keywords
        if self.is_reserved_keyword(&field.name) {
            warnings.push(FieldValidationWarning {
                rule: "field_name_reserved".to_string(),
                message: format!("Field name '{}' is a reserved keyword", field.name),
                suggestion: Some(format!("Consider renaming to '{}_field'", field.name)),
            });
        }

        // Check field name length
        if field.name.len() > 63 {
            warnings.push(FieldValidationWarning {
                rule: "field_name_length".to_string(),
                message: format!("Field name '{}' is very long ({} characters)", field.name, field.name.len()),
                suggestion: Some("Consider using a shorter, more descriptive name".to_string()),
            });
        }

        // Validate data type
        if let Err(type_error) = self.validate_data_type(&field.data_type) {
            errors.push(type_error);
        }

        // Check auto-increment compatibility
        if field.auto_increment && !field.data_type.is_numeric() {
            errors.push(FieldValidationError {
                rule: "auto_increment_numeric".to_string(),
                message: "AUTO_INCREMENT can only be used with numeric data types".to_string(),
                severity: ValidationSeverity::Error,
            });
        }

        // Check identity compatibility
        if field.identity.is_some() && !field.data_type.is_numeric() {
            errors.push(FieldValidationError {
                rule: "identity_numeric".to_string(),
                message: "IDENTITY can only be used with numeric data types".to_string(),
                severity: ValidationSeverity::Error,
            });
        }

        // Check default value compatibility
        if let Some(default_value) = &field.default_value {
            if let Err(default_error) = self.validate_default_value(&field.data_type, default_value) {
                errors.push(default_error);
            }
        }

        // Check primary key constraints
        if field.is_primary_key(&table.primary_key) && field.nullable {
            errors.push(FieldValidationError {
                rule: "primary_key_not_null".to_string(),
                message: "Primary key fields cannot be nullable".to_string(),
                severity: ValidationSeverity::Error,
            });
        }

        // Check foreign key constraints
        if field.is_foreign_key(&table.foreign_keys) && field.auto_increment {
            warnings.push(FieldValidationWarning {
                rule: "foreign_key_auto_increment".to_string(),
                message: "Foreign key fields with AUTO_INCREMENT are unusual".to_string(),
                suggestion: Some("Consider removing AUTO_INCREMENT from foreign key fields".to_string()),
            });
        }

        // Check field naming conventions
        if !self.is_valid_identifier(&field.name) {
            errors.push(FieldValidationError {
                rule: "field_name_format".to_string(),
                message: format!("Field name '{}' contains invalid characters", field.name),
                severity: ValidationSeverity::Error,
            });
        }

        FieldValidationResult {
            field_name: field.name.clone(),
            is_valid: errors.is_empty(),
            errors,
            warnings,
        }
    }

    pub fn check_field_compatibility(&self, source: &DataType, target: &DataType) -> FieldCompatibilityResult {
        let mut notes = Vec::new();
        let mut data_loss_possible = false;
        let mut conversion_required = false;

        let is_compatible = match (source, target) {
            // Same types are always compatible
            (s, t) if s == t => true,

            // Numeric type compatibility
            (DataType::TinyInt { unsigned: s_unsigned }, DataType::SmallInt { unsigned: t_unsigned }) => {
                conversion_required = true;
                !(*s_unsigned && !*t_unsigned) // Can't convert unsigned to signed if value might overflow
            }
            (DataType::SmallInt { unsigned: s_unsigned }, DataType::Integer { unsigned: t_unsigned }) => {
                conversion_required = true;
                !(*s_unsigned && !*t_unsigned)
            }
            (DataType::Integer { unsigned: s_unsigned }, DataType::BigInt { unsigned: t_unsigned }) => {
                conversion_required = true;
                !(*s_unsigned && !*t_unsigned)
            }

            // String type compatibility
            (DataType::Char { length: s_len }, DataType::VarChar { length: t_len }) => {
                conversion_required = true;
                if let (Some(s), Some(t)) = (s_len, t_len) {
                    if s > t {
                        data_loss_possible = true;
                        notes.push(format!("Character data may be truncated from {} to {} characters", s, t));
                    }
                }
                true
            }
            (DataType::VarChar { length: s_len }, DataType::Text) => {
                conversion_required = true;
                true
            }

            // Boolean compatibility
            (DataType::Boolean, DataType::TinyInt { unsigned: true }) => {
                conversion_required = true;
                notes.push("Boolean will be converted to 0/1".to_string());
                true
            }

            // Default: not compatible
            _ => {
                notes.push(format!("Incompatible types: {} cannot be converted to {}", source, target));
                false
            }
        };

        FieldCompatibilityResult {
            source_type: source.clone(),
            target_type: target.clone(),
            is_compatible,
            conversion_required,
            data_loss_possible,
            notes,
        }
    }

    pub fn generate_field_documentation(&self, field: &Column, table: &Table) -> FieldDocumentation {
        let mut constraints = Vec::new();
        let mut relationships = Vec::new();

        // Primary key constraint
        if field.is_primary_key(&table.primary_key) {
            constraints.push("PRIMARY KEY".to_string());
        }

        // Foreign key relationships
        for fk in &table.foreign_keys {
            if fk.columns.contains(&field.name) {
                relationships.push(format!("References {}.{}", fk.referenced_table, fk.referenced_columns.join(", ")));
                constraints.push(format!("FOREIGN KEY -> {}.{}", fk.referenced_table, fk.referenced_columns.join(", ")));
            }
        }

        // Unique constraints
        for uc in &table.unique_constraints {
            if uc.columns.contains(&field.name) {
                constraints.push("UNIQUE".to_string());
            }
        }

        // Nullability
        if !field.nullable {
            constraints.push("NOT NULL".to_string());
        }

        // Auto increment
        if field.auto_increment {
            constraints.push("AUTO_INCREMENT".to_string());
        }

        // Identity
        if field.identity.is_some() {
            constraints.push("IDENTITY".to_string());
        }

        // Validation rules
        let validation_rules = field.metadata.validation_rules.iter()
            .map(|rule| format!("{}: {}", rule.rule_type, rule.message.as_deref().unwrap_or("")))
            .collect();

        // API info
        let api_info = field.metadata.api_field.as_ref().map(|api| {
            let validation = api.validation.iter()
                .map(|v| format!("Validation: {}", v))
                .collect();

            FieldApiInfo {
                json_name: api.name.clone(),
                required: api.required,
                validation,
                example: api.example.clone(),
            }
        });

        FieldDocumentation {
            field_name: field.name.clone(),
            data_type: field.get_data_type_name(),
            description: field.metadata.description.clone(),
            nullable: field.nullable,
            default_value: field.default_value.clone(),
            constraints,
            relationships,
            examples: field.metadata.example_values.clone(),
            validation_rules,
            api_info,
        }
    }

    pub fn suggest_field_improvements(&self, field: &Column, table: &Table) -> Vec<FieldSuggestion> {
        let mut suggestions = Vec::new();

        // Suggest adding NOT NULL for primary keys
        if field.is_primary_key(&table.primary_key) && field.nullable {
            suggestions.push(FieldSuggestion {
                field_name: field.name.clone(),
                suggestion_type: SuggestionType::AddConstraint,
                message: "Primary key fields should be NOT NULL".to_string(),
                sql_fix: format!("ALTER TABLE {} MODIFY {} {} NOT NULL", table.name, field.name, field.get_data_type_name()),
            });
        }

        // Suggest shorter field names
        if field.name.len() > 30 {
            suggestions.push(FieldSuggestion {
                field_name: field.name.clone(),
                suggestion_type: SuggestionType::Naming,
                message: format!("Field name '{}' is quite long ({} characters)", field.name, field.name.len()),
                sql_fix: format!("Consider using a shorter alias or renaming the field"),
            });
        }

        // Suggest adding default values for nullable fields
        if field.nullable && field.default_value.is_none() && !field.is_foreign_key(&table.foreign_keys) {
            suggestions.push(FieldSuggestion {
                field_name: field.name.clone(),
                suggestion_type: SuggestionType::AddDefault,
                message: "Consider adding a default value for nullable fields".to_string(),
                sql_fix: format!("ALTER TABLE {} ALTER COLUMN {} SET DEFAULT <value>", table.name, field.name),
            });
        }

        // Suggest using appropriate data types
        match &field.data_type {
            DataType::VarChar { length: None } => {
                suggestions.push(FieldSuggestion {
                    field_name: field.name.clone(),
                    suggestion_type: SuggestionType::DataType,
                    message: "VARCHAR without length limit may cause performance issues".to_string(),
                    sql_fix: format!("ALTER TABLE {} MODIFY {} VARCHAR(255)", table.name, field.name),
                });
            }
            DataType::Text => {
                suggestions.push(FieldSuggestion {
                    field_name: field.name.clone(),
                    suggestion_type: SuggestionType::DataType,
                    message: "TEXT fields cannot have default values in some databases".to_string(),
                    sql_fix: format!("Consider using VARCHAR with appropriate length"),
                });
            }
            _ => {}
        }

        suggestions
    }

    pub fn analyze_field_relationships(&self, field: &Column, database: &Database) -> FieldRelationshipAnalysis {
        let mut referenced_by = Vec::new();
        let mut references = Vec::new();
        let mut related_tables = HashSet::new();

        // Find tables that reference this field's table via foreign keys
        if let Some(table) = database.get_table(None, &field.name) {
            // This field might be referenced by other tables
            for other_table in database.get_all_tables() {
                for fk in &other_table.foreign_keys {
                    if fk.referenced_table == table.name {
                        referenced_by.push(format!("{}.{}", other_table.name, fk.columns.join(", ")));
                        related_tables.insert(other_table.name.clone());
                    }
                }
            }
        }

        // Find what this field references (if it's a foreign key)
        for table in database.get_all_tables() {
            if let Some(column) = table.get_column(&field.name) {
                for fk in &table.foreign_keys {
                    if fk.columns.contains(&field.name) {
                        references.push(format!("{}.{}", fk.referenced_table, fk.referenced_columns.join(", ")));
                        related_tables.insert(fk.referenced_table.clone());
                    }
                }
            }
        }

        FieldRelationshipAnalysis {
            field_name: field.name.clone(),
            references,
            referenced_by,
            related_tables: related_tables.into_iter().collect(),
        }
    }

    pub fn convert_data_type(&self, data_type: &str, target_dialect: &SqlDialect) -> Option<DataType> {
        // This would handle cross-dialect data type conversion
        // For now, just return the mapped type if available
        self.type_mappings.get(data_type).cloned()
    }

    fn validate_data_type(&self, data_type: &DataType) -> Result<(), FieldValidationError> {
        match data_type {
            DataType::Decimal { precision, scale } => {
                if let (Some(p), Some(s)) = (precision, scale) {
                    if s > p {
                        return Err(FieldValidationError {
                            rule: "decimal_scale_precision".to_string(),
                            message: format!("Decimal scale ({}) cannot be greater than precision ({})", s, p),
                            severity: ValidationSeverity::Error,
                        });
                    }
                }
            }
            DataType::Char { length: Some(len) } | DataType::VarChar { length: Some(len) } => {
                if *len == 0 {
                    return Err(FieldValidationError {
                        rule: "string_length_zero".to_string(),
                        message: "String length cannot be zero".to_string(),
                        severity: ValidationSeverity::Error,
                    });
                }
                if *len > 65535 {
                    return Err(FieldValidationError {
                        rule: "string_length_too_large".to_string(),
                        message: "String length cannot exceed 65535 characters".to_string(),
                        severity: ValidationSeverity::Error,
                    });
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn validate_default_value(&self, data_type: &DataType, default_value: &str) -> Result<(), FieldValidationError> {
        match data_type {
            DataType::Boolean => {
                if !matches!(default_value.to_uppercase().as_str(), "TRUE" | "FALSE" | "1" | "0") {
                    return Err(FieldValidationError {
                        rule: "boolean_default".to_string(),
                        message: format!("Invalid boolean default value: {}", default_value),
                        severity: ValidationSeverity::Error,
                    });
                }
            }
            DataType::TinyInt { .. } | DataType::SmallInt { .. } | DataType::Integer { .. } | DataType::BigInt { .. } => {
                if default_value.parse::<i64>().is_err() {
                    return Err(FieldValidationError {
                        rule: "numeric_default".to_string(),
                        message: format!("Invalid numeric default value: {}", default_value),
                        severity: ValidationSeverity::Error,
                    });
                }
            }
            DataType::Float { .. } | DataType::Double | DataType::Real | DataType::Decimal { .. } => {
                if default_value.parse::<f64>().is_err() {
                    return Err(FieldValidationError {
                        rule: "decimal_default".to_string(),
                        message: format!("Invalid decimal default value: {}", default_value),
                        severity: ValidationSeverity::Error,
                    });
                }
            }
            DataType::Char { .. } | DataType::VarChar { .. } | DataType::Text |
            DataType::NChar { .. } | DataType::NVarChar { .. } | DataType::NText => {
                // String defaults are generally valid, but check for quotes
                if !default_value.starts_with('\'') || !default_value.ends_with('\'') {
                    return Err(FieldValidationError {
                        rule: "string_default_quotes".to_string(),
                        message: "String default values should be quoted".to_string(),
                        severity: ValidationSeverity::Warning,
                    });
                }
            }
            _ => {} // Other types are more flexible
        }
        Ok(())
    }

    fn is_reserved_keyword(&self, word: &str) -> bool {
        let config = self.dialect.config();
        config.keywords.contains(&word.to_uppercase())
    }

    fn is_valid_identifier(&self, identifier: &str) -> bool {
        if identifier.is_empty() {
            return false;
        }

        // First character must be letter or underscore
        let first_char = identifier.chars().next().unwrap();
        if !first_char.is_alphabetic() && first_char != '_' {
            return false;
        }

        // Remaining characters must be alphanumeric or underscore
        for ch in identifier.chars().skip(1) {
            if !ch.is_alphanumeric() && ch != '_' {
                return false;
            }
        }

        true
    }

    fn build_type_mappings(dialect: &SqlDialect) -> HashMap<String, DataType> {
        let config = dialect.config();
        config.data_types
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FieldSuggestion {
    pub field_name: String,
    pub suggestion_type: SuggestionType,
    pub message: String,
    pub sql_fix: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SuggestionType {
    AddConstraint,
    Naming,
    AddDefault,
    DataType,
    Performance,
    Security,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FieldRelationshipAnalysis {
    pub field_name: String,
    pub references: Vec<String>,
    pub referenced_by: Vec<String>,
    pub related_tables: Vec<String>,
}

pub struct FieldDocumentationGenerator;

impl FieldDocumentationGenerator {
    pub fn generate_markdown(fields: &[Column], table_name: &str) -> String {
        let mut markdown = format!("## {} Table Fields\n\n", table_name);
        markdown.push_str("| Field | Type | Nullable | Default | Description |\n");
        markdown.push_str("|-------|------|----------|---------|-------------|\n");

        for field in fields {
            let nullable = if field.nullable { "✓" } else { "✗" };
            let default = field.default_value.as_deref().unwrap_or("-");
            let description = field.metadata.description.as_deref().unwrap_or("-");

            markdown.push_str(&format!(
                "| {} | {} | {} | {} | {} |\n",
                field.name,
                field.get_data_type_name(),
                nullable,
                default,
                description
            ));
        }

        markdown.push_str("\n");

        // Add constraints section
        let constraints: Vec<_> = fields.iter()
            .filter_map(|field| {
                if !field.nullable {
                    Some(format!("- `{}`: NOT NULL", field.name))
                } else {
                    None
                }
            })
            .collect();

        if !constraints.is_empty() {
            markdown.push_str("### Constraints\n\n");
            for constraint in constraints {
                markdown.push_str(&format!("{}\n", constraint));
            }
            markdown.push_str("\n");
        }

        markdown
    }

    pub fn generate_api_spec(fields: &[Column]) -> serde_json::Value {
        let properties = fields.iter()
            .filter_map(|field| {
                field.metadata.api_field.as_ref().map(|api| {
                    let mut property = serde_json::json!({
                        "type": field.data_type.get_simple_name().to_lowercase(),
                        "description": field.metadata.description
                    });

                    if !api.required {
                        property["nullable"] = serde_json::Value::Bool(true);
                    }

                    if let Some(example) = &api.example {
                        property["example"] = serde_json::Value::String(example.clone());
                    }

                    (api.name.clone(), property)
                })
            })
            .collect::<serde_json::Map<String, serde_json::Value>>();

        let required_fields: Vec<String> = fields.iter()
            .filter_map(|field| {
                field.metadata.api_field.as_ref()
                    .filter(|api| api.required)
                    .map(|api| api.name.clone())
            })
            .collect();

        let mut schema = serde_json::json!({
            "type": "object",
            "properties": serde_json::Value::Object(properties)
        });

        if !required_fields.is_empty() {
            schema["required"] = serde_json::Value::Array(
                required_fields.into_iter().map(serde_json::Value::String).collect()
            );
        }

        schema
    }
}

pub struct FieldTransformationService;

impl FieldTransformationService {
    pub fn normalize_field_name(name: &str, naming_convention: &NamingConvention) -> String {
        match naming_convention {
            NamingConvention::SnakeCase => Self::to_snake_case(name),
            NamingConvention::CamelCase => Self::to_camel_case(name),
            NamingConvention::PascalCase => Self::to_pascal_case(name),
            NamingConvention::KebabCase => Self::to_kebab_case(name),
            NamingConvention::ScreamingSnakeCase => Self::to_screaming_snake_case(name),
        }
    }

    pub fn generate_field_alias(field: &Column, convention: &NamingConvention) -> String {
        Self::normalize_field_name(&field.name, convention)
    }

    pub fn suggest_field_renames(fields: &[Column], convention: &NamingConvention) -> Vec<FieldRenameSuggestion> {
        fields.iter()
            .filter_map(|field| {
                let suggested_name = Self::normalize_field_name(&field.name, convention);
                if suggested_name != field.name {
                    Some(FieldRenameSuggestion {
                        current_name: field.name.clone(),
                        suggested_name,
                        reason: format!("Field name should follow {} convention", convention),
                    })
                } else {
                    None
                }
            })
            .collect()
    }

    fn to_snake_case(s: &str) -> String {
        let mut result = String::new();
        let chars: Vec<char> = s.chars().collect();

        for (i, &ch) in chars.iter().enumerate() {
            if ch.is_uppercase() {
                if i > 0 && chars[i - 1].is_lowercase() {
                    result.push('_');
                }
                result.push(ch.to_lowercase().next().unwrap());
            } else {
                result.push(ch);
            }
        }

        result
    }

    fn to_camel_case(s: &str) -> String {
        let mut result = String::new();
        let mut capitalize_next = false;

        for ch in s.chars() {
            if ch == '_' {
                capitalize_next = true;
            } else if capitalize_next {
                result.push(ch.to_uppercase().next().unwrap());
                capitalize_next = false;
            } else if result.is_empty() {
                result.push(ch.to_lowercase().next().unwrap());
            } else {
                result.push(ch);
            }
        }

        result
    }

    fn to_pascal_case(s: &str) -> String {
        let camel = Self::to_camel_case(s);
        let mut chars: Vec<char> = camel.chars().collect();
        if let Some(first) = chars.first_mut() {
            *first = first.to_uppercase().next().unwrap();
        }
        chars.into_iter().collect()
    }

    fn to_kebab_case(s: &str) -> String {
        Self::to_snake_case(s).replace('_', "-")
    }

    fn to_screaming_snake_case(s: &str) -> String {
        Self::to_snake_case(s).to_uppercase()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum NamingConvention {
    SnakeCase,
    CamelCase,
    PascalCase,
    KebabCase,
    ScreamingSnakeCase,
}

impl std::fmt::Display for NamingConvention {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NamingConvention::SnakeCase => write!(f, "snake_case"),
            NamingConvention::CamelCase => write!(f, "camelCase"),
            NamingConvention::PascalCase => write!(f, "PascalCase"),
            NamingConvention::KebabCase => write!(f, "kebab-case"),
            NamingConvention::ScreamingSnakeCase => write!(f, "SCREAMING_SNAKE_CASE"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FieldRenameSuggestion {
    pub current_name: String,
    pub suggested_name: String,
    pub reason: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field_validation() {
        let service = FieldService::new(SqlDialect::PostgreSQL);
        let table = Table::new("test_table");

        let valid_field = Column::new("valid_field", DataType::Integer { unsigned: false });
        let result = service.validate_field(&valid_field, &table);
        assert!(result.is_valid);

        let invalid_field = Column::new("", DataType::Integer { unsigned: false });
        let result = service.validate_field(&invalid_field, &table);
        assert!(!result.is_valid);
        assert!(!result.errors.is_empty());
    }

    #[test]
    fn test_data_type_compatibility() {
        let service = FieldService::new(SqlDialect::PostgreSQL);

        let result = service.check_field_compatibility(
            &DataType::Integer { unsigned: false },
            &DataType::BigInt { unsigned: false }
        );
        assert!(result.is_compatible);
        assert!(result.conversion_required);
    }

    #[test]
    fn test_naming_conventions() {
        assert_eq!(FieldTransformationService::to_snake_case("UserName"), "user_name");
        assert_eq!(FieldTransformationService::to_camel_case("user_name"), "userName");
        assert_eq!(FieldTransformationService::to_pascal_case("user_name"), "UserName");
        assert_eq!(FieldTransformationService::to_kebab_case("user_name"), "user-name");
        assert_eq!(FieldTransformationService::to_screaming_snake_case("user_name"), "USER_NAME");
    }

    #[test]
    fn test_identifier_validation() {
        let service = FieldService::new(SqlDialect::PostgreSQL);

        assert!(service.is_valid_identifier("valid_name"));
        assert!(service.is_valid_identifier("_private"));
        assert!(service.is_valid_identifier("field123"));
        assert!(!service.is_valid_identifier(""));
        assert!(!service.is_valid_identifier("123invalid"));
        assert!(!service.is_valid_identifier("invalid-name"));
    }
}
