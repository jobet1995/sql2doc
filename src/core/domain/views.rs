use std::collections::{HashMap, HashSet};
use serde::{Deserialize, Serialize};

use crate::core::domain::entity::*;
use crate::core::ast::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ViewAnalysis {
    pub view_name: String,
    pub schema_name: Option<String>,
    pub definition: String,
    pub is_materialized: bool,
    pub dependencies: ViewDependencies,
    pub complexity: ViewComplexity,
    pub performance_characteristics: ViewPerformanceCharacteristics,
    pub issues: Vec<ViewIssue>,
    pub recommendations: Vec<ViewRecommendation>,
    pub metadata: ViewMetadata,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ViewDependencies {
    pub tables_referenced: Vec<String>,
    pub views_referenced: Vec<String>,
    pub functions_used: Vec<String>,
    pub columns_selected: Vec<String>,
    pub dependency_depth: usize,
    pub is_recursive: bool,
    pub circular_dependencies: Vec<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ViewComplexity {
    pub lines_of_code: usize,
    pub nested_subqueries: usize,
    pub join_count: usize,
    pub aggregate_functions: usize,
    pub window_functions: usize,
    pub complexity_score: f64, // 0.0 to 1.0
    pub maintainability_index: f64, // 0.0 to 1.0
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ViewPerformanceCharacteristics {
    pub estimated_row_count: Option<u64>,
    pub estimated_cost: Option<f64>,
    pub indexing_opportunities: Vec<IndexingOpportunity>,
    pub refresh_strategy: Option<MaterializedViewRefreshStrategy>,
    pub caching_recommendations: Vec<CachingRecommendation>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IndexingOpportunity {
    pub table_name: String,
    pub columns: Vec<String>,
    pub opportunity_type: IndexOpportunityType,
    pub estimated_benefit: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum IndexOpportunityType {
    ForeignKeyIndex,
    JoinColumnIndex,
    FilterColumnIndex,
    SortColumnIndex,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MaterializedViewRefreshStrategy {
    OnCommit,
    OnDemand,
    Scheduled(String),
    Never,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CachingRecommendation {
    pub recommendation_type: CacheRecommendationType,
    pub description: String,
    pub cache_duration: Option<String>,
    pub invalidation_triggers: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CacheRecommendationType {
    QueryResultCache,
    MaterializedView,
    ApplicationCache,
    NoCache,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ViewIssue {
    pub issue_type: ViewIssueType,
    pub severity: IssueSeverity,
    pub description: String,
    pub impact: String,
    pub suggested_fix: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ViewIssueType {
    MissingIndexes,
    ComplexJoins,
    NestedSubqueries,
    CircularDependencies,
    PerformanceProblems,
    SecurityConcerns,
    MaintainabilityIssues,
    DataFreshness,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ViewRecommendation {
    pub recommendation_type: ViewRecommendationType,
    pub description: String,
    pub benefit: String,
    pub effort: EffortLevel,
    pub sql_commands: Vec<String>,
    pub expected_improvement: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ViewRecommendationType {
    CreateMaterializedView,
    AddIndexes,
    SimplifyQuery,
    RestructureJoins,
    AddWhereClause,
    CreateIndexedView,
    PartitionBaseTables,
    OptimizeBaseQuery,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ViewMetadata {
    pub description: Option<String>,
    pub created_by: Option<String>,
    pub created_at: Option<String>,
    pub last_modified: Option<String>,
    pub usage_count: Option<u64>,
    pub security_classification: Option<SecurityClassification>,
    pub business_owner: Option<String>,
    pub technical_owner: Option<String>,
    pub tags: Vec<String>,
    pub custom_properties: HashMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SecurityClassification {
    Public,
    Internal,
    Confidential,
    Restricted,
    Classified,
}

pub struct ViewAnalysisService;

impl ViewAnalysisService {
    pub fn analyze_view(view: &View, database: &Database) -> ViewAnalysis {
        let mut issues = Vec::new();
        let mut recommendations = Vec::new();

        // Analyze dependencies
        let dependencies = Self::analyze_dependencies(view, database);

        // Calculate complexity
        let complexity = Self::calculate_complexity(view);

        // Analyze performance characteristics
        let performance_characteristics = Self::analyze_performance(view, database);

        // Check for issues
        issues.extend(Self::analyze_issues(view, &dependencies, &complexity, database));

        // Generate recommendations
        recommendations.extend(Self::generate_recommendations(view, &dependencies, &complexity, &performance_characteristics));

        ViewAnalysis {
            view_name: view.name.clone(),
            schema_name: view.schema_name.clone(),
            definition: view.definition.clone(),
            is_materialized: matches!(view.view_type, ViewType::Materialized),
            dependencies,
            complexity,
            performance_characteristics,
            issues,
            recommendations,
            metadata: ViewMetadata {
                description: view.metadata.description.clone(),
                created_by: view.metadata.created_by.clone(),
                created_at: view.metadata.created_at.clone(),
                last_modified: view.metadata.last_modified.clone(),
                usage_count: view.metadata.usage_count,
                security_classification: view.metadata.security_classification.clone(),
                business_owner: view.metadata.business_owner.clone(),
                technical_owner: view.metadata.technical_owner.clone(),
                tags: view.metadata.tags.clone(),
                custom_properties: view.metadata.custom_properties.clone(),
            },
        }
    }

    pub fn analyze_view_dependencies_across_database(database: &Database) -> ViewDependencyGraph {
        let mut graph = ViewDependencyGraph {
            nodes: HashSet::new(),
            edges: Vec::new(),
            circular_dependencies: Vec::new(),
            dependency_levels: HashMap::new(),
        };

        // Add all views and tables as nodes
        for view in database.get_all_views() {
            graph.nodes.insert(format!("view:{}", view.name));
        }

        for table in database.get_all_tables() {
            graph.nodes.insert(format!("table:{}", table.name));
        }

        // Analyze dependencies for each view
        for view in database.get_all_views() {
            let analysis = Self::analyze_view(view, database);

            for table_ref in &analysis.dependencies.tables_referenced {
                graph.edges.push(ViewDependencyEdge {
                    from: format!("view:{}", view.name),
                    to: format!("table:{}", table_ref),
                    dependency_type: DependencyType::TableReference,
                    strength: DependencyStrength::Strong,
                });
            }

            for view_ref in &analysis.dependencies.views_referenced {
                graph.edges.push(ViewDependencyEdge {
                    from: format!("view:{}", view.name),
                    to: format!("view:{}", view_ref),
                    dependency_type: DependencyType::ViewReference,
                    strength: DependencyStrength::Strong,
                });
            }
        }

        // Detect circular dependencies
        graph.circular_dependencies = Self::detect_circular_dependencies(&graph);

        // Calculate dependency levels
        graph.dependency_levels = Self::calculate_dependency_levels(&graph);

        graph
    }

    fn analyze_dependencies(view: &View, database: &Database) -> ViewDependencies {
        // Parse the view definition to extract dependencies
        // This is a simplified implementation - in practice, you'd use the AST parser
        let definition = view.definition.to_lowercase();

        let mut tables_referenced = Vec::new();
        let mut views_referenced = Vec::new();
        let mut functions_used = Vec::new();

        // Simple pattern matching for table/view references
        // In a real implementation, this would use proper SQL parsing
        for table in database.get_all_tables() {
            if definition.contains(&table.name.to_lowercase()) {
                tables_referenced.push(table.name.clone());
            }
        }

        for other_view in database.get_all_views() {
            if other_view.name != view.name && definition.contains(&other_view.name.to_lowercase()) {
                views_referenced.push(other_view.name.clone());
            }
        }

        // Extract column references (simplified)
        let columns_selected = Self::extract_columns_from_definition(&view.definition);

        // Calculate dependency depth
        let dependency_depth = Self::calculate_dependency_depth(&views_referenced, database);

        // Check for circular dependencies
        let circular_dependencies = Vec::new(); // Would need graph analysis

        ViewDependencies {
            tables_referenced,
            views_referenced,
            functions_used,
            columns_selected,
            dependency_depth,
            is_recursive: definition.contains("recursive"),
            circular_dependencies,
        }
    }

    fn calculate_complexity(view: &View) -> ViewComplexity {
        let definition = view.definition.to_lowercase();
        let lines_of_code = definition.lines().count();

        // Count various complexity indicators
        let nested_subqueries = definition.matches("select").count().saturating_sub(1);
        let join_count = definition.matches("join").count();
        let aggregate_functions = Self::count_aggregate_functions(&definition);
        let window_functions = Self::count_window_functions(&definition);

        // Calculate complexity score (0.0 to 1.0)
        let complexity_score = Self::calculate_complexity_score(
            lines_of_code,
            nested_subqueries,
            join_count,
            aggregate_functions,
            window_functions,
        );

        // Calculate maintainability index
        let maintainability_index = Self::calculate_maintainability_index(
            lines_of_code,
            nested_subqueries,
            join_count,
        );

        ViewComplexity {
            lines_of_code,
            nested_subqueries,
            join_count,
            aggregate_functions,
            window_functions,
            complexity_score,
            maintainability_index,
        }
    }

    fn analyze_performance(view: &View, database: &Database) -> ViewPerformanceCharacteristics {
        let mut indexing_opportunities = Vec::new();

        // Analyze the view definition for indexing opportunities
        let definition = view.definition.to_lowercase();

        // Look for JOIN conditions that could benefit from indexes
        for table in database.get_all_tables() {
            if definition.contains(&table.name.to_lowercase()) {
                // Check for foreign key relationships
                for fk in &table.foreign_keys {
                    if definition.contains(&fk.columns[0].to_lowercase()) {
                        indexing_opportunities.push(IndexingOpportunity {
                            table_name: table.name.clone(),
                            columns: fk.columns.clone(),
                            opportunity_type: IndexOpportunityType::ForeignKeyIndex,
                            estimated_benefit: 0.7,
                        });
                    }
                }
            }
        }

        // Determine refresh strategy for materialized views
        let refresh_strategy = if matches!(view.view_type, ViewType::Materialized) {
            Some(MaterializedViewRefreshStrategy::OnDemand) // Default assumption
        } else {
            None
        };

        // Generate caching recommendations
        let caching_recommendations = Self::generate_caching_recommendations(view);

        ViewPerformanceCharacteristics {
            estimated_row_count: None, // Would need query execution statistics
            estimated_cost: None, // Would need query execution plan analysis
            indexing_opportunities,
            refresh_strategy,
            caching_recommendations,
        }
    }

    fn analyze_issues(view: &View, dependencies: &ViewDependencies, complexity: &ViewComplexity, database: &Database) -> Vec<ViewIssue> {
        let mut issues = Vec::new();

        // Check for high complexity
        if complexity.complexity_score > 0.8 {
            issues.push(ViewIssue {
                issue_type: ViewIssueType::MaintainabilityIssues,
                severity: IssueSeverity::Medium,
                description: format!("View '{}' has high complexity score ({:.2})", view.name, complexity.complexity_score),
                impact: "Difficult to maintain and optimize",
                suggested_fix: Some("Consider breaking down into smaller, simpler views".to_string()),
            });
        }

        // Check for deep dependency chains
        if dependencies.dependency_depth > 3 {
            issues.push(ViewIssue {
                issue_type: ViewIssueType::CircularDependencies,
                severity: IssueSeverity::High,
                description: format!("View '{}' has deep dependency chain (depth: {})", view.name, dependencies.dependency_depth),
                impact: "Performance degradation and maintenance complexity",
                suggested_fix: Some("Consider flattening the view hierarchy".to_string()),
            });
        }

        // Check for missing indexes
        if !dependencies.tables_referenced.is_empty() {
            issues.push(ViewIssue {
                issue_type: ViewIssueType::MissingIndexes,
                severity: IssueSeverity::Medium,
                description: format!("Tables referenced by view '{}' may benefit from additional indexes", view.name),
                impact: "Poor query performance",
                suggested_fix: Some("Analyze query execution plans and add appropriate indexes".to_string()),
            });
        }

        // Check for materialized view refresh concerns
        if matches!(view.view_type, ViewType::Materialized) && dependencies.views_referenced.len() > 0 {
            issues.push(ViewIssue {
                issue_type: ViewIssueType::DataFreshness,
                severity: IssueSeverity::Medium,
                description: "Materialized view references other views, which may cause refresh issues".to_string(),
                impact: "Data may become stale or refresh may fail",
                suggested_fix: Some("Consider restructuring to reference base tables directly".to_string()),
            });
        }

        issues
    }

    fn generate_recommendations(
        view: &View,
        dependencies: &ViewDependencies,
        complexity: &ViewComplexity,
        performance: &ViewPerformanceCharacteristics,
    ) -> Vec<ViewRecommendation> {
        let mut recommendations = Vec::new();

        // Recommend materialization for complex views
        if !matches!(view.view_type, ViewType::Materialized) && complexity.complexity_score > 0.7 {
            recommendations.push(ViewRecommendation {
                recommendation_type: ViewRecommendationType::CreateMaterializedView,
                description: "Consider creating a materialized view for better performance".to_string(),
                benefit: "Faster query execution at the cost of storage and maintenance".to_string(),
                effort: EffortLevel::Medium,
                sql_commands: vec![
                    format!("-- Create materialized view"),
                    format!("CREATE MATERIALIZED VIEW {}_materialized AS", view.name),
                    view.definition.clone(),
                    format!("-- Create refresh function"),
                    format!("CREATE OR REPLACE FUNCTION refresh_{}() RETURNS void AS $$", view.name),
                    format!("BEGIN"),
                    format!("  REFRESH MATERIALIZED VIEW {};", view.name),
                    format!("END;"),
                    format!("$$ LANGUAGE plpgsql;"),
                ],
                expected_improvement: Some(0.8),
            });
        }

        // Recommend index creation
        if !performance.indexing_opportunities.is_empty() {
            let opportunity = &performance.indexing_opportunities[0];
            recommendations.push(ViewRecommendation {
                recommendation_type: ViewRecommendationType::AddIndexes,
                description: format!("Add indexes on {} for better join performance", opportunity.columns.join(", ")),
                benefit: "Improved query execution time".to_string(),
                effort: EffortLevel::Low,
                sql_commands: vec![
                    format!("CREATE INDEX idx_{}_{} ON {} ({});",
                        opportunity.table_name,
                        opportunity.columns.join("_"),
                        opportunity.table_name,
                        opportunity.columns.join(", ")),
                ],
                expected_improvement: Some(0.6),
            });
        }

        // Recommend query simplification
        if complexity.nested_subqueries > 2 {
            recommendations.push(ViewRecommendation {
                recommendation_type: ViewRecommendationType::SimplifyQuery,
                description: "Simplify view by reducing nested subqueries".to_string(),
                benefit: "Better performance and maintainability".to_string(),
                effort: EffortLevel::High,
                sql_commands: vec![
                    "-- Consider rewriting subqueries as JOINs or CTEs".to_string(),
                    "-- Example: Replace subquery with JOIN".to_string(),
                ],
                expected_improvement: Some(0.4),
            });
        }

        recommendations
    }

    fn count_aggregate_functions(definition: &str) -> usize {
        let aggregates = ["count(", "sum(", "avg(", "min(", "max(", "stddev(", "variance("];
        aggregates.iter()
            .map(|agg| definition.matches(agg).count())
            .sum()
    }

    fn count_window_functions(definition: &str) -> usize {
        let windows = ["row_number(", "rank(", "dense_rank(", "percent_rank(", "cume_dist(", "ntile(", "lag(", "lead(", "first_value(", "last_value("];
        windows.iter()
            .map(|win| definition.matches(win).count())
            .sum()
    }

    fn calculate_complexity_score(lines: usize, subqueries: usize, joins: usize, aggregates: usize, windows: usize) -> f64 {
        let base_score = (lines as f64 / 50.0).min(1.0) * 0.3;
        let subquery_score = (subqueries as f64 / 5.0).min(1.0) * 0.25;
        let join_score = (joins as f64 / 10.0).min(1.0) * 0.2;
        let aggregate_score = (aggregates as f64 / 5.0).min(1.0) * 0.15;
        let window_score = (windows as f64 / 3.0).min(1.0) * 0.1;

        (base_score + subquery_score + join_score + aggregate_score + window_score).min(1.0)
    }

    fn calculate_maintainability_index(lines: usize, subqueries: usize, joins: usize) -> f64 {
        let complexity_penalty = (subqueries as f64 * 0.2) + (joins as f64 * 0.1);
        let size_penalty = (lines as f64 / 100.0).min(1.0) * 0.3;

        (1.0 - complexity_penalty - size_penalty).max(0.0)
    }

    fn extract_columns_from_definition(definition: &str) -> Vec<String> {
        // Simple column extraction - in practice, this would use AST parsing
        use regex::Regex;
        let column_regex = Regex::new(r"\b([a-zA-Z_][a-zA-Z0-9_]*)\b").unwrap();

        column_regex.captures_iter(definition)
            .filter_map(|cap| cap.get(1))
            .map(|m| m.as_str().to_string())
            .filter(|col| !Self::is_sql_keyword(col))
            .collect::<HashSet<String>>()
            .into_iter()
            .collect()
    }

    fn is_sql_keyword(word: &str) -> bool {
        let keywords = ["select", "from", "where", "join", "group", "order", "by", "having", "insert", "update", "delete"];
        keywords.contains(&word.to_lowercase().as_str())
    }

    fn calculate_dependency_depth(view_refs: &[String], database: &Database) -> usize {
        let mut max_depth = 0;

        for view_ref in view_refs {
            if let Some(view) = database.get_view(None, view_ref) {
                let analysis = Self::analyze_view(view, database);
                max_depth = max_depth.max(analysis.dependencies.dependency_depth + 1);
            }
        }

        max_depth
    }

    fn generate_caching_recommendations(view: &View) -> Vec<CachingRecommendation> {
        let mut recommendations = Vec::new();

        // For frequently accessed views
        if view.metadata.usage_count.unwrap_or(0) > 1000 {
            recommendations.push(CachingRecommendation {
                recommendation_type: CacheRecommendationType::QueryResultCache,
                description: "Implement query result caching for frequently accessed view".to_string(),
                cache_duration: Some("15 minutes".to_string()),
                invalidation_triggers: vec![
                    "Base table updates".to_string(),
                    "Scheduled refresh".to_string(),
                ],
            });
        }

        // For complex views
        if view.definition.lines().count() > 20 {
            recommendations.push(CachingRecommendation {
                recommendation_type: CacheRecommendationType::MaterializedView,
                description: "Consider materialized view for complex query caching".to_string(),
                cache_duration: Some("1 hour".to_string()),
                invalidation_triggers: vec![
                    "Base table changes".to_string(),
                    "Manual refresh".to_string(),
                ],
            });
        }

        recommendations
    }

    fn detect_circular_dependencies(graph: &ViewDependencyGraph) -> Vec<Vec<String>> {
        // Simple cycle detection - in practice, this would use proper graph algorithms
        let mut cycles = Vec::new();

        // This is a placeholder implementation
        // A real implementation would use DFS or topological sort

        cycles
    }

    fn calculate_dependency_levels(graph: &ViewDependencyGraph) -> HashMap<String, usize> {
        let mut levels = HashMap::new();

        // Calculate dependency levels using topological sort
        // This is a simplified implementation

        for node in &graph.nodes {
            levels.insert(node.clone(), 0);
        }

        levels
    }
}

pub struct ViewDocumentationService;

impl ViewDocumentationService {
    pub fn generate_view_documentation(analysis: &ViewAnalysis) -> String {
        let mut doc = format!("### View: {}\n\n", analysis.view_name);

        if let Some(schema) = &analysis.schema_name {
            doc.push_str(&format!("**Schema:** {}\n", schema));
        }

        doc.push_str(&format!("**Type:** {}\n", if analysis.is_materialized { "Materialized View" } else { "Regular View" }));

        if let Some(description) = &analysis.metadata.description {
            doc.push_str(&format!("**Description:** {}\n", description));
        }

        doc.push_str(&format!("**Complexity Score:** {:.2}%\n", analysis.complexity.complexity_score * 100.0));
        doc.push_str(&format!("**Maintainability Index:** {:.2}%\n", analysis.complexity.maintainability_index * 100.0));

        doc.push_str("\n#### Dependencies\n\n");
        doc.push_str(&format!("- **Tables Referenced:** {}\n", analysis.dependencies.tables_referenced.join(", ")));
        doc.push_str(&format!("- **Views Referenced:** {}\n", analysis.dependencies.views_referenced.join(", ")));
        doc.push_str(&format!("- **Dependency Depth:** {}\n", analysis.dependencies.dependency_depth));
        doc.push_str(&format!("- **Recursive:** {}\n", if analysis.dependencies.is_recursive { "Yes" } else { "No" }));

        doc.push_str("\n#### Complexity Metrics\n\n");
        doc.push_str(&format!("- **Lines of Code:** {}\n", analysis.complexity.lines_of_code));
        doc.push_str(&format!("- **Nested Subqueries:** {}\n", analysis.complexity.nested_subqueries));
        doc.push_str(&format!("- **Join Count:** {}\n", analysis.complexity.join_count));
        doc.push_str(&format!("- **Aggregate Functions:** {}\n", analysis.complexity.aggregate_functions));
        doc.push_str(&format!("- **Window Functions:** {}\n", analysis.complexity.window_functions));

        if !analysis.performance_characteristics.indexing_opportunities.is_empty() {
            doc.push_str("\n#### Indexing Opportunities\n\n");
            for opportunity in &analysis.performance_characteristics.indexing_opportunities {
                doc.push_str(&format!("- **{}:** {} ({:.1}% benefit)\n",
                    opportunity.table_name,
                    opportunity.columns.join(", "),
                    opportunity.estimated_benefit * 100.0));
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

        doc.push_str("\n#### Definition\n\n");
        doc.push_str("```sql\n");
        doc.push_str(&analysis.definition);
        doc.push_str("\n```\n\n");

        doc.push_str("---\n\n");
        doc
    }

    fn issue_type_name(issue_type: &ViewIssueType) -> &'static str {
        match issue_type {
            ViewIssueType::MissingIndexes => "Missing Indexes",
            ViewIssueType::ComplexJoins => "Complex Joins",
            ViewIssueType::NestedSubqueries => "Nested Subqueries",
            ViewIssueType::CircularDependencies => "Circular Dependencies",
            ViewIssueType::PerformanceProblems => "Performance Problems",
            ViewIssueType::SecurityConcerns => "Security Concerns",
            ViewIssueType::MaintainabilityIssues => "Maintainability Issues",
            ViewIssueType::DataFreshness => "Data Freshness",
        }
    }

    fn recommendation_type_name(rec_type: &ViewRecommendationType) -> &'static str {
        match rec_type {
            ViewRecommendationType::CreateMaterializedView => "Create Materialized View",
            ViewRecommendationType::AddIndexes => "Add Indexes",
            ViewRecommendationType::SimplifyQuery => "Simplify Query",
            ViewRecommendationType::RestructureJoins => "Restructure Joins",
            ViewRecommendationType::AddWhereClause => "Add Where Clause",
            ViewRecommendationType::CreateIndexedView => "Create Indexed View",
            ViewRecommendationType::PartitionBaseTables => "Partition Base Tables",
            ViewRecommendationType::OptimizeBaseQuery => "Optimize Base Query",
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
pub struct ViewDependencyGraph {
    pub nodes: HashSet<String>,
    pub edges: Vec<ViewDependencyEdge>,
    pub circular_dependencies: Vec<Vec<String>>,
    pub dependency_levels: HashMap<String, usize>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ViewDependencyEdge {
    pub from: String,
    pub to: String,
    pub dependency_type: DependencyType,
    pub strength: DependencyStrength,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DependencyType {
    TableReference,
    ViewReference,
    FunctionCall,
    ColumnReference,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DependencyStrength {
    Weak,
    Moderate,
    Strong,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_view_complexity_calculation() {
        let view = View {
            name: "complex_view".to_string(),
            schema_name: Some("public".to_string()),
            definition: r#"
                SELECT u.name, COUNT(o.id) as order_count, AVG(o.total) as avg_order
                FROM users u
                LEFT JOIN (
                    SELECT user_id, id, total
                    FROM orders
                    WHERE status = 'completed'
                ) o ON u.id = o.user_id
                GROUP BY u.id, u.name
                HAVING COUNT(o.id) > 5
            "#.to_string(),
            view_type: ViewType::Regular,
            metadata: ViewMetadata::default(),
        };

        let complexity = ViewAnalysisService::calculate_complexity(&view);

        assert_eq!(complexity.lines_of_code, 10);
        assert_eq!(complexity.join_count, 1);
        assert!(complexity.complexity_score > 0.0);
        assert!(complexity.maintainability_index > 0.0);
    }

    #[test]
    fn test_column_extraction() {
        let definition = "SELECT name, email, COUNT(*) FROM users WHERE active = true";
        let columns = ViewAnalysisService::extract_columns_from_definition(definition);

        assert!(columns.contains(&"name".to_string()));
        assert!(columns.contains(&"email".to_string()));
        assert!(columns.contains(&"active".to_string()));
        assert!(!columns.contains(&"select".to_string()));
        assert!(!columns.contains(&"from".to_string()));
    }

    #[test]
    fn test_aggregate_function_counting() {
        let definition = "SELECT COUNT(*), SUM(amount), AVG(price), MAX(date) FROM orders";
        let count = ViewAnalysisService::count_aggregate_functions(definition);

        assert_eq!(count, 4);
    }
}
