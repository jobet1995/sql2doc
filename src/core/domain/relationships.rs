use std::collections::{HashMap, HashSet, VecDeque};
use serde::{Deserialize, Serialize};

use crate::core::domain::entity::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RelationshipValidationResult {
    pub relationship: Relationship,
    pub is_valid: bool,
    pub errors: Vec<RelationshipValidationError>,
    pub warnings: Vec<RelationshipValidationWarning>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RelationshipValidationError {
    pub rule: String,
    pub message: String,
    pub severity: ValidationSeverity,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RelationshipValidationWarning {
    pub rule: String,
    pub message: String,
    pub suggestion: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Relationship {
    pub id: String,
    pub source_table: String,
    pub target_table: String,
    pub source_columns: Vec<String>,
    pub target_columns: Vec<String>,
    pub relationship_type: RelationshipType,
    pub cardinality: Cardinality,
    pub constraints: Vec<RelationshipConstraint>,
    pub metadata: RelationshipMetadata,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RelationshipType {
    OneToOne,
    OneToMany,
    ManyToOne,
    ManyToMany,
    SelfReferencing,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Cardinality {
    pub source_min: u32,
    pub source_max: Option<u32>, // None means many
    pub target_min: u32,
    pub target_max: Option<u32>, // None means many
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RelationshipConstraint {
    ForeignKey {
        name: Option<String>,
        on_delete: ReferentialAction,
        on_update: ReferentialAction,
    },
    UniqueConstraint,
    CheckConstraint(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RelationshipMetadata {
    pub description: Option<String>,
    pub business_rule: Option<String>,
    pub is_required: bool,
    pub is_identifying: bool,
    pub tags: Vec<String>,
    pub custom_properties: HashMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RelationshipGraph {
    pub nodes: HashSet<String>,
    pub edges: Vec<Relationship>,
    pub adjacency_list: HashMap<String, Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DependencyAnalysis {
    pub table_name: String,
    pub depends_on: Vec<String>,
    pub depended_by: Vec<String>,
    pub dependency_depth: usize,
    pub has_circular_dependencies: bool,
    pub circular_paths: Vec<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RelationshipSuggestion {
    pub table_name: String,
    pub relationship_type: String,
    pub suggestion: String,
    pub impact: SuggestionImpact,
    pub sql_commands: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SuggestionImpact {
    Low,
    Medium,
    High,
    Critical,
}

pub struct RelationshipValidationService;

impl RelationshipValidationService {
    pub fn validate_relationship(relationship: &Relationship, database: &Database) -> RelationshipValidationResult {
        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        // Check if source table exists
        if database.get_table(None, &relationship.source_table).is_none() {
            errors.push(RelationshipValidationError {
                rule: "source_table_exists".to_string(),
                message: format!("Source table '{}' does not exist", relationship.source_table),
                severity: ValidationSeverity::Error,
            });
        }

        // Check if target table exists
        if database.get_table(None, &relationship.target_table).is_none() {
            errors.push(RelationshipValidationError {
                rule: "target_table_exists".to_string(),
                message: format!("Target table '{}' does not exist", relationship.target_table),
                severity: ValidationSeverity::Error,
            });
        }

        // Check if source columns exist and are valid
        if let Some(source_table) = database.get_table(None, &relationship.source_table) {
            for col_name in &relationship.source_columns {
                if let Some(column) = source_table.get_column(col_name) {
                    // Check if column is nullable for required relationships
                    if relationship.metadata.is_required && column.nullable {
                        warnings.push(RelationshipValidationWarning {
                            rule: "required_relationship_nullable".to_string(),
                            message: format!("Required relationship has nullable column '{}'", col_name),
                            suggestion: Some(format!("Consider making column '{}' NOT NULL", col_name)),
                        });
                    }
                } else {
                    errors.push(RelationshipValidationError {
                        rule: "source_column_exists".to_string(),
                        message: format!("Source column '{}' does not exist in table '{}'", col_name, relationship.source_table),
                        severity: ValidationSeverity::Error,
                    });
                }
            }
        }

        // Check if target columns exist
        if let Some(target_table) = database.get_table(None, &relationship.target_table) {
            for col_name in &relationship.target_columns {
                if target_table.get_column(col_name).is_none() {
                    errors.push(RelationshipValidationError {
                        rule: "target_column_exists".to_string(),
                        message: format!("Target column '{}' does not exist in table '{}'", col_name, relationship.target_table),
                        severity: ValidationSeverity::Error,
                    });
                }
            }
        }

        // Check cardinality consistency
        match relationship.relationship_type {
            RelationshipType::OneToOne => {
                if relationship.cardinality.source_max != Some(1) || relationship.cardinality.target_max != Some(1) {
                    warnings.push(RelationshipValidationWarning {
                        rule: "one_to_one_cardinality".to_string(),
                        message: "One-to-one relationship should have max cardinality of 1 on both sides".to_string(),
                        suggestion: Some("Consider adding unique constraints to enforce one-to-one relationship".to_string()),
                    });
                }
            }
            RelationshipType::OneToMany => {
                if relationship.cardinality.source_max != Some(1) {
                    errors.push(RelationshipValidationError {
                        rule: "one_to_many_cardinality".to_string(),
                        message: "One-to-many relationship must have max cardinality of 1 on the 'one' side".to_string(),
                        severity: ValidationSeverity::Error,
                    });
                }
            }
            RelationshipType::ManyToOne => {
                if relationship.cardinality.target_max != Some(1) {
                    errors.push(RelationshipValidationError {
                        rule: "many_to_one_cardinality".to_string(),
                        message: "Many-to-one relationship must have max cardinality of 1 on the 'one' side".to_string(),
                        severity: ValidationSeverity::Error,
                    });
                }
            }
            RelationshipType::ManyToMany => {
                // Many-to-many relationships typically require a junction table
                // This would be validated separately
            }
            RelationshipType::SelfReferencing => {
                if relationship.source_table != relationship.target_table {
                    errors.push(RelationshipValidationError {
                        rule: "self_referencing_consistency".to_string(),
                        message: "Self-referencing relationship must have the same source and target table".to_string(),
                        severity: ValidationSeverity::Error,
                    });
                }
            }
        }

        // Check for circular dependencies
        let graph = RelationshipGraph::from_database(database);
        if let Some(cycles) = graph.detect_cycles() {
            if cycles.iter().any(|cycle| cycle.contains(&relationship.source_table) && cycle.contains(&relationship.target_table)) {
                warnings.push(RelationshipValidationWarning {
                    rule: "circular_dependency".to_string(),
                    message: format!("Relationship creates circular dependency: {} -> {}", relationship.source_table, relationship.target_table),
                    suggestion: Some("Consider restructuring the schema to avoid circular dependencies".to_string()),
                });
            }
        }

        RelationshipValidationResult {
            relationship: relationship.clone(),
            is_valid: errors.is_empty(),
            errors,
            warnings,
        }
    }

    pub fn validate_foreign_key_constraint(fk: &ForeignKey, database: &Database) -> RelationshipValidationResult {
        // Convert ForeignKey to Relationship for validation
        let relationship = Relationship {
            id: fk.name.clone().unwrap_or_else(|| format!("fk_{}_{}", fk.table_name, fk.referenced_table)),
            source_table: fk.table_name.clone(),
            target_table: fk.referenced_table.clone(),
            source_columns: fk.columns.clone(),
            target_columns: fk.referenced_columns.clone(),
            relationship_type: RelationshipType::ManyToOne, // Default assumption
            cardinality: Cardinality {
                source_min: 0,
                source_max: None,
                target_min: 1,
                target_max: Some(1),
            },
            constraints: vec![RelationshipConstraint::ForeignKey {
                name: fk.name.clone(),
                on_delete: fk.on_delete.clone(),
                on_update: fk.on_update.clone(),
            }],
            metadata: RelationshipMetadata {
                description: fk.metadata.description.clone(),
                business_rule: None,
                is_required: false,
                is_identifying: false,
                tags: fk.metadata.custom_properties.keys().cloned().collect(),
                custom_properties: fk.metadata.custom_properties.clone(),
            },
        };

        Self::validate_relationship(&relationship, database)
    }
}

pub struct RelationshipAnalysisService;

impl RelationshipAnalysisService {
    pub fn analyze_table_relationships(table: &Table, database: &Database) -> TableRelationshipAnalysis {
        let mut incoming_relationships = Vec::new();
        let mut outgoing_relationships = Vec::new();
        let mut self_references = Vec::new();

        // Find incoming relationships (tables that reference this table)
        for other_table in database.get_all_tables() {
            for fk in &other_table.foreign_keys {
                if fk.referenced_table == table.name {
                    incoming_relationships.push(AnalyzedRelationship {
                        related_table: other_table.name.clone(),
                        relationship_type: Self::infer_relationship_type(fk, other_table, table),
                        columns: fk.columns.clone(),
                        referenced_columns: fk.referenced_columns.clone(),
                        is_nullable: fk.columns.iter().any(|col_name|
                            other_table.get_column(col_name).map_or(false, |col| col.nullable)
                        ),
                        constraint_name: fk.name.clone(),
                    });
                }
            }
        }

        // Find outgoing relationships (tables this table references)
        for fk in &table.foreign_keys {
            if fk.referenced_table == table.name {
                // Self-reference
                self_references.push(AnalyzedRelationship {
                    related_table: table.name.clone(),
                    relationship_type: RelationshipType::SelfReferencing,
                    columns: fk.columns.clone(),
                    referenced_columns: fk.referenced_columns.clone(),
                    is_nullable: fk.columns.iter().any(|col_name|
                        table.get_column(col_name).map_or(false, |col| col.nullable)
                    ),
                    constraint_name: fk.name.clone(),
                });
            } else {
                outgoing_relationships.push(AnalyzedRelationship {
                    related_table: fk.referenced_table.clone(),
                    relationship_type: Self::infer_relationship_type(fk, table, database.get_table(None, &fk.referenced_table).unwrap()),
                    columns: fk.columns.clone(),
                    referenced_columns: fk.referenced_columns.clone(),
                    is_nullable: fk.columns.iter().any(|col_name|
                        table.get_column(col_name).map_or(false, |col| col.nullable)
                    ),
                    constraint_name: fk.name.clone(),
                });
            }
        }

        let total_relationships = incoming_relationships.len() + outgoing_relationships.len() + self_references.len();
        let has_circular_refs = !self_references.is_empty();

        TableRelationshipAnalysis {
            table_name: table.name.clone(),
            incoming_relationships,
            outgoing_relationships,
            self_references,
            total_relationships,
            has_circular_refs,
            isolation_level: Self::determine_isolation_level(&incoming_relationships, &outgoing_relationships),
        }
    }

    pub fn detect_relationship_patterns(database: &Database) -> Vec<RelationshipPattern> {
        let mut patterns = Vec::new();
        let graph = RelationshipGraph::from_database(database);

        // Detect star schema patterns
        if let Some(star_center) = Self::find_star_schema_center(&graph) {
            patterns.push(RelationshipPattern {
                pattern_type: PatternType::StarSchema,
                description: format!("Star schema centered around table '{}'", star_center),
                involved_tables: graph.get_connected_tables(&star_center),
                confidence: 0.8,
            });
        }

        // Detect snowflake schema patterns
        if Self::is_snowflake_schema(&graph) {
            patterns.push(RelationshipPattern {
                pattern_type: PatternType::SnowflakeSchema,
                description: "Snowflake schema detected with normalized dimension tables".to_string(),
                involved_tables: graph.nodes.iter().cloned().collect(),
                confidence: 0.7,
            });
        }

        // Detect master-detail patterns
        let master_detail_patterns = Self::find_master_detail_patterns(database);
        patterns.extend(master_detail_patterns);

        patterns
    }

    fn infer_relationship_type(fk: &ForeignKey, source_table: &Table, target_table: &Table) -> RelationshipType {
        // Check if the foreign key columns are unique in the source table
        let fk_columns_unique = fk.columns.iter().all(|col_name| {
            source_table.get_column(col_name)
                .map_or(false, |col| col.is_unique(&source_table.unique_constraints))
        });

        // Check if the referenced columns are unique in the target table
        let ref_columns_unique = fk.referenced_columns.iter().all(|col_name| {
            target_table.get_column(col_name)
                .map_or(false, |col| col.is_unique(&target_table.unique_constraints) ||
                                  col.is_primary_key(&target_table.primary_key))
        });

        match (fk_columns_unique, ref_columns_unique) {
            (true, true) => RelationshipType::OneToOne,
            (false, true) => RelationshipType::ManyToOne,
            (true, false) => RelationshipType::OneToMany,
            (false, false) => RelationshipType::ManyToMany,
        }
    }

    fn determine_isolation_level(incoming: &[AnalyzedRelationship], outgoing: &[AnalyzedRelationship]) -> IsolationLevel {
        let total_relationships = incoming.len() + outgoing.len();

        if total_relationships == 0 {
            IsolationLevel::Isolated
        } else if total_relationships <= 2 {
            IsolationLevel::Low
        } else if total_relationships <= 5 {
            IsolationLevel::Medium
        } else {
            IsolationLevel::High
        }
    }

    fn find_star_schema_center(graph: &RelationshipGraph) -> Option<String> {
        for node in &graph.nodes {
            let outgoing_count = graph.adjacency_list.get(node).map_or(0, |adj| adj.len());
            let incoming_count: usize = graph.edges.iter()
                .filter(|edge| edge.target_table == *node)
                .count();

            // Star center has many outgoing relationships and few incoming
            if outgoing_count >= 3 && incoming_count <= 1 {
                return Some(node.clone());
            }
        }
        None
    }

    fn is_snowflake_schema(graph: &RelationshipGraph) -> bool {
        // Snowflake schemas have chains of relationships (normalization)
        let mut has_chains = false;
        for node in &graph.nodes {
            if let Some(adjacent) = graph.adjacency_list.get(node) {
                if adjacent.len() >= 2 {
                    has_chains = true;
                    break;
                }
            }
        }
        has_chains && graph.edges.len() > graph.nodes.len()
    }

    fn find_master_detail_patterns(database: &Database) -> Vec<RelationshipPattern> {
        let mut patterns = Vec::new();

        for table in database.get_all_tables() {
            // Look for tables with foreign keys to a master table
            let foreign_keys: Vec<_> = table.foreign_keys.iter()
                .filter(|fk| fk.referenced_table != table.name)
                .collect();

            if !foreign_keys.is_empty() {
                let master_tables: HashSet<_> = foreign_keys.iter()
                    .map(|fk| fk.referenced_table.as_str())
                    .collect();

                if master_tables.len() == 1 {
                    let master_table = master_tables.into_iter().next().unwrap();
                    patterns.push(RelationshipPattern {
                        pattern_type: PatternType::MasterDetail,
                        description: format!("{} is a detail table for master table {}", table.name, master_table),
                        involved_tables: vec![table.name.clone(), master_table.to_string()],
                        confidence: 0.9,
                    });
                }
            }
        }

        patterns
    }
}

pub struct DependencyMappingService;

impl DependencyMappingService {
    pub fn analyze_dependencies(database: &Database) -> Vec<DependencyAnalysis> {
        let mut analyses = Vec::new();
        let graph = RelationshipGraph::from_database(database);

        for table_name in &graph.nodes {
            let depends_on: Vec<String> = graph.edges.iter()
                .filter(|edge| edge.source_table == *table_name)
                .map(|edge| edge.target_table.clone())
                .collect();

            let depended_by: Vec<String> = graph.edges.iter()
                .filter(|edge| edge.target_table == *table_name)
                .map(|edge| edge.source_table.clone())
                .collect();

            let dependency_depth = Self::calculate_dependency_depth(&graph, table_name);
            let circular_paths = graph.find_circular_paths(table_name);

            analyses.push(DependencyAnalysis {
                table_name: table_name.clone(),
                depends_on,
                depended_by,
                dependency_depth,
                has_circular_dependencies: !circular_paths.is_empty(),
                circular_paths,
            });
        }

        analyses
    }

    pub fn generate_dependency_report(database: &Database) -> DependencyReport {
        let analyses = Self::analyze_dependencies(database);

        let tables_with_circular_deps: Vec<_> = analyses.iter()
            .filter(|analysis| analysis.has_circular_dependencies)
            .map(|analysis| analysis.table_name.clone())
            .collect();

        let max_depth = analyses.iter()
            .map(|analysis| analysis.dependency_depth)
            .max()
            .unwrap_or(0);

        let isolated_tables: Vec<_> = analyses.iter()
            .filter(|analysis| analysis.depends_on.is_empty() && analysis.depended_by.is_empty())
            .map(|analysis| analysis.table_name.clone())
            .collect();

        DependencyReport {
            total_tables: analyses.len(),
            tables_with_circular_dependencies: tables_with_circular_deps,
            max_dependency_depth: max_depth,
            isolated_tables,
            dependency_chains: Self::find_longest_dependency_chains(&analyses),
        }
    }

    fn calculate_dependency_depth(graph: &RelationshipGraph, table: &str) -> usize {
        let mut visited = HashSet::new();
        let mut max_depth = 0;

        Self::dfs_depth(graph, table, &mut visited, 0, &mut max_depth);

        max_depth
    }

    fn dfs_depth(graph: &RelationshipGraph, current: &str, visited: &mut HashSet<String>, depth: usize, max_depth: &mut usize) {
        if visited.contains(current) {
            return;
        }

        visited.insert(current.to_string());
        *max_depth = (*max_depth).max(depth);

        if let Some(adjacent) = graph.adjacency_list.get(current) {
            for neighbor in adjacent {
                Self::dfs_depth(graph, neighbor, visited, depth + 1, max_depth);
            }
        }

        visited.remove(current);
    }

    fn find_longest_dependency_chains(analyses: &[DependencyAnalysis]) -> Vec<Vec<String>> {
        let mut chains = Vec::new();

        for analysis in analyses {
            if analysis.dependency_depth >= 3 {
                // Find chains for tables with significant dependency depth
                let mut chain = vec![analysis.table_name.clone()];
                let mut current = &analysis.table_name;

                // Follow dependency chain
                for _ in 0..analysis.dependency_depth {
                    if let Some(next) = analyses.iter()
                        .find(|a| a.depended_by.contains(current) && a.dependency_depth < analysis.dependency_depth)
                    {
                        chain.push(next.table_name.clone());
                        current = &next.table_name;
                    } else {
                        break;
                    }
                }

                if chain.len() >= 4 {
                    chains.push(chain);
                }
            }
        }

        chains
    }
}

pub struct RelationshipGraphService;

impl RelationshipGraphService {
    pub fn build_relationship_graph(database: &Database) -> RelationshipGraph {
        let mut nodes = HashSet::new();
        let mut edges = Vec::new();
        let mut adjacency_list = HashMap::new();

        // Add all tables as nodes
        for table in database.get_all_tables() {
            nodes.insert(table.name.clone());
            adjacency_list.entry(table.name.clone()).or_insert_with(Vec::new);
        }

        // Add relationships as edges
        for table in database.get_all_tables() {
            for fk in &table.foreign_keys {
                if fk.referenced_table != table.name { // Skip self-references for adjacency
                    edges.push(Relationship {
                        id: fk.name.clone().unwrap_or_else(|| format!("fk_{}_{}", table.name, fk.referenced_table)),
                        source_table: table.name.clone(),
                        target_table: fk.referenced_table.clone(),
                        source_columns: fk.columns.clone(),
                        target_columns: fk.referenced_columns.clone(),
                        relationship_type: RelationshipAnalysisService::infer_relationship_type(
                            fk, table, database.get_table(None, &fk.referenced_table).unwrap()
                        ),
                        cardinality: Cardinality {
                            source_min: 0,
                            source_max: None,
                            target_min: 1,
                            target_max: Some(1),
                        },
                        constraints: vec![RelationshipConstraint::ForeignKey {
                            name: fk.name.clone(),
                            on_delete: fk.on_delete.clone(),
                            on_update: fk.on_update.clone(),
                        }],
                        metadata: RelationshipMetadata {
                            description: fk.metadata.description.clone(),
                            business_rule: None,
                            is_required: false,
                            is_identifying: false,
                            tags: fk.metadata.custom_properties.keys().cloned().collect(),
                            custom_properties: fk.metadata.custom_properties.clone(),
                        },
                    });

                    // Update adjacency list
                    adjacency_list.entry(table.name.clone())
                        .or_insert_with(Vec::new)
                        .push(fk.referenced_table.clone());
                }
            }
        }

        RelationshipGraph {
            nodes,
            edges,
            adjacency_list,
        }
    }
}

pub struct RelationshipDocumentationService;

impl RelationshipDocumentationService {
    pub fn generate_relationship_diagram(database: &Database, format: DiagramFormat) -> String {
        match format {
            DiagramFormat::Mermaid => Self::generate_mermaid_diagram(database),
            DiagramFormat::PlantUML => Self::generate_plantuml_diagram(database),
            DiagramFormat::Ascii => Self::generate_ascii_diagram(database),
        }
    }

    pub fn generate_relationship_report(database: &Database) -> RelationshipReport {
        let graph = RelationshipGraph::from_database(database);
        let analyses = DependencyMappingService::analyze_dependencies(database);
        let patterns = RelationshipAnalysisService::detect_relationship_patterns(database);

        let total_relationships = graph.edges.len();
        let self_referencing_tables: HashSet<_> = graph.edges.iter()
            .filter(|edge| edge.source_table == edge.target_table)
            .map(|edge| edge.source_table.clone())
            .collect();

        let isolated_tables: Vec<_> = analyses.iter()
            .filter(|analysis| analysis.depends_on.is_empty() && analysis.depended_by.is_empty())
            .map(|analysis| analysis.table_name.clone())
            .collect();

        RelationshipReport {
            total_tables: graph.nodes.len(),
            total_relationships,
            self_referencing_tables: self_referencing_tables.into_iter().collect(),
            isolated_tables,
            relationship_patterns: patterns,
            dependency_analysis: analyses,
        }
    }

    fn generate_mermaid_diagram(database: &Database) -> String {
        let mut diagram = String::from("erDiagram\n");

        for table in database.get_all_tables() {
            diagram.push_str(&format!("    {} {{\n", table.name));

            for column in &table.columns {
                let pk_marker = if column.is_primary_key(&table.primary_key) { " PK" } else { "" };
                let fk_marker = if column.is_foreign_key(&table.foreign_keys) { " FK" } else { "" };

                diagram.push_str(&format!("        {} {}{} \"{}\"\n",
                    column.data_type.get_simple_name(),
                    column.name,
                    pk_marker,
                    fk_marker.trim()
                ));
            }

            diagram.push_str("    }\n");
        }

        // Add relationships
        for table in database.get_all_tables() {
            for fk in &table.foreign_keys {
                if let Some(cardinality) = Self::get_relationship_cardinality(fk, table, database.get_table(None, &fk.referenced_table).unwrap()) {
                    diagram.push_str(&format!("    {} {} {} : \"{}\"\n",
                        table.name,
                        cardinality,
                        fk.referenced_table,
                        fk.name.as_deref().unwrap_or("")
                    ));
                }
            }
        }

        diagram
    }

    fn generate_plantuml_diagram(database: &Database) -> String {
        let mut diagram = String::from("@startuml\n");

        for table in database.get_all_tables() {
            diagram.push_str(&format!("entity {} {{\n", table.name));

            for column in &table.columns {
                let pk_marker = if column.is_primary_key(&table.primary_key) { "*" } else { "" };
                let fk_marker = if column.is_foreign_key(&table.foreign_keys) { "#" } else { "" };

                diagram.push_str(&format!("    {} {}{} : {}\n",
                    pk_marker,
                    fk_marker,
                    column.name,
                    column.data_type.get_simple_name()
                ));
            }

            diagram.push_str("}\n");
        }

        // Add relationships
        for table in database.get_all_tables() {
            for fk in &table.foreign_keys {
                let cardinality = Self::get_relationship_cardinality_symbol(fk, table, database.get_table(None, &fk.referenced_table).unwrap());
                diagram.push_str(&format!("{} {} {}\n",
                    table.name,
                    cardinality,
                    fk.referenced_table
                ));
            }
        }

        diagram.push_str("@enduml\n");
        diagram
    }

    fn generate_ascii_diagram(database: &Database) -> String {
        let mut diagram = String::new();

        for table in database.get_all_tables() {
            diagram.push_str(&format!("┌─ {} ─┐\n", table.name));

            for column in &table.columns {
                let markers = format!("{}{}",
                    if column.is_primary_key(&table.primary_key) { "PK " } else { "" },
                    if column.is_foreign_key(&table.foreign_keys) { "FK" } else { "" }
                );

                diagram.push_str(&format!("│ {:<15} {:<10} │\n",
                    column.name,
                    format!("{} {}", column.data_type.get_simple_name(), markers.trim())
                ));
            }

            diagram.push_str("└─────────────────────────────┘\n\n");
        }

        diagram
    }

    fn get_relationship_cardinality(fk: &ForeignKey, source_table: &Table, target_table: &Table) -> Option<String> {
        let rel_type = RelationshipAnalysisService::infer_relationship_type(fk, source_table, target_table);

        match rel_type {
            RelationshipType::OneToOne => Some("||--||".to_string()),
            RelationshipType::OneToMany => Some("||--o{".to_string()),
            RelationshipType::ManyToOne => Some("}o--||".to_string()),
            RelationshipType::ManyToMany => Some("}o--o{".to_string()),
            RelationshipType::SelfReferencing => Some("||--||".to_string()),
        }
    }

    fn get_relationship_cardinality_symbol(fk: &ForeignKey, source_table: &Table, target_table: &Table) -> String {
        let rel_type = RelationshipAnalysisService::infer_relationship_type(fk, source_table, target_table);

        match rel_type {
            RelationshipType::OneToOne => "1--1",
            RelationshipType::OneToMany => "1--*",
            RelationshipType::ManyToOne => "*--1",
            RelationshipType::ManyToMany => "*--*",
            RelationshipType::SelfReferencing => "1--1",
        }.to_string()
    }
}

impl RelationshipGraph {
    pub fn from_database(database: &Database) -> Self {
        RelationshipGraphService::build_relationship_graph(database)
    }

    pub fn detect_cycles(&self) -> Option<Vec<Vec<String>>> {
        let mut cycles = Vec::new();

        for node in &self.nodes {
            if let Some(cycle) = self.find_cycle_from_node(node) {
                cycles.push(cycle);
            }
        }

        if cycles.is_empty() {
            None
        } else {
            Some(cycles)
        }
    }

    pub fn find_circular_paths(&self, start_node: &str) -> Vec<Vec<String>> {
        let mut paths = Vec::new();
        let mut visited = HashSet::new();
        let mut current_path = Vec::new();

        self.dfs_paths(start_node, start_node, &mut visited, &mut current_path, &mut paths);

        paths.into_iter()
            .filter(|path| path.len() > 2 && path.first() == path.last())
            .map(|path| {
                let mut clean_path = path.clone();
                clean_path.pop(); // Remove the duplicate end
                clean_path
            })
            .collect()
    }

    pub fn get_connected_tables(&self, start_table: &str) -> Vec<String> {
        let mut visited = HashSet::new();
        let mut result = Vec::new();

        self.dfs_connected(start_table, &mut visited, &mut result);

        result
    }

    fn find_cycle_from_node(&self, start: &str) -> Option<Vec<String>> {
        let mut visited = HashSet::new();
        let mut path = Vec::new();

        self.dfs_cycle(start, start, &mut visited, &mut path)
    }

    fn dfs_cycle(&self, start: &str, current: &str, visited: &mut HashSet<String>, path: &mut Vec<String>) -> Option<Vec<String>> {
        if visited.contains(current) {
            if current == start && path.len() > 2 {
                let mut cycle = path.clone();
                cycle.push(start.to_string());
                return Some(cycle);
            }
            return None;
        }

        visited.insert(current.to_string());
        path.push(current.to_string());

        if let Some(adjacent) = self.adjacency_list.get(current) {
            for neighbor in adjacent {
                if let Some(cycle) = self.dfs_cycle(start, neighbor, visited, path) {
                    return Some(cycle);
                }
            }
        }

        path.pop();
        visited.remove(current);
        None
    }

    fn dfs_paths(&self, start: &str, current: &str, visited: &mut HashSet<String>, path: &mut Vec<String>, paths: &mut Vec<Vec<String>>) {
        visited.insert(current.to_string());
        path.push(current.to_string());

        if let Some(adjacent) = self.adjacency_list.get(current) {
            for neighbor in adjacent {
                if !visited.contains(neighbor) {
                    self.dfs_paths(start, neighbor, visited, path, paths);
                } else if neighbor == start && path.len() > 2 {
                    // Found a cycle back to start
                    let mut cycle_path = path.clone();
                    cycle_path.push(start.to_string());
                    paths.push(cycle_path);
                }
            }
        }

        path.pop();
        visited.remove(current);
    }

    fn dfs_connected(&self, current: &str, visited: &mut HashSet<String>, result: &mut Vec<String>) {
        if visited.contains(current) {
            return;
        }

        visited.insert(current.to_string());
        result.push(current.to_string());

        if let Some(adjacent) = self.adjacency_list.get(current) {
            for neighbor in adjacent {
                self.dfs_connected(neighbor, visited, result);
            }
        }

        // Also check incoming connections
        for (node, adjacent) in &self.adjacency_list {
            if adjacent.contains(&current.to_string()) && !visited.contains(node) {
                self.dfs_connected(node, visited, result);
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TableRelationshipAnalysis {
    pub table_name: String,
    pub incoming_relationships: Vec<AnalyzedRelationship>,
    pub outgoing_relationships: Vec<AnalyzedRelationship>,
    pub self_references: Vec<AnalyzedRelationship>,
    pub total_relationships: usize,
    pub has_circular_refs: bool,
    pub isolation_level: IsolationLevel,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnalyzedRelationship {
    pub related_table: String,
    pub relationship_type: RelationshipType,
    pub columns: Vec<String>,
    pub referenced_columns: Vec<String>,
    pub is_nullable: bool,
    pub constraint_name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum IsolationLevel {
    Isolated,
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RelationshipPattern {
    pub pattern_type: PatternType,
    pub description: String,
    pub involved_tables: Vec<String>,
    pub confidence: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PatternType {
    StarSchema,
    SnowflakeSchema,
    MasterDetail,
    Hierarchical,
    Network,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DiagramFormat {
    Mermaid,
    PlantUML,
    Ascii,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DependencyReport {
    pub total_tables: usize,
    pub tables_with_circular_dependencies: Vec<String>,
    pub max_dependency_depth: usize,
    pub isolated_tables: Vec<String>,
    pub dependency_chains: Vec<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RelationshipReport {
    pub total_tables: usize,
    pub total_relationships: usize,
    pub self_referencing_tables: Vec<String>,
    pub isolated_tables: Vec<String>,
    pub relationship_patterns: Vec<RelationshipPattern>,
    pub dependency_analysis: Vec<DependencyAnalysis>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_relationship_validation() {
        let mut database = Database::new("test_db");
        let mut table1 = Table::new("users");
        table1.add_column(Column::new("id", DataType::Integer { unsigned: false }));
        table1.set_primary_key(vec!["id".to_string()]);

        let mut table2 = Table::new("posts");
        table2.add_column(Column::new("id", DataType::Integer { unsigned: false }));
        table2.add_column(Column::new("user_id", DataType::Integer { unsigned: false }));
        table2.set_primary_key(vec!["id".to_string()]);

        let fk = ForeignKey {
            name: Some("fk_posts_users".to_string()),
            table_name: "posts".to_string(),
            columns: vec!["user_id".to_string()],
            referenced_table: "users".to_string(),
            referenced_columns: vec!["id".to_string()],
            on_delete: ReferentialAction::Cascade,
            on_update: ReferentialAction::Restrict,
            constraint_name: None,
            metadata: ForeignKeyMetadata::default(),
        };

        table2.add_foreign_key(fk);

        database.add_table(table1);
        database.add_table(table2);

        let relationship = Relationship {
            id: "test_relationship".to_string(),
            source_table: "posts".to_string(),
            target_table: "users".to_string(),
            source_columns: vec!["user_id".to_string()],
            target_columns: vec!["id".to_string()],
            relationship_type: RelationshipType::ManyToOne,
            cardinality: Cardinality {
                source_min: 0,
                source_max: None,
                target_min: 1,
                target_max: Some(1),
            },
            constraints: vec![],
            metadata: RelationshipMetadata::default(),
        };

        let result = RelationshipValidationService::validate_relationship(&relationship, &database);
        assert!(result.is_valid);
    }

    #[test]
    fn test_relationship_graph() {
        let mut database = Database::new("test_db");

        let mut table1 = Table::new("users");
        table1.add_column(Column::new("id", DataType::Integer { unsigned: false }));
        table1.set_primary_key(vec!["id".to_string()]);

        let mut table2 = Table::new("posts");
        table2.add_column(Column::new("id", DataType::Integer { unsigned: false }));
        table2.add_column(Column::new("user_id", DataType::Integer { unsigned: false }));

        let fk = ForeignKey {
            name: None,
            table_name: "posts".to_string(),
            columns: vec!["user_id".to_string()],
            referenced_table: "users".to_string(),
            referenced_columns: vec!["id".to_string()],
            on_delete: ReferentialAction::Cascade,
            on_update: ReferentialAction::Restrict,
            constraint_name: None,
            metadata: ForeignKeyMetadata::default(),
        };

        table2.add_foreign_key(fk);

        database.add_table(table1);
        database.add_table(table2);

        let graph = RelationshipGraph::from_database(&database);

        assert_eq!(graph.nodes.len(), 2);
        assert_eq!(graph.edges.len(), 1);
        assert!(graph.nodes.contains("users"));
        assert!(graph.nodes.contains("posts"));
    }

    #[test]
    fn test_dependency_analysis() {
        let mut database = Database::new("test_db");

        let mut table1 = Table::new("users");
        table1.add_column(Column::new("id", DataType::Integer { unsigned: false }));
        table1.set_primary_key(vec!["id".to_string()]);

        let mut table2 = Table::new("posts");
        table2.add_column(Column::new("id", DataType::Integer { unsigned: false }));
        table2.add_column(Column::new("user_id", DataType::Integer { unsigned: false }));

        let fk = ForeignKey {
            name: None,
            table_name: "posts".to_string(),
            columns: vec!["user_id".to_string()],
            referenced_table: "users".to_string(),
            referenced_columns: vec!["id".to_string()],
            on_delete: ReferentialAction::Cascade,
            on_update: ReferentialAction::Restrict,
            constraint_name: None,
            metadata: ForeignKeyMetadata::default(),
        };

        table2.add_foreign_key(fk);

        database.add_table(table1);
        database.add_table(table2);

        let analyses = DependencyMappingService::analyze_dependencies(&database);

        assert_eq!(analyses.len(), 2);

        let posts_analysis = analyses.iter().find(|a| a.table_name == "posts").unwrap();
        assert!(posts_analysis.depends_on.contains(&"users".to_string()));

        let users_analysis = analyses.iter().find(|a| a.table_name == "users").unwrap();
        assert!(users_analysis.depended_by.contains(&"posts".to_string()));
    }
}
