use std::collections::{HashMap, HashSet};
use serde::{Deserialize, Serialize};

use crate::core::domain::entity::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IndexAnalysis {
    pub index_name: String,
    pub table_name: String,
    pub columns: Vec<IndexColumnAnalysis>,
    pub index_type: IndexType,
    pub is_unique: bool,
    pub is_primary: bool,
    pub coverage_score: f64, // 0.0 to 1.0
    pub redundancy_score: f64, // 0.0 to 1.0
    pub usage_estimate: Option<f64>,
    pub issues: Vec<IndexIssue>,
    pub recommendations: Vec<IndexRecommendation>,
    pub metadata: IndexMetadata,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IndexColumnAnalysis {
    pub column_name: String,
    pub data_type: String,
    pub sort_order: SortOrder,
    pub selectivity_estimate: Option<f64>,
    pub cardinality_estimate: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IndexIssue {
    pub issue_type: IndexIssueType,
    pub severity: IssueSeverity,
    pub description: String,
    pub impact: String,
    pub suggested_fix: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum IndexIssueType {
    RedundantIndex,
    MissingIndex,
    InefficientIndex,
    UnusedIndex,
    LargeIndex,
    PoorSelectivity,
    ConflictingIndex,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IndexRecommendation {
    pub recommendation_type: RecommendationType,
    pub description: String,
    pub benefit: String,
    pub effort: EffortLevel,
    pub sql_commands: Vec<String>,
    pub expected_improvement: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RecommendationType {
    CreateIndex,
    DropIndex,
    ModifyIndex,
    AddIncludedColumns,
    ChangeIndexType,
    ReorderColumns,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum IssueSeverity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EffortLevel {
    Trivial,
    Low,
    Medium,
    High,
    VeryHigh,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IndexMetadata {
    pub description: Option<String>,
    pub created_by: Option<String>,
    pub created_at: Option<String>,
    pub last_used: Option<String>,
    pub usage_count: Option<u64>,
    pub size_estimate: Option<String>,
    pub maintenance_cost: Option<String>,
    pub tags: Vec<String>,
    pub custom_properties: HashMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IndexUsageAnalysis {
    pub table_name: String,
    pub total_indexes: usize,
    pub used_indexes: usize,
    pub unused_indexes: usize,
    pub redundant_indexes: usize,
    pub missing_indexes: Vec<MissingIndexSuggestion>,
    pub index_coverage: f64,
    pub overall_score: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MissingIndexSuggestion {
    pub columns: Vec<String>,
    pub reason: String,
    pub estimated_benefit: f64,
    pub query_patterns: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IndexPerformanceMetrics {
    pub index_name: String,
    pub avg_fragmentation: Option<f64>,
    pub page_space_used: Option<f64>,
    pub avg_page_space_used: Option<f64>,
    pub record_count: Option<u64>,
    pub avg_record_size: Option<u64>,
    pub maintenance_cost: Option<f64>,
    pub usage_stats: Option<IndexUsageStats>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IndexUsageStats {
    pub user_seeks: u64,
    pub user_scans: u64,
    pub user_lookups: u64,
    pub user_updates: u64,
    pub last_user_seek: Option<String>,
    pub last_user_scan: Option<String>,
    pub last_user_lookup: Option<String>,
    pub last_user_update: Option<String>,
}

pub struct IndexValidationService;

impl IndexValidationService {
    pub fn validate_index(index: &Index, table: &Table, database: &Database) -> IndexAnalysis {
        let mut issues = Vec::new();
        let mut recommendations = Vec::new();

        // Validate columns exist
        let mut valid_columns = Vec::new();
        for col in &index.columns {
            if let Some(table_column) = table.get_column(&col.name) {
                valid_columns.push(IndexColumnAnalysis {
                    column_name: col.name.clone(),
                    data_type: table_column.get_data_type_name(),
                    sort_order: col.sort_order.clone(),
                    selectivity_estimate: Self::estimate_selectivity(table_column, table),
                    cardinality_estimate: Some(1000), // Placeholder - would be calculated from actual data
                });
            } else {
                issues.push(IndexIssue {
                    issue_type: IndexIssueType::InefficientIndex,
                    severity: IssueSeverity::High,
                    description: format!("Index column '{}' does not exist in table '{}'", col.name, table.name),
                    impact: "Index cannot be created or used".to_string(),
                    suggested_fix: Some(format!("Remove column '{}' from index or add it to table '{}'", col.name, table.name)),
                });
            }
        }

        // Check for redundant indexes
        let redundancy_score = Self::calculate_redundancy(index, table);
        if redundancy_score > 0.8 {
            issues.push(IndexIssue {
                issue_type: IndexIssueType::RedundantIndex,
                severity: IssueSeverity::Medium,
                description: format!("Index '{}' is redundant with existing indexes", index.name),
                impact: "Consumes storage and maintenance overhead without benefit".to_string(),
                suggested_fix: Some(format!("Consider dropping index '{}' if redundant", index.name)),
            });
        }

        // Check index size and complexity
        if index.columns.len() > 5 {
            issues.push(IndexIssue {
                issue_type: IndexIssueType::LargeIndex,
                severity: IssueSeverity::Low,
                description: format!("Index '{}' has {} columns, which may be inefficient", index.name, index.columns.len()),
                impact: "Large indexes consume more storage and may slow down updates".to_string(),
                suggested_fix: Some("Consider reducing index width or using included columns".to_string()),
            });
        }

        // Check for poor selectivity
        let avg_selectivity = valid_columns.iter()
            .filter_map(|col| col.selectivity_estimate)
            .sum::<f64>() / valid_columns.len() as f64;

        if avg_selectivity < 0.1 && valid_columns.len() == 1 {
            issues.push(IndexIssue {
                issue_type: IndexIssueType::PoorSelectivity,
                severity: IssueSeverity::Medium,
                description: format!("Index '{}' has low selectivity ({:.2}%), may not be effective", index.name, avg_selectivity * 100.0),
                impact: "Queries may not benefit from this index".to_string(),
                suggested_fix: Some("Consider adding more selective columns or removing this index".to_string()),
            });
        }

        // Generate recommendations
        if redundancy_score < 0.3 {
            recommendations.push(IndexRecommendation {
                recommendation_type: RecommendationType::AddIncludedColumns,
                description: format!("Consider adding included columns to index '{}' for better coverage", index.name),
                benefit: "Reduces lookups and improves query performance".to_string(),
                effort: EffortLevel::Medium,
                sql_commands: vec![format!("-- Add included columns to index\n-- Example: ALTER INDEX {} ADD INCLUDE (column1, column2);", index.name)],
                expected_improvement: Some(0.2),
            });
        }

        // Check for missing indexes on foreign keys
        if Self::is_foreign_key_index_missing(index, table) {
            recommendations.push(IndexRecommendation {
                recommendation_type: RecommendationType::CreateIndex,
                description: "Foreign key columns should be indexed for better join performance".to_string(),
                benefit: "Improves join performance and referential integrity checks".to_string(),
                effort: EffortLevel::Low,
                sql_commands: vec![format!("CREATE INDEX idx_{}_{} ON {} ({});",
                    table.name,
                    index.columns.iter().map(|c| c.name.as_str()).collect::<Vec<_>>().join("_"),
                    table.name,
                    index.columns.iter().map(|c| c.name.as_str()).collect::<Vec<_>>().join(", "))],
                expected_improvement: Some(0.5),
            });
        }

        let coverage_score = Self::calculate_coverage(index, table);
        let usage_estimate = Some(0.7); // Placeholder - would be calculated from actual usage stats

        IndexAnalysis {
            index_name: index.name.clone(),
            table_name: table.name.clone(),
            columns: valid_columns,
            index_type: index.index_type.clone(),
            is_unique: index.unique,
            is_primary: false, // Would be determined by checking against primary key
            coverage_score,
            redundancy_score,
            usage_estimate,
            issues,
            recommendations,
            metadata: IndexMetadata {
                description: Some(format!("{} index on table {}", Self::index_type_description(&index.index_type), table.name)),
                created_by: None,
                created_at: None,
                last_used: None,
                usage_count: None,
                size_estimate: Some("~10MB".to_string()), // Placeholder
                maintenance_cost: Some("Low".to_string()), // Placeholder
                tags: vec!["index".to_string(), if index.unique { "unique".to_string() } else { "non-unique".to_string() }],
                custom_properties: HashMap::new(),
            },
        }
    }

    fn estimate_selectivity(column: &Column, table: &Table) -> Option<f64> {
        match &column.data_type {
            DataType::Boolean => Some(0.5), // 2 possible values
            DataType::TinyInt { .. } => Some(0.1), // Assuming 10 distinct values
            DataType::SmallInt { .. } => Some(0.05), // Assuming 20 distinct values
            DataType::Integer { .. } => Some(0.01), // Assuming 100 distinct values
            DataType::BigInt { .. } => Some(0.005), // Assuming 200 distinct values
            DataType::VarChar { .. } | DataType::Text => Some(0.3), // Text fields often have good selectivity
            DataType::Date | DataType::DateTime => Some(0.2), // Date fields have moderate selectivity
            _ => Some(0.1), // Default assumption
        }
    }

    fn calculate_redundancy(index: &Index, table: &Table) -> f64 {
        let mut redundancy_score = 0.0;

        for existing_index in &table.indexes {
            if existing_index.name != index.name {
                let overlap = Self::calculate_column_overlap(index, existing_index);
                redundancy_score = redundancy_score.max(overlap);
            }
        }

        redundancy_score
    }

    fn calculate_column_overlap(index1: &Index, index2: &Index) -> f64 {
        let cols1: HashSet<_> = index1.columns.iter().map(|c| &c.name).collect();
        let cols2: HashSet<_> = index2.columns.iter().map(|c| &c.name).collect();

        let intersection = cols1.intersection(&cols2).count();
        let union = cols1.union(&cols2).count();

        if union == 0 {
            0.0
        } else {
            intersection as f64 / union as f64
        }
    }

    fn calculate_coverage(index: &Index, table: &Table) -> f64 {
        let indexed_columns: HashSet<_> = index.columns.iter().map(|c| &c.name).collect();
        let total_columns = table.columns.len();

        if total_columns == 0 {
            0.0
        } else {
            indexed_columns.len() as f64 / total_columns as f64
        }
    }

    fn is_foreign_key_index_missing(index: &Index, table: &Table) -> bool {
        for fk in &table.foreign_keys {
            let fk_columns: HashSet<_> = fk.columns.iter().collect();
            let index_columns: HashSet<_> = index.columns.iter().map(|c| &c.name).collect();

            if fk_columns == index_columns {
                return false; // Index exists for this FK
            }
        }
        false // Not checking for missing FK indexes in this context
    }

    fn index_type_description(index_type: &IndexType) -> &'static str {
        match index_type {
            IndexType::BTree => "B-Tree",
            IndexType::Hash => "Hash",
            IndexType::Gist => "GiST",
            IndexType::Gin => "GIN",
            IndexType::SpGist => "SP-GiST",
            IndexType::Brin => "BRIN",
            IndexType::Custom(name) => name,
        }
    }
}

pub struct IndexAnalysisService;

impl IndexAnalysisService {
    pub fn analyze_table_indexes(table: &Table, database: &Database) -> IndexUsageAnalysis {
        let total_indexes = table.indexes.len();
        let mut used_indexes = 0;
        let mut unused_indexes = 0;
        let mut redundant_indexes = 0;
        let mut missing_indexes = Vec::new();

        // Analyze existing indexes
        for index in &table.indexes {
            let analysis = IndexValidationService::validate_index(index, table, database);

            if analysis.usage_estimate.unwrap_or(0.0) > 0.1 {
                used_indexes += 1;
            } else {
                unused_indexes += 1;
            }

            if analysis.redundancy_score > 0.8 {
                redundant_indexes += 1;
            }
        }

        // Suggest missing indexes
        missing_indexes.extend(Self::suggest_missing_indexes(table, database));

        let index_coverage = if total_indexes > 0 {
            used_indexes as f64 / total_indexes as f64
        } else {
            0.0
        };

        let overall_score = (index_coverage * 0.6) + ((1.0 - (redundant_indexes as f64 / total_indexes.max(1) as f64)) * 0.4);

        IndexUsageAnalysis {
            table_name: table.name.clone(),
            total_indexes,
            used_indexes,
            unused_indexes,
            redundant_indexes,
            missing_indexes,
            index_coverage,
            overall_score,
        }
    }

    pub fn analyze_database_indexes(database: &Database) -> DatabaseIndexReport {
        let mut table_analyses = Vec::new();
        let mut total_indexes = 0;
        let mut total_used = 0;
        let mut total_unused = 0;
        let mut total_redundant = 0;
        let mut all_missing_indexes = Vec::new();

        for table in database.get_all_tables() {
            let analysis = Self::analyze_table_indexes(table, database);
            table_analyses.push(analysis.clone());

            total_indexes += analysis.total_indexes;
            total_used += analysis.used_indexes;
            total_unused += analysis.unused_indexes;
            total_redundant += analysis.redundant_indexes;
            all_missing_indexes.extend(analysis.missing_indexes);
        }

        let overall_coverage = if total_indexes > 0 {
            total_used as f64 / total_indexes as f64
        } else {
            0.0
        };

        DatabaseIndexReport {
            total_tables: database.get_all_tables().len(),
            total_indexes,
            total_used_indexes: total_used,
            total_unused_indexes: total_unused,
            total_redundant_indexes: total_redundant,
            overall_coverage,
            table_analyses,
            suggested_missing_indexes: all_missing_indexes,
            optimization_recommendations: Self::generate_database_recommendations(&table_analyses),
        }
    }

    fn suggest_missing_indexes(table: &Table, database: &Database) -> Vec<MissingIndexSuggestion> {
        let mut suggestions = Vec::new();

        // Suggest indexes for foreign keys
        for fk in &table.foreign_keys {
            let has_index = table.indexes.iter().any(|idx| {
                let idx_cols: HashSet<_> = idx.columns.iter().map(|c| &c.name).collect();
                let fk_cols: HashSet<_> = fk.columns.iter().collect();
                idx_cols == fk_cols
            });

            if !has_index {
                suggestions.push(MissingIndexSuggestion {
                    columns: fk.columns.clone(),
                    reason: format!("Foreign key to {}.{}", fk.referenced_table, fk.referenced_columns.join(", ")),
                    estimated_benefit: 0.7,
                    query_patterns: vec![
                        format!("JOIN queries between {} and {}", table.name, fk.referenced_table),
                        format!("Foreign key lookups on {}", table.name),
                    ],
                });
            }
        }

        // Suggest indexes for common query patterns (this would be enhanced with actual query analysis)
        let text_columns: Vec<_> = table.columns.iter()
            .filter(|col| matches!(col.data_type, DataType::VarChar { .. } | DataType::Text))
            .map(|col| &col.name)
            .collect();

        if text_columns.len() >= 2 {
            suggestions.push(MissingIndexSuggestion {
                columns: vec![text_columns[0].clone(), text_columns[1].clone()],
                reason: "Common text field combinations in WHERE clauses".to_string(),
                estimated_benefit: 0.4,
                query_patterns: vec![
                    "Text search queries".to_string(),
                    "Filtering by multiple text fields".to_string(),
                ],
            });
        }

        suggestions
    }

    fn generate_database_recommendations(analyses: &[IndexUsageAnalysis]) -> Vec<String> {
        let mut recommendations = Vec::new();

        let total_unused: usize = analyses.iter().map(|a| a.unused_indexes).sum();
        if total_unused > 0 {
            recommendations.push(format!("Consider dropping {} unused indexes to reduce maintenance overhead", total_unused));
        }

        let total_missing: usize = analyses.iter().map(|a| a.missing_indexes.len()).sum();
        if total_missing > 0 {
            recommendations.push(format!("Consider creating {} missing indexes to improve query performance", total_missing));
        }

        let avg_coverage: f64 = analyses.iter().map(|a| a.index_coverage).sum::<f64>() / analyses.len() as f64;
        if avg_coverage < 0.5 {
            recommendations.push("Overall index coverage is low. Consider comprehensive index analysis.".to_string());
        }

        recommendations
    }
}

pub struct IndexOptimizationService;

impl IndexOptimizationService {
    pub fn optimize_index(index: &Index, analysis: &IndexAnalysis, table: &Table) -> Vec<IndexOptimization> {
        let mut optimizations = Vec::new();

        // Column order optimization
        if Self::should_reorder_columns(index) {
            optimizations.push(IndexOptimization {
                optimization_type: IndexOptimizationType::ReorderColumns,
                description: "Consider reordering index columns for better selectivity".to_string(),
                benefit: "Improves index efficiency and query performance".to_string(),
                effort: EffortLevel::Medium,
                sql_commands: vec![
                    format!("-- Drop and recreate index with better column order"),
                    format!("DROP INDEX {};", index.name),
                    format!("CREATE INDEX {} ON {} ({});", index.name, table.name,
                        Self::suggest_column_order(index, table).iter().map(|c| c.as_str()).collect::<Vec<_>>().join(", ")),
                ],
                expected_improvement: Some(0.3),
            });
        }

        // Index type optimization
        if Self::should_change_index_type(index, table) {
            let suggested_type = Self::suggest_index_type(index, table);
            optimizations.push(IndexOptimization {
                optimization_type: IndexOptimizationType::ChangeIndexType,
                description: format!("Consider changing index type from {:?} to {:?}", index.index_type, suggested_type),
                benefit: "Better performance for specific query patterns".to_string(),
                effort: EffortLevel::High,
                sql_commands: vec![
                    format!("-- Recreate index with different type"),
                    format!("DROP INDEX {};", index.name),
                    format!("CREATE INDEX {} ON {} USING {} ({});", index.name, table.name,
                        Self::index_type_name(&suggested_type),
                        index.columns.iter().map(|c| c.name.as_str()).collect::<Vec<_>>().join(", ")),
                ],
                expected_improvement: Some(0.4),
            });
        }

        // Partial index suggestion
        if Self::should_create_partial_index(index, table) {
            optimizations.push(IndexOptimization {
                optimization_type: IndexOptimizationType::CreatePartialIndex,
                description: "Consider creating a partial index for frequently queried subsets".to_string(),
                benefit: "Reduces index size and maintenance cost".to_string(),
                effort: EffortLevel::Medium,
                sql_commands: vec![
                    format!("-- Create partial index"),
                    format!("CREATE INDEX {}_partial ON {} ({}) WHERE active = true;",
                        index.name, table.name,
                        index.columns.iter().map(|c| c.name.as_str()).collect::<Vec<_>>().join(", ")),
                ],
                expected_improvement: Some(0.25),
            });
        }

        // Composite index expansion
        if index.columns.len() == 1 && Self::should_expand_to_composite(index, table) {
            optimizations.push(IndexOptimization {
                optimization_type: IndexOptimizationType::ExpandComposite,
                description: "Consider expanding single-column index to composite for better coverage".to_string(),
                benefit: "Improves query performance for multi-column WHERE clauses".to_string(),
                effort: EffortLevel::Low,
                sql_commands: vec![
                    format!("-- Expand to composite index"),
                    format!("CREATE INDEX {}_composite ON {} ({}, {});",
                        index.name, table.name,
                        index.columns[0].name,
                        Self::suggest_additional_columns(index, table).join(", ")),
                ],
                expected_improvement: Some(0.35),
            });
        }

        optimizations
    }

    fn should_reorder_columns(index: &Index) -> bool {
        // Simple heuristic: reorder if we have more than one column
        index.columns.len() > 1
    }

    fn suggest_column_order(index: &Index, table: &Table) -> Vec<String> {
        // Simple heuristic: order by selectivity (higher first)
        let mut columns_with_selectivity: Vec<_> = index.columns.iter().filter_map(|col| {
            table.get_column(&col.name).map(|table_col| {
                let selectivity = IndexValidationService::estimate_selectivity(table_col, table).unwrap_or(0.1);
                (col.name.clone(), selectivity)
            })
        }).collect();

        columns_with_selectivity.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        columns_with_selectivity.into_iter().map(|(name, _)| name).collect()
    }

    fn should_change_index_type(index: &Index, table: &Table) -> bool {
        // Suggest different index types based on data and usage patterns
        match index.index_type {
            IndexType::BTree => {
                // Suggest GIN for text search, BRIN for sequential data
                table.columns.iter().any(|col| matches!(col.data_type, DataType::Text | DataType::Json))
            }
            _ => false,
        }
    }

    fn suggest_index_type(index: &Index, table: &Table) -> IndexType {
        if table.columns.iter().any(|col| matches!(col.data_type, DataType::Text | DataType::Json)) {
            IndexType::Gin
        } else if table.columns.iter().any(|col| matches!(col.data_type, DataType::Date | DataType::DateTime)) {
            IndexType::Brin
        } else {
            IndexType::BTree
        }
    }

    fn should_create_partial_index(index: &Index, table: &Table) -> bool {
        // Suggest partial indexes if table has status-like columns
        table.columns.iter().any(|col|
            col.name.to_lowercase().contains("active") ||
            col.name.to_lowercase().contains("status") ||
            col.name.to_lowercase().contains("deleted")
        )
    }

    fn should_expand_to_composite(index: &Index, table: &Table) -> bool {
        // Suggest expansion if there are other columns frequently used together
        table.columns.len() > index.columns.len()
    }

    fn suggest_additional_columns(index: &Index, table: &Table) -> Vec<String> {
        // Simple suggestion: add next most common columns
        table.columns.iter()
            .filter(|col| !index.columns.iter().any(|idx_col| idx_col.name == col.name))
            .take(2)
            .map(|col| col.name.clone())
            .collect()
    }

    fn index_type_name(index_type: &IndexType) -> &'static str {
        match index_type {
            IndexType::BTree => "btree",
            IndexType::Hash => "hash",
            IndexType::Gist => "gist",
            IndexType::Gin => "gin",
            IndexType::SpGist => "spgist",
            IndexType::Brin => "brin",
            IndexType::Custom(_) => "custom",
        }
    }
}

pub struct IndexDocumentationService;

impl IndexDocumentationService {
    pub fn generate_index_documentation(analysis: &IndexAnalysis) -> String {
        let mut doc = format!("### Index: {}\n\n", analysis.index_name);

        doc.push_str(&format!("**Table:** {}\n", analysis.table_name));
        doc.push_str(&format!("**Type:** {}\n", Self::index_type_description(&analysis.index_type)));
        doc.push_str(&format!("**Unique:** {}\n", if analysis.is_unique { "Yes" } else { "No" }));

        if let Some(description) = &analysis.metadata.description {
            doc.push_str(&format!("**Description:** {}\n", description));
        }

        doc.push_str(&format!("**Coverage Score:** {:.2}%\n", analysis.coverage_score * 100.0));
        doc.push_str(&format!("**Redundancy Score:** {:.2}%\n", analysis.redundancy_score * 100.0));

        if let Some(usage) = analysis.usage_estimate {
            doc.push_str(&format!("**Usage Estimate:** {:.2}%\n", usage * 100.0));
        }

        doc.push_str("\n#### Columns\n\n");
        doc.push_str("| Column | Data Type | Sort Order | Selectivity |\n");
        doc.push_str("|--------|-----------|------------|-------------|\n");

        for col in &analysis.columns {
            doc.push_str(&format!("| {} | {} | {} | {:.2}% |\n",
                col.column_name,
                col.data_type,
                Self::sort_order_name(&col.sort_order),
                col.selectivity_estimate.unwrap_or(0.0) * 100.0
            ));
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
                doc.push_str(&format!("  - **Impact:** {}\n", issue.impact));

                if let Some(fix) = &issue.suggested_fix {
                    doc.push_str(&format!("  - **Suggested Fix:** {}\n", fix));
                }
            }
        }

        if !analysis.recommendations.is_empty() {
            doc.push_str("\n#### Recommendations\n\n");
            for rec in &analysis.recommendations {
                doc.push_str(&format!("- **{}:** {}\n", Self::recommendation_type_name(&rec.recommendation_type), rec.description));
                doc.push_str(&format!("  - **Benefit:** {}\n", rec.benefit));
                doc.push_str(&format!("  - **Effort:** {}\n", Self::effort_level_name(&rec.effort)));

                if !rec.sql_commands.is_empty() {
                    doc.push_str("  - **SQL Commands:**\n");
                    for cmd in &rec.sql_commands {
                        doc.push_str(&format!("    ```sql\n    {}\n    ```\n", cmd));
                    }
                }
            }
        }

        doc.push_str("\n---\n\n");
        doc
    }

    pub fn generate_index_performance_report(analyses: &[IndexAnalysis]) -> String {
        let mut report = "# Index Performance Report\n\n".to_string();

        let total_indexes = analyses.len();
        let used_indexes = analyses.iter().filter(|a| a.usage_estimate.unwrap_or(0.0) > 0.1).count();
        let redundant_indexes = analyses.iter().filter(|a| a.redundancy_score > 0.8).count();
        let avg_coverage = analyses.iter().map(|a| a.coverage_score).sum::<f64>() / total_indexes as f64;

        report.push_str(&format!("## Summary\n\n"));
        report.push_str(&format!("- **Total Indexes:** {}\n", total_indexes));
        report.push_str(&format!("- **Used Indexes:** {} ({:.1}%)\n", used_indexes, (used_indexes as f64 / total_indexes as f64) * 100.0));
        report.push_str(&format!("- **Redundant Indexes:** {}\n", redundant_indexes));
        report.push_str(&format!("- **Average Coverage:** {:.1}%\n", avg_coverage * 100.0));

        report.push_str("\n## Index Details\n\n");
        for analysis in analyses {
            report.push_str(&Self::generate_index_documentation(analysis));
        }

        report
    }

    fn index_type_description(index_type: &IndexType) -> &'static str {
        match index_type {
            IndexType::BTree => "B-Tree (Balanced Tree)",
            IndexType::Hash => "Hash",
            IndexType::Gist => "GiST (Generalized Search Tree)",
            IndexType::Gin => "GIN (Generalized Inverted Index)",
            IndexType::SpGist => "SP-GiST (Space-Partitioned GiST)",
            IndexType::Brin => "BRIN (Block Range Index)",
            IndexType::Custom(name) => name,
        }
    }

    fn sort_order_name(sort_order: &SortOrder) -> &'static str {
        match sort_order {
            SortOrder::Ascending => "ASC",
            SortOrder::Descending => "DESC",
        }
    }

    fn issue_type_name(issue_type: &IndexIssueType) -> &'static str {
        match issue_type {
            IndexIssueType::RedundantIndex => "Redundant Index",
            IndexIssueType::MissingIndex => "Missing Index",
            IndexIssueType::InefficientIndex => "Inefficient Index",
            IndexIssueType::UnusedIndex => "Unused Index",
            IndexIssueType::LargeIndex => "Large Index",
            IndexIssueType::PoorSelectivity => "Poor Selectivity",
            IndexIssueType::ConflictingIndex => "Conflicting Index",
        }
    }

    fn recommendation_type_name(rec_type: &RecommendationType) -> &'static str {
        match rec_type {
            RecommendationType::CreateIndex => "Create Index",
            RecommendationType::DropIndex => "Drop Index",
            RecommendationType::ModifyIndex => "Modify Index",
            RecommendationType::AddIncludedColumns => "Add Included Columns",
            RecommendationType::ChangeIndexType => "Change Index Type",
            RecommendationType::ReorderColumns => "Reorder Columns",
        }
    }

    fn effort_level_name(effort: &EffortLevel) -> &'static str {
        match effort {
            EffortLevel::Trivial => "Trivial",
            EffortLevel::Low => "Low",
            EffortLevel::Medium => "Medium",
            EffortLevel::High => "High",
            EffortLevel::VeryHigh => "Very High",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IndexOptimization {
    pub optimization_type: IndexOptimizationType,
    pub description: String,
    pub benefit: String,
    pub effort: EffortLevel,
    pub sql_commands: Vec<String>,
    pub expected_improvement: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum IndexOptimizationType {
    ReorderColumns,
    ChangeIndexType,
    CreatePartialIndex,
    ExpandComposite,
    AddIncludedColumns,
    CompressIndex,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DatabaseIndexReport {
    pub total_tables: usize,
    pub total_indexes: usize,
    pub total_used_indexes: usize,
    pub total_unused_indexes: usize,
    pub total_redundant_indexes: usize,
    pub overall_coverage: f64,
    pub table_analyses: Vec<IndexUsageAnalysis>,
    pub suggested_missing_indexes: Vec<MissingIndexSuggestion>,
    pub optimization_recommendations: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index_validation() {
        let mut table = Table::new("users");
        table.add_column(Column::new("id", DataType::Integer { unsigned: false }));
        table.add_column(Column::new("email", DataType::VarChar { length: Some(255) }));
        table.add_column(Column::new("name", DataType::VarChar { length: Some(100) }));

        let index = Index {
            name: "idx_users_email".to_string(),
            table_name: "users".to_string(),
            columns: vec![IndexColumn {
                name: "email".to_string(),
                sort_order: SortOrder::Ascending,
                nulls_position: NullsPosition::Last,
            }],
            unique: false,
            index_type: IndexType::BTree,
            where_clause: None,
            metadata: IndexMetadata::default(),
        };

        table.indexes.push(index.clone());

        let database = Database::new("test_db");
        let analysis = IndexValidationService::validate_index(&index, &table, &database);

        assert_eq!(analysis.index_name, "idx_users_email");
        assert_eq!(analysis.table_name, "users");
        assert_eq!(analysis.columns.len(), 1);
        assert!(analysis.coverage_score > 0.0);
        assert!(analysis.redundancy_score >= 0.0);
    }

    #[test]
    fn test_index_redundancy_calculation() {
        let index1 = Index {
            name: "idx1".to_string(),
            table_name: "test".to_string(),
            columns: vec![IndexColumn {
                name: "col1".to_string(),
                sort_order: SortOrder::Ascending,
                nulls_position: NullsPosition::Last,
            }],
            unique: false,
            index_type: IndexType::BTree,
            where_clause: None,
            metadata: IndexMetadata::default(),
        };

        let index2 = Index {
            name: "idx2".to_string(),
            table_name: "test".to_string(),
            columns: vec![IndexColumn {
                name: "col1".to_string(),
                sort_order: SortOrder::Ascending,
                nulls_position: NullsPosition::Last,
            }, IndexColumn {
                name: "col2".to_string(),
                sort_order: SortOrder::Ascending,
                nulls_position: NullsPosition::Last,
            }],
            unique: false,
            index_type: IndexType::BTree,
            where_clause: None,
            metadata: IndexMetadata::default(),
        };

        let redundancy = IndexValidationService::calculate_column_overlap(&index1, &index2);
        assert_eq!(redundancy, 0.5); // 1 out of 2 columns overlap
    }

    #[test]
    fn test_selectivity_estimation() {
        let table = Table::new("test");
        let bool_column = Column::new("active", DataType::Boolean);
        let int_column = Column::new("user_id", DataType::Integer { unsigned: false });

        let bool_selectivity = IndexValidationService::estimate_selectivity(&bool_column, &table);
        let int_selectivity = IndexValidationService::estimate_selectivity(&int_column, &table);

        assert_eq!(bool_selectivity, Some(0.5));
        assert_eq!(int_selectivity, Some(0.01));
    }
}
