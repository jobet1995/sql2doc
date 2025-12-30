use std::collections::{HashMap, HashSet};
use serde::{Deserialize, Serialize};

use crate::core::domain::entity::*;
use crate::core::domain::relationships::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JunctionTable {
    pub table_name: String,
    pub left_table: String,
    pub right_table: String,
    pub left_columns: Vec<String>,
    pub right_columns: Vec<String>,
    pub junction_columns: Vec<JunctionColumn>,
    pub primary_key: Option<PrimaryKey>,
    pub foreign_keys: Vec<ForeignKey>,
    pub additional_columns: Vec<Column>,
    pub metadata: JunctionTableMetadata,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JunctionColumn {
    pub column: Column,
    pub relationship_role: RelationshipRole,
    pub refers_to_table: String,
    pub refers_to_column: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RelationshipRole {
    LeftForeignKey,
    RightForeignKey,
    JunctionData,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JunctionTableMetadata {
    pub description: Option<String>,
    pub relationship_description: Option<String>,
    pub left_cardinality: CardinalityDescription,
    pub right_cardinality: CardinalityDescription,
    pub junction_purpose: JunctionPurpose,
    pub tags: Vec<String>,
    pub custom_properties: HashMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CardinalityDescription {
    pub min: u32,
    pub max: Option<u32>, // None means unlimited
    pub description: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum JunctionPurpose {
    ManyToManyRelationship,
    HierarchicalRelationship,
    PolymorphicAssociation,
    AuditTrail,
    TemporalRelationship,
    Custom(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JunctionTableAnalysis {
    pub table_name: String,
    pub is_junction_table: bool,
    pub confidence_score: f64, // 0.0 to 1.0
    pub junction_type: Option<JunctionType>,
    pub detected_relationships: Vec<DetectedRelationship>,
    pub issues: Vec<JunctionIssue>,
    pub recommendations: Vec<JunctionRecommendation>,
    pub metadata: JunctionTableMetadata,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum JunctionType {
    SimpleManyToMany,
    CompositeManyToMany,
    SelfReferencing,
    Polymorphic,
    Temporal,
    ExtendedProperties,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DetectedRelationship {
    pub left_table: String,
    pub right_table: String,
    pub left_columns: Vec<String>,
    pub right_columns: Vec<String>,
    pub relationship_type: RelationshipType,
    pub constraints: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JunctionIssue {
    pub issue_type: JunctionIssueType,
    pub severity: IssueSeverity,
    pub description: String,
    pub affected_columns: Vec<String>,
    pub sql_fix: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum JunctionIssueType {
    MissingPrimaryKey,
    MissingForeignKeys,
    IncorrectCardinality,
    RedundantColumns,
    DataIntegrity,
    Performance,
    NamingConvention,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum IssueSeverity {
    Critical,
    High,
    Medium,
    Low,
    Info,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JunctionRecommendation {
    pub recommendation_type: RecommendationType,
    pub description: String,
    pub benefit: String,
    pub effort: EffortLevel,
    pub sql_commands: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RecommendationType {
    AddConstraint,
    RemoveRedundancy,
    ImproveNaming,
    OptimizePerformance,
    EnhanceDataIntegrity,
    AddDocumentation,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EffortLevel {
    Trivial,
    Low,
    Medium,
    High,
    VeryHigh,
}

pub struct JunctionTableDetectionService;

impl JunctionTableDetectionService {
    pub fn analyze_table(table: &Table, database: &Database) -> JunctionTableAnalysis {
        let foreign_keys = &table.foreign_keys;
        let columns = &table.columns;

        // Basic junction table criteria
        let has_multiple_foreign_keys = foreign_keys.len() >= 2;
        let has_few_additional_columns = Self::count_additional_columns(table) <= 3;
        let has_composite_primary_key = Self::has_composite_primary_key_from_foreign_keys(table);
        let follows_naming_convention = Self::follows_junction_naming_convention(&table.name);

        // Calculate confidence score
        let mut confidence_score = 0.0;
        if has_multiple_foreign_keys { confidence_score += 0.4; }
        if has_few_additional_columns { confidence_score += 0.3; }
        if has_composite_primary_key { confidence_score += 0.2; }
        if follows_naming_convention { confidence_score += 0.1; }

        // Determine if it's likely a junction table
        let is_junction_table = confidence_score >= 0.6;

        // Analyze relationships
        let detected_relationships = if is_junction_table {
            Self::analyze_junction_relationships(table, database)
        } else {
            Vec::new()
        };

        // Determine junction type
        let junction_type = if is_junction_table {
            Some(Self::determine_junction_type(table, &detected_relationships))
        } else {
            None
        };

        // Find issues
        let issues = Self::analyze_issues(table, is_junction_table);

        // Generate recommendations
        let recommendations = Self::generate_recommendations(table, &issues, is_junction_table);

        // Create metadata
        let metadata = Self::create_junction_metadata(table, &detected_relationships, junction_type.as_ref());

        JunctionTableAnalysis {
            table_name: table.name.clone(),
            is_junction_table,
            confidence_score,
            junction_type,
            detected_relationships,
            issues,
            recommendations,
            metadata,
        }
    }

    pub fn find_all_junction_tables(database: &Database) -> Vec<JunctionTableAnalysis> {
        database.get_all_tables()
            .iter()
            .map(|table| Self::analyze_table(table, database))
            .filter(|analysis| analysis.is_junction_table)
            .collect()
    }

    pub fn create_junction_table(table: &Table, analysis: &JunctionTableAnalysis) -> Option<JunctionTable> {
        if !analysis.is_junction_table || analysis.detected_relationships.len() < 2 {
            return None;
        }

        let left_rel = &analysis.detected_relationships[0];
        let right_rel = &analysis.detected_relationships[1];

        let junction_columns = Self::categorize_columns(table, &analysis.detected_relationships);

        Some(JunctionTable {
            table_name: table.name.clone(),
            left_table: left_rel.left_table.clone(),
            right_table: right_rel.right_table.clone(),
            left_columns: left_rel.left_columns.clone(),
            right_columns: right_rel.right_columns.clone(),
            junction_columns,
            primary_key: table.primary_key.clone(),
            foreign_keys: table.foreign_keys.clone(),
            additional_columns: Self::get_additional_columns(table, &analysis.detected_relationships),
            metadata: analysis.metadata.clone(),
        })
    }

    fn count_additional_columns(table: &Table) -> usize {
        table.columns.iter()
            .filter(|col| {
                !table.foreign_keys.iter().any(|fk| fk.columns.contains(&col.name)) &&
                !table.primary_key.as_ref().map_or(false, |pk| pk.columns.contains(&col.name))
            })
            .count()
    }

    fn has_composite_primary_key_from_foreign_keys(table: &Table) -> bool {
        if let Some(pk) = &table.primary_key {
            if pk.columns.len() >= 2 {
                // Check if primary key columns are foreign key columns
                let fk_columns: HashSet<_> = table.foreign_keys.iter()
                    .flat_map(|fk| &fk.columns)
                    .collect();

                return pk.columns.iter().all(|col| fk_columns.contains(col));
            }
        }
        false
    }

    fn follows_junction_naming_convention(table_name: &str) -> bool {
        let name = table_name.to_lowercase();

        // Common junction table naming patterns
        name.contains("_to_") ||
        name.contains("_and_") ||
        name.contains("junction") ||
        name.contains("link") ||
        name.contains("assoc") ||
        name.contains("relation") ||
        // Check for concatenated table names
        Self::is_concatenated_table_names(&name)
    }

    fn is_concatenated_table_names(name: &str) -> bool {
        // Simple heuristic: if name contains underscores and parts look like table names
        let parts: Vec<&str> = name.split('_').collect();
        parts.len() >= 3 && parts.iter().all(|part| part.len() >= 2 && part.chars().all(|c| c.is_alphanumeric()))
    }

    fn analyze_junction_relationships(table: &Table, database: &Database) -> Vec<DetectedRelationship> {
        let mut relationships = Vec::new();

        for fk in &table.foreign_keys {
            if let Some(referenced_table) = database.get_table(None, &fk.referenced_table) {
                let relationship_type = Self::infer_relationship_type(fk, table, referenced_table);

                relationships.push(DetectedRelationship {
                    left_table: table.name.clone(),
                    right_table: fk.referenced_table.clone(),
                    left_columns: fk.columns.clone(),
                    right_columns: fk.referenced_columns.clone(),
                    relationship_type,
                    constraints: vec![format!("FOREIGN KEY ({}) REFERENCES {} ({})",
                        fk.columns.join(", "), fk.referenced_table, fk.referenced_columns.join(", "))],
                });
            }
        }

        relationships
    }

    fn infer_relationship_type(fk: &ForeignKey, source_table: &Table, target_table: &Table) -> RelationshipType {
        // For junction tables, relationships are typically many-to-one from junction to referenced tables
        RelationshipType::ManyToOne
    }

    fn determine_junction_type(table: &Table, relationships: &[DetectedRelationship]) -> JunctionType {
        let additional_columns = Self::count_additional_columns(table);

        if additional_columns == 0 {
            JunctionType::SimpleManyToMany
        } else if additional_columns <= 2 {
            JunctionType::ExtendedProperties
        } else if Self::has_temporal_columns(table) {
            JunctionType::Temporal
        } else if Self::is_self_referencing(relationships) {
            JunctionType::SelfReferencing
        } else {
            JunctionType::CompositeManyToMany
        }
    }

    fn has_temporal_columns(table: &Table) -> bool {
        table.columns.iter().any(|col| {
            matches!(col.data_type, DataType::Date | DataType::Time | DataType::DateTime | DataType::Timestamp)
        })
    }

    fn is_self_referencing(relationships: &[DetectedRelationship]) -> bool {
        relationships.iter().any(|rel| rel.left_table == rel.right_table)
    }

    fn analyze_issues(table: &Table, is_junction_table: bool) -> Vec<JunctionIssue> {
        let mut issues = Vec::new();

        if is_junction_table {
            // Check for missing primary key
            if table.primary_key.is_none() {
                issues.push(JunctionIssue {
                    issue_type: JunctionIssueType::MissingPrimaryKey,
                    severity: IssueSeverity::Critical,
                    description: "Junction table should have a primary key".to_string(),
                    affected_columns: Vec::new(),
                    sql_fix: Some(format!("ALTER TABLE {} ADD PRIMARY KEY ({});",
                        table.name,
                        table.foreign_keys.iter()
                            .flat_map(|fk| &fk.columns)
                            .collect::<Vec<_>>()
                            .join(", "))),
                });
            }

            // Check for missing foreign keys
            if table.foreign_keys.len() < 2 {
                issues.push(JunctionIssue {
                    issue_type: JunctionIssueType::MissingForeignKeys,
                    severity: IssueSeverity::High,
                    description: "Junction table should have at least two foreign keys".to_string(),
                    affected_columns: Vec::new(),
                    sql_fix: None,
                });
            }

            // Check for nullable foreign key columns
            for fk in &table.foreign_keys {
                for col_name in &fk.columns {
                    if let Some(column) = table.get_column(col_name) {
                        if column.nullable {
                            issues.push(JunctionIssue {
                                issue_type: JunctionIssueType::DataIntegrity,
                                severity: IssueSeverity::High,
                                description: format!("Foreign key column '{}' should not be nullable in junction table", col_name),
                                affected_columns: vec![col_name.clone()],
                                sql_fix: Some(format!("ALTER TABLE {} MODIFY {} {} NOT NULL;",
                                    table.name, col_name, Self::get_data_type_name(&column.data_type))),
                            });
                        }
                    }
                }
            }
        }

        issues
    }

    fn generate_recommendations(table: &Table, issues: &[JunctionIssue], is_junction_table: bool) -> Vec<JunctionRecommendation> {
        let mut recommendations = Vec::new();

        if is_junction_table {
            // Add indexes on foreign key columns if not already present
            for fk in &table.foreign_keys {
                let has_index = table.indexes.iter().any(|idx| {
                    idx.columns == fk.columns
                });

                if !has_index {
                    recommendations.push(JunctionRecommendation {
                        recommendation_type: RecommendationType::OptimizePerformance,
                        description: format!("Add index on foreign key columns ({}) for better performance", fk.columns.join(", ")),
                        benefit: "Improves join performance and foreign key lookups".to_string(),
                        effort: EffortLevel::Low,
                        sql_commands: vec![format!("CREATE INDEX idx_{}_{} ON {} ({});",
                            table.name, fk.columns.join("_"), table.name, fk.columns.join(", "))],
                    });
                }
            }

            // Suggest adding CASCADE DELETE/UPDATE if not present
            for fk in &table.foreign_keys {
                if fk.on_delete == ReferentialAction::NoAction {
                    recommendations.push(JunctionRecommendation {
                        recommendation_type: RecommendationType::EnhanceDataIntegrity,
                        description: format!("Consider adding CASCADE DELETE to foreign key for automatic cleanup"),
                        benefit: "Ensures data consistency when referenced records are deleted".to_string(),
                        effort: EffortLevel::Medium,
                        sql_commands: vec![format!("ALTER TABLE {} DROP CONSTRAINT {}, ADD CONSTRAINT {} FOREIGN KEY ({}) REFERENCES {} ({}) ON DELETE CASCADE;",
                            table.name, fk.name.as_deref().unwrap_or("fk_constraint"),
                            fk.name.as_deref().unwrap_or("fk_constraint"),
                            fk.columns.join(", "), fk.referenced_table, fk.referenced_columns.join(", "))],
                    });
                }
            }
        }

        recommendations
    }

    fn create_junction_metadata(table: &Table, relationships: &[DetectedRelationship], junction_type: Option<&JunctionType>) -> JunctionTableMetadata {
        let left_cardinality = if relationships.len() >= 2 {
            CardinalityDescription {
                min: 0,
                max: None,
                description: "Many (junction table can reference multiple records)".to_string(),
            }
        } else {
            CardinalityDescription {
                min: 0,
                max: Some(1),
                description: "Optional single reference".to_string(),
            }
        };

        let right_cardinality = left_cardinality.clone();

        let junction_purpose = match junction_type {
            Some(JunctionType::SimpleManyToMany) => JunctionPurpose::ManyToManyRelationship,
            Some(JunctionType::Temporal) => JunctionPurpose::TemporalRelationship,
            Some(JunctionType::SelfReferencing) => JunctionPurpose::HierarchicalRelationship,
            _ => JunctionPurpose::ManyToManyRelationship,
        };

        JunctionTableMetadata {
            description: Some(format!("Junction table implementing many-to-many relationship between {} tables",
                relationships.len())),
            relationship_description: Some(Self::generate_relationship_description(relationships)),
            left_cardinality,
            right_cardinality,
            junction_purpose,
            tags: vec!["junction".to_string(), "many-to-many".to_string()],
            custom_properties: HashMap::new(),
        }
    }

    fn generate_relationship_description(relationships: &[DetectedRelationship]) -> String {
        if relationships.is_empty() {
            "No relationships detected".to_string()
        } else if relationships.len() == 2 {
            format!("Links {} to {}", relationships[0].right_table, relationships[1].right_table)
        } else {
            format!("Links {} and {} other tables",
                relationships[0].right_table,
                relationships.len() - 1)
        }
    }

    fn categorize_columns(table: &Table, relationships: &[DetectedRelationship]) -> Vec<JunctionColumn> {
        table.columns.iter().map(|column| {
            // Determine which relationship this column belongs to
            let mut relationship_role = RelationshipRole::JunctionData;
            let mut refers_to_table = String::new();
            let mut refers_to_column = String::new();

            for rel in relationships {
                if rel.left_columns.contains(&column.name) {
                    relationship_role = RelationshipRole::LeftForeignKey;
                    refers_to_table = rel.right_table.clone();
                    if let Some(col_idx) = rel.left_columns.iter().position(|c| c == &column.name) {
                        if col_idx < rel.right_columns.len() {
                            refers_to_column = rel.right_columns[col_idx].clone();
                        }
                    }
                    break;
                }
            }

            JunctionColumn {
                column: column.clone(),
                relationship_role,
                refers_to_table,
                refers_to_column,
            }
        }).collect()
    }

    fn get_additional_columns(table: &Table, relationships: &[DetectedRelationship]) -> Vec<Column> {
        let fk_columns: HashSet<_> = relationships.iter()
            .flat_map(|rel| &rel.left_columns)
            .collect();

        let pk_columns: HashSet<_> = table.primary_key.as_ref()
            .map(|pk| pk.columns.iter().collect())
            .unwrap_or_default();

        table.columns.iter()
            .filter(|col| !fk_columns.contains(&col.name) && !pk_columns.contains(&col.name))
            .cloned()
            .collect()
    }

    fn get_data_type_name(data_type: &DataType) -> String {
        match data_type {
            DataType::Integer { .. } => "INTEGER".to_string(),
            DataType::BigInt { .. } => "BIGINT".to_string(),
            DataType::VarChar { .. } => "VARCHAR(255)".to_string(),
            _ => data_type.get_simple_name(),
        }
    }
}

pub struct JunctionTableOptimizationService;

impl JunctionTableOptimizationService {
    pub fn optimize_junction_table(junction: &JunctionTable, database: &Database) -> Vec<JunctionOptimization> {
        let mut optimizations = Vec::new();

        // Check for redundant junction tables
        if Self::is_redundant_junction(junction, database) {
            optimizations.push(JunctionOptimization {
                optimization_type: JunctionOptimizationType::RemoveRedundancy,
                description: "This junction table may be redundant - consider direct many-to-many relationship".to_string(),
                impact: OptimizationImpact::High,
                sql_commands: vec![
                    format!("-- Consider removing table {} and implementing direct relationship", junction.table_name),
                    "-- This requires application changes to handle many-to-many relationships".to_string(),
                ],
            });
        }

        // Suggest adding surrogate primary key
        if junction.primary_key.as_ref().map_or(false, |pk| pk.columns.len() > 2) {
            optimizations.push(JunctionOptimization {
                optimization_type: JunctionOptimizationType::AddSurrogateKey,
                description: "Consider adding a surrogate primary key for better performance".to_string(),
                impact: OptimizationImpact::Medium,
                sql_commands: vec![
                    format!("ALTER TABLE {} ADD COLUMN id SERIAL PRIMARY KEY;", junction.table_name),
                    format!("CREATE UNIQUE INDEX idx_{}_composite ON {} ({}, {});",
                        junction.table_name, junction.table_name,
                        junction.left_columns.join(", "), junction.right_columns.join(", ")),
                ],
            });
        }

        // Check for missing indexes
        for fk in &junction.foreign_keys {
            optimizations.push(JunctionOptimization {
                optimization_type: JunctionOptimizationType::AddIndex,
                description: format!("Add index on foreign key columns ({})", fk.columns.join(", ")),
                impact: OptimizationImpact::Low,
                sql_commands: vec![format!("CREATE INDEX idx_{}_{} ON {} ({});",
                    junction.table_name, fk.columns.join("_"), junction.table_name, fk.columns.join(", "))],
            });
        }

        // Suggest partitioning for large junction tables
        optimizations.push(JunctionOptimization {
            optimization_type: JunctionOptimizationType::AddPartitioning,
            description: "Consider partitioning large junction tables by date or range".to_string(),
            impact: OptimizationImpact::Medium,
            sql_commands: vec![
                format!("-- Example partitioning by left table ID"),
                format!("-- CREATE TABLE {}_partitioned PARTITION OF {} FOR VALUES FROM (0) TO (1000000);", junction.table_name, junction.table_name),
            ],
        });

        optimizations
    }

    pub fn analyze_junction_performance(junction: &JunctionTable, database: &Database) -> JunctionPerformanceAnalysis {
        let table_size_estimate = junction.additional_columns.len() * 100; // Rough estimate
        let relationship_count = junction.foreign_keys.len();

        let complexity_score = match relationship_count {
            0..=1 => PerformanceComplexity::Simple,
            2 => PerformanceComplexity::Moderate,
            3..=5 => PerformanceComplexity::Complex,
            _ => PerformanceComplexity::VeryComplex,
        };

        let recommendations = match complexity_score {
            PerformanceComplexity::Simple => vec!["Ensure foreign key indexes are in place".to_string()],
            PerformanceComplexity::Moderate => vec![
                "Consider composite indexes on foreign key combinations".to_string(),
                "Monitor query performance on joins through this junction".to_string(),
            ],
            PerformanceComplexity::Complex => vec![
                "Implement query result caching".to_string(),
                "Consider denormalization for read-heavy workloads".to_string(),
                "Review and optimize complex join queries".to_string(),
            ],
            PerformanceComplexity::VeryComplex => vec![
                "Consider redesigning the data model".to_string(),
                "Implement advanced indexing strategies".to_string(),
                "Use materialized views for complex aggregations".to_string(),
            ],
        };

        JunctionPerformanceAnalysis {
            table_name: junction.table_name.clone(),
            estimated_row_count: None, // Would need actual statistics
            estimated_size_mb: table_size_estimate,
            complexity_score,
            performance_recommendations: recommendations,
            indexing_strategy: Self::suggest_indexing_strategy(junction),
        }
    }

    fn is_redundant_junction(junction: &JunctionTable, database: &Database) -> bool {
        // Check if there's already a direct many-to-many relationship
        // or if the junction table only has foreign keys and no additional data
        junction.additional_columns.is_empty() &&
        junction.foreign_keys.len() == 2 &&
        Self::has_direct_relationship(&junction.left_table, &junction.right_table, database)
    }

    fn has_direct_relationship(table1: &str, table2: &str, database: &Database) -> bool {
        if let Some(table) = database.get_table(None, table1) {
            return table.foreign_keys.iter().any(|fk| fk.referenced_table == table2);
        }
        false
    }

    fn suggest_indexing_strategy(junction: &JunctionTable) -> Vec<String> {
        let mut strategies = Vec::new();

        // Composite index on all foreign keys
        if junction.foreign_keys.len() >= 2 {
            let all_fk_columns: Vec<String> = junction.foreign_keys.iter()
                .flat_map(|fk| fk.columns.clone())
                .collect();

            strategies.push(format!("Composite index: CREATE INDEX idx_{}_all_fk ON {} ({});",
                junction.table_name, junction.table_name, all_fk_columns.join(", ")));
        }

        // Individual indexes for each foreign key
        for fk in &junction.foreign_keys {
            strategies.push(format!("Foreign key index: CREATE INDEX idx_{}_{} ON {} ({});",
                junction.table_name, fk.columns.join("_"), junction.table_name, fk.columns.join(", ")));
        }

        // Indexes on commonly queried combinations
        if !junction.additional_columns.is_empty() {
            for additional_col in &junction.additional_columns {
                strategies.push(format!("Query optimization index: CREATE INDEX idx_{}_{} ON {} ({}, {});",
                    junction.table_name, additional_col.name, junction.table_name,
                    junction.left_columns.join(", "), additional_col.name));
            }
        }

        strategies
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JunctionOptimization {
    pub optimization_type: JunctionOptimizationType,
    pub description: String,
    pub impact: OptimizationImpact,
    pub sql_commands: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum JunctionOptimizationType {
    AddSurrogateKey,
    AddIndex,
    RemoveRedundancy,
    AddPartitioning,
    OptimizeConstraints,
    AddTriggers,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum OptimizationImpact {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JunctionPerformanceAnalysis {
    pub table_name: String,
    pub estimated_row_count: Option<u64>,
    pub estimated_size_mb: usize,
    pub complexity_score: PerformanceComplexity,
    pub performance_recommendations: Vec<String>,
    pub indexing_strategy: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PerformanceComplexity {
    Simple,
    Moderate,
    Complex,
    VeryComplex,
}

pub struct JunctionTableDocumentationService;

impl JunctionTableDocumentationService {
    pub fn generate_junction_documentation(junction: &JunctionTable) -> JunctionDocumentation {
        let relationship_summary = format!(
            "Junction table linking '{}' and '{}' tables",
            junction.left_table, junction.right_table
        );

        let column_documentation = junction.junction_columns.iter()
            .map(|jc| JunctionColumnDocumentation {
                name: jc.column.name.clone(),
                data_type: jc.column.get_data_type_name(),
                role: match jc.relationship_role {
                    RelationshipRole::LeftForeignKey => "Foreign key to left table".to_string(),
                    RelationshipRole::RightForeignKey => "Foreign key to right table".to_string(),
                    RelationshipRole::JunctionData => "Additional junction data".to_string(),
                },
                nullable: jc.column.nullable,
                references: if jc.refers_to_table.is_empty() {
                    None
                } else {
                    Some(format!("{}.{}", jc.refers_to_table, jc.refers_to_column))
                },
            })
            .collect();

        let usage_examples = Self::generate_usage_examples(junction);

        JunctionDocumentation {
            table_name: junction.table_name.clone(),
            relationship_summary,
            purpose: Self::describe_purpose(&junction.metadata.junction_purpose),
            column_documentation,
            usage_examples,
            constraints: Self::document_constraints(junction),
            performance_notes: Self::generate_performance_notes(junction),
        }
    }

    pub fn generate_api_relationship_documentation(junction: &JunctionTable) -> ApiRelationshipDocumentation {
        ApiRelationshipDocumentation {
            relationship_name: format!("{}To{}", junction.left_table, junction.right_table),
            left_entity: junction.left_table.clone(),
            right_entity: junction.right_table.clone(),
            junction_entity: junction.table_name.clone(),
            cardinality: "Many-to-Many".to_string(),
            endpoints: Self::suggest_api_endpoints(junction),
            data_transfer_objects: Self::suggest_dtos(junction),
        }
    }

    fn describe_purpose(purpose: &JunctionPurpose) -> String {
        match purpose {
            JunctionPurpose::ManyToManyRelationship => {
                "Implements a many-to-many relationship between two entities".to_string()
            }
            JunctionPurpose::HierarchicalRelationship => {
                "Represents hierarchical relationships (e.g., parent-child, categories)".to_string()
            }
            JunctionPurpose::PolymorphicAssociation => {
                "Enables polymorphic associations between different entity types".to_string()
            }
            JunctionPurpose::AuditTrail => {
                "Maintains audit trails and historical relationships".to_string()
            }
            JunctionPurpose::TemporalRelationship => {
                "Tracks relationships that change over time".to_string()
            }
            JunctionPurpose::Custom(description) => {
                format!("Custom purpose: {}", description)
            }
        }
    }

    fn generate_usage_examples(junction: &JunctionTable) -> Vec<String> {
        vec![
            format!("-- Find all {} associated with a specific {}\nSELECT j.* FROM {} j WHERE j.{} = ?;",
                junction.right_table, junction.left_table, junction.table_name,
                junction.left_columns.first().unwrap_or(&"left_id".to_string())),
            format!("-- Count relationships per {}\nSELECT {}, COUNT(*) FROM {} GROUP BY {};",
                junction.left_table, junction.left_columns.first().unwrap_or(&"left_id".to_string()),
                junction.table_name, junction.left_columns.first().unwrap_or(&"left_id".to_string())),
            format!("-- Complex join example\nSELECT l.*, r.* FROM {} l\nJOIN {} j ON l.id = j.{}\nJOIN {} r ON r.id = j.{};",
                junction.left_table, junction.table_name,
                junction.left_columns.first().unwrap_or(&"left_id".to_string()),
                junction.right_table, junction.right_columns.first().unwrap_or(&"right_id".to_string())),
        ]
    }

    fn document_constraints(junction: &JunctionTable) -> Vec<String> {
        let mut constraints = Vec::new();

        if let Some(pk) = &junction.primary_key {
            constraints.push(format!("PRIMARY KEY: ({})", pk.columns.join(", ")));
        }

        for fk in &junction.foreign_keys {
            constraints.push(format!("FOREIGN KEY ({} -> {}.{})",
                fk.columns.join(", "),
                fk.referenced_table,
                fk.referenced_columns.join(", ")));
        }

        constraints
    }

    fn generate_performance_notes(junction: &JunctionTable) -> Vec<String> {
        let mut notes = Vec::new();

        notes.push(format!("Ensure indexes exist on all foreign key columns"));
        notes.push(format!("Consider composite indexes for common query patterns"));
        notes.push(format!("Monitor join performance through this junction table"));

        if junction.additional_columns.len() > 2 {
            notes.push(format!("Large junction table - consider partitioning or archiving strategies"));
        }

        notes
    }

    fn suggest_api_endpoints(junction: &JunctionTable) -> Vec<String> {
        vec![
            format!("GET /{}/{{id}}/{} - Get all {} for a {}", junction.left_table, junction.right_table, junction.right_table, junction.left_table),
            format!("POST /{}/{{id}}/{} - Associate {} with {}", junction.left_table, junction.right_table, junction.right_table, junction.left_table),
            format!("DELETE /{}/{{id}}/{{}/{{id2}} - Remove association", junction.left_table, junction.right_table),
        ]
    }

    fn suggest_dtos(junction: &JunctionTable) -> Vec<String> {
        vec![
            format!("{}AssociationDTO - Contains {}, {}, and any additional junction data", junction.table_name, junction.left_table, junction.right_table),
            format!("{}With{}DTO - {} entity with nested {} associations", junction.left_table, junction.right_table, junction.left_table, junction.right_table),
        ]
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JunctionDocumentation {
    pub table_name: String,
    pub relationship_summary: String,
    pub purpose: String,
    pub column_documentation: Vec<JunctionColumnDocumentation>,
    pub usage_examples: Vec<String>,
    pub constraints: Vec<String>,
    pub performance_notes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JunctionColumnDocumentation {
    pub name: String,
    pub data_type: String,
    pub role: String,
    pub nullable: bool,
    pub references: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApiRelationshipDocumentation {
    pub relationship_name: String,
    pub left_entity: String,
    pub right_entity: String,
    pub junction_entity: String,
    pub cardinality: String,
    pub endpoints: Vec<String>,
    pub data_transfer_objects: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_junction_table_detection() {
        let mut database = Database::new("test_db");

        // Create a proper junction table
        let mut junction_table = Table::new("user_roles");
        junction_table.add_column(Column::new("user_id", DataType::Integer { unsigned: false }));
        junction_table.add_column(Column::new("role_id", DataType::Integer { unsigned: false }));
        junction_table.add_column(Column::new("assigned_at", DataType::DateTime));

        // Add foreign keys
        junction_table.add_foreign_key(ForeignKey {
            name: Some("fk_user_roles_user".to_string()),
            table_name: "user_roles".to_string(),
            columns: vec!["user_id".to_string()],
            referenced_table: "users".to_string(),
            referenced_columns: vec!["id".to_string()],
            on_delete: ReferentialAction::Cascade,
            on_update: ReferentialAction::Restrict,
            constraint_name: None,
            metadata: ForeignKeyMetadata::default(),
        });

        junction_table.add_foreign_key(ForeignKey {
            name: Some("fk_user_roles_role".to_string()),
            table_name: "user_roles".to_string(),
            columns: vec!["role_id".to_string()],
            referenced_table: "roles".to_string(),
            referenced_columns: vec!["id".to_string()],
            on_delete: ReferentialAction::Cascade,
            on_update: ReferentialAction::Restrict,
            constraint_name: None,
            metadata: ForeignKeyMetadata::default(),
        });

        // Add composite primary key
        junction_table.set_primary_key(vec!["user_id".to_string(), "role_id".to_string()]);

        database.add_table(junction_table);

        let analysis = JunctionTableDetectionService::analyze_table(database.get_table(None, "user_roles").unwrap(), &database);

        assert!(analysis.is_junction_table);
        assert!(analysis.confidence_score > 0.6);
        assert_eq!(analysis.detected_relationships.len(), 2);
    }

    #[test]
    fn test_junction_table_creation() {
        let mut database = Database::new("test_db");

        let mut junction_table = Table::new("user_posts");
        junction_table.add_column(Column::new("user_id", DataType::Integer { unsigned: false }));
        junction_table.add_column(Column::new("post_id", DataType::Integer { unsigned: false }));

        junction_table.add_foreign_key(ForeignKey {
            name: None,
            table_name: "user_posts".to_string(),
            columns: vec!["user_id".to_string()],
            referenced_table: "users".to_string(),
            referenced_columns: vec!["id".to_string()],
            on_delete: ReferentialAction::Cascade,
            on_update: ReferentialAction::Restrict,
            constraint_name: None,
            metadata: ForeignKeyMetadata::default(),
        });

        database.add_table(junction_table);

        let analysis = JunctionTableDetectionService::analyze_table(database.get_table(None, "user_posts").unwrap(), &database);

        if analysis.is_junction_table {
            let junction = JunctionTableDetectionService::create_junction_table(database.get_table(None, "user_posts").unwrap(), &analysis);
            assert!(junction.is_some());
            let junction = junction.unwrap();
            assert_eq!(junction.table_name, "user_posts");
            assert_eq!(junction.left_table, "users");
        }
    }

    #[test]
    fn test_naming_convention_detection() {
        assert!(JunctionTableDetectionService::follows_junction_naming_convention("user_roles"));
        assert!(JunctionTableDetectionService::follows_junction_naming_convention("posts_to_categories"));
        assert!(JunctionTableDetectionService::follows_junction_naming_convention("student_course_junction"));
        assert!(!JunctionTableDetectionService::follows_junction_naming_convention("users"));
        assert!(!JunctionTableDetectionService::follows_junction_naming_convention("simple_table"));
    }
}
