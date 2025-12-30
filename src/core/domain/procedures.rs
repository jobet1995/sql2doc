use std::collections::{HashMap, HashSet};
use serde::{Deserialize, Serialize};

use crate::core::domain::entity::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProcedureAnalysis {
    pub procedure_name: String,
    pub schema_name: Option<String>,
    pub parameters: Vec<ProcedureParameter>,
    pub definition: String,
    pub complexity: ProcedureComplexity,
    pub dependencies: ProcedureDependencies,
    pub performance_characteristics: ProcedurePerformance,
    pub security_assessment: ProcedureSecurity,
    pub business_logic: Vec<ProcedureBusinessLogic>,
    pub issues: Vec<ProcedureIssue>,
    pub recommendations: Vec<ProcedureRecommendation>,
    pub metadata: ProcedureMetadata,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProcedureParameter {
    pub name: String,
    pub data_type: String,
    pub parameter_mode: ParameterMode,
    pub is_nullable: bool,
    pub default_value: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ParameterMode {
    In,
    Out,
    InOut,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProcedureComplexity {
    pub lines_of_code: usize,
    pub cyclomatic_complexity: usize,
    pub nesting_depth: usize,
    pub number_of_variables: usize,
    pub number_of_statements: usize,
    pub maintainability_index: f64,
    pub complexity_score: f64, // 0.0 to 1.0
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProcedureDependencies {
    pub tables_accessed: Vec<String>,
    pub views_accessed: Vec<String>,
    pub procedures_called: Vec<String>,
    pub functions_called: Vec<String>,
    pub external_dependencies: Vec<String>,
    pub dependency_depth: usize,
    pub is_recursive: bool,
    pub circular_dependencies: Vec<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProcedurePerformance {
    pub estimated_execution_time: Option<f64>,
    pub estimated_cpu_cost: Option<f64>,
    pub estimated_io_cost: Option<f64>,
    pub memory_usage_estimate: Option<String>,
    pub temp_table_usage: bool,
    pub cursor_usage: bool,
    pub optimization_opportunities: Vec<PerformanceOptimization>,
    pub execution_plans: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PerformanceOptimization {
    pub optimization_type: OptimizationType,
    pub description: String,
    pub expected_improvement: f64,
    pub implementation_effort: EffortLevel,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum OptimizationType {
    QueryRewrite,
    IndexCreation,
    TempTableElimination,
    CursorReplacement,
    ParameterSniffingFix,
    StatisticsUpdate,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProcedureSecurity {
    pub security_level: SecurityLevel,
    pub dynamic_sql_usage: bool,
    pub input_validation: ValidationLevel,
    pub privilege_escalation_risk: RiskLevel,
    pub data_exposure_risk: RiskLevel,
    pub injection_vulnerabilities: Vec<String>,
    pub security_recommendations: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ValidationLevel {
    None,
    Basic,
    Comprehensive,
    Paranoid,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProcedureBusinessLogic {
    pub logic_type: BusinessLogicType,
    pub description: String,
    pub affected_entities: Vec<String>,
    pub business_rules: Vec<String>,
    pub error_handling: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BusinessLogicType {
    DataProcessing,
    BusinessRuleEnforcement,
    WorkflowManagement,
    Reporting,
    Maintenance,
    Integration,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProcedureIssue {
    pub issue_type: ProcedureIssueType,
    pub severity: IssueSeverity,
    pub description: String,
    pub line_number: Option<usize>,
    pub impact: String,
    pub suggested_fix: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ProcedureIssueType {
    PerformanceProblem,
    SecurityVulnerability,
    CodeQualityIssue,
    MaintainabilityProblem,
    LogicError,
    ResourceLeak,
    ErrorHandlingGap,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProcedureRecommendation {
    pub recommendation_type: RecommendationType,
    pub description: String,
    pub benefit: String,
    pub effort: EffortLevel,
    pub sql_commands: Vec<String>,
    pub expected_improvement: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RecommendationType {
    PerformanceOptimization,
    SecurityEnhancement,
    CodeRefactoring,
    ErrorHandlingImprovement,
    DocumentationUpdate,
    TestingAddition,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProcedureMetadata {
    pub description: Option<String>,
    pub created_by: Option<String>,
    pub created_at: Option<String>,
    pub last_modified: Option<String>,
    pub version: Option<String>,
    pub business_owner: Option<String>,
    pub technical_owner: Option<String>,
    pub usage_frequency: Option<String>,
    pub tags: Vec<String>,
    pub custom_properties: HashMap<String, String>,
}

pub struct ProcedureAnalysisService;

impl ProcedureAnalysisService {
    pub fn analyze_procedure(procedure: &Procedure, database: &Database) -> ProcedureAnalysis {
        let mut issues = Vec::new();
        let mut recommendations = Vec::new();

        let parameters = Self::analyze_parameters(procedure);
        let complexity = Self::calculate_complexity(procedure);
        let dependencies = Self::analyze_dependencies(procedure, database);
        let performance_characteristics = Self::analyze_performance(procedure, database);
        let security_assessment = Self::assess_security(procedure);
        let business_logic = Self::extract_business_logic(procedure);

        issues.extend(Self::analyze_issues(procedure, &complexity, &security_assessment));
        recommendations.extend(Self::generate_recommendations(&issues, &performance_characteristics, &security_assessment));

        ProcedureAnalysis {
            procedure_name: procedure.name.clone(),
            schema_name: procedure.schema_name.clone(),
            parameters,
            definition: procedure.definition.clone(),
            complexity,
            dependencies,
            performance_characteristics,
            security_assessment,
            business_logic,
            issues,
            recommendations,
            metadata: ProcedureMetadata {
                description: procedure.metadata.description.clone(),
                created_by: procedure.metadata.created_by.clone(),
                created_at: procedure.metadata.created_at.clone(),
                last_modified: procedure.metadata.last_modified.clone(),
                version: procedure.metadata.version.clone(),
                business_owner: procedure.metadata.business_owner.clone(),
                technical_owner: procedure.metadata.technical_owner.clone(),
                usage_frequency: procedure.metadata.usage_frequency.clone(),
                tags: procedure.metadata.tags.clone(),
                custom_properties: procedure.metadata.custom_properties.clone(),
            },
        }
    }

    pub fn analyze_procedure_dependencies(database: &Database) -> ProcedureDependencyGraph {
        let mut graph = ProcedureDependencyGraph {
            nodes: HashSet::new(),
            edges: Vec::new(),
            execution_paths: Vec::new(),
            circular_dependencies: Vec::new(),
        };

        // Add all procedures and related objects as nodes
        for procedure in database.get_all_procedures() {
            graph.nodes.insert(format!("procedure:{}.{}", procedure.schema_name.as_deref().unwrap_or("dbo"), procedure.name));
        }

        // Analyze dependencies
        for procedure in database.get_all_procedures() {
            let analysis = Self::analyze_procedure(procedure, database);

            for called_proc in &analysis.dependencies.procedures_called {
                graph.edges.push(ProcedureDependencyEdge {
                    from: format!("procedure:{}.{}", procedure.schema_name.as_deref().unwrap_or("dbo"), procedure.name),
                    to: format!("procedure:dbo.{}", called_proc), // Assuming dbo schema
                    dependency_type: ProcedureDependencyType::ProcedureCall,
                    execution_order: 1,
                });
            }
        }

        graph
    }

    fn analyze_parameters(procedure: &Procedure) -> Vec<ProcedureParameter> {
        procedure.parameters.iter().map(|param| {
            ProcedureParameter {
                name: param.name.clone(),
                data_type: param.data_type.get_simple_name(),
                parameter_mode: param.parameter_mode.clone(),
                is_nullable: param.is_nullable,
                default_value: param.default_value.clone(),
                description: param.metadata.description.clone(),
            }
        }).collect()
    }

    fn calculate_complexity(procedure: &Procedure) -> ProcedureComplexity {
        let definition = procedure.definition.to_lowercase();
        let lines_of_code = definition.lines().count();

        // Estimate cyclomatic complexity (simplified)
        let decision_points = definition.matches("if").count() +
                             definition.matches("case").count() +
                             definition.matches("while").count() +
                             definition.matches("for").count() +
                             definition.matches("and").count() / 2 + // Rough estimate
                             definition.matches("or").count() / 2;

        let cyclomatic_complexity = decision_points + 1;

        // Estimate nesting depth
        let mut max_nesting = 0;
        let mut current_nesting = 0;

        for line in definition.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("if") || trimmed.starts_with("while") ||
               trimmed.starts_with("for") || trimmed.starts_with("begin") {
                current_nesting += 1;
                max_nesting = max_nesting.max(current_nesting);
            }
            if trimmed.starts_with("end") {
                current_nesting = current_nesting.saturating_sub(1);
            }
        }

        let nesting_depth = max_nesting;

        // Count variables and statements
        let number_of_variables = definition.matches("declare").count();
        let number_of_statements = definition.matches(";").count();

        // Calculate complexity score (0.0 to 1.0)
        let complexity_score = Self::calculate_complexity_score(
            lines_of_code,
            cyclomatic_complexity,
            nesting_depth,
            number_of_statements,
        );

        // Calculate maintainability index
        let maintainability_index = Self::calculate_maintainability_index(
            lines_of_code,
            cyclomatic_complexity,
            number_of_statements,
        );

        ProcedureComplexity {
            lines_of_code,
            cyclomatic_complexity,
            nesting_depth,
            number_of_variables,
            number_of_statements,
            maintainability_index,
            complexity_score,
        }
    }

    fn analyze_dependencies(procedure: &Procedure, database: &Database) -> ProcedureDependencies {
        let definition = procedure.definition.to_lowercase();

        let mut tables_accessed = Vec::new();
        let mut views_accessed = Vec::new();
        let mut procedures_called = Vec::new();
        let mut functions_called = Vec::new();

        // Extract table references
        for table in database.get_all_tables() {
            if definition.contains(&table.name.to_lowercase()) {
                tables_accessed.push(table.name.clone());
            }
        }

        // Extract view references
        for view in database.get_all_views() {
            if definition.contains(&view.name.to_lowercase()) {
                views_accessed.push(view.name.clone());
            }
        }

        // Extract procedure calls (simplified pattern matching)
        for other_proc in database.get_all_procedures() {
            if other_proc.name != procedure.name &&
               definition.contains(&format!("exec {}", other_proc.name.to_lowercase())) {
                procedures_called.push(other_proc.name.clone());
            }
        }

        // Extract function calls
        for function in database.get_all_functions() {
            if definition.contains(&function.name.to_lowercase()) {
                functions_called.push(function.name.clone());
            }
        }

        let dependency_depth = Self::calculate_dependency_depth(&procedures_called, database);
        let circular_dependencies = Vec::new(); // Would need graph analysis

        ProcedureDependencies {
            tables_accessed,
            views_accessed,
            procedures_called,
            functions_called,
            external_dependencies: Vec::new(), // Would analyze external system calls
            dependency_depth,
            is_recursive: definition.contains("exec") && definition.contains(&procedure.name.to_lowercase()),
            circular_dependencies,
        }
    }

    fn analyze_performance(procedure: &Procedure, database: &Database) -> ProcedurePerformance {
        let definition = procedure.definition.to_lowercase();

        let temp_table_usage = definition.contains("#") || definition.contains("tempdb");
        let cursor_usage = definition.contains("cursor") || definition.contains("fetch");

        let mut optimization_opportunities = Vec::new();

        if cursor_usage {
            optimization_opportunities.push(PerformanceOptimization {
                optimization_type: OptimizationType::CursorReplacement,
                description: "Replace cursor operations with set-based operations".to_string(),
                expected_improvement: 0.8,
                implementation_effort: EffortLevel::High,
            });
        }

        if temp_table_usage {
            optimization_opportunities.push(PerformanceOptimization {
                optimization_type: OptimizationType::TempTableElimination,
                description: "Consider eliminating temporary table usage".to_string(),
                expected_improvement: 0.6,
                implementation_effort: EffortLevel::Medium,
            });
        }

        if definition.contains("select *") {
            optimization_opportunities.push(PerformanceOptimization {
                optimization_type: OptimizationType::QueryRewrite,
                description: "Replace SELECT * with specific column selection".to_string(),
                expected_improvement: 0.3,
                implementation_effort: EffortLevel::Low,
            });
        }

        ProcedurePerformance {
            estimated_execution_time: None, // Would need execution statistics
            estimated_cpu_cost: None,
            estimated_io_cost: None,
            memory_usage_estimate: Some("Medium".to_string()), // Placeholder
            temp_table_usage,
            cursor_usage,
            optimization_opportunities,
            execution_plans: Vec::new(), // Would be populated from query analysis
        }
    }

    fn assess_security(procedure: &Procedure) -> ProcedureSecurity {
        let definition = procedure.definition.to_lowercase();

        let dynamic_sql_usage = definition.contains("exec(") ||
                               definition.contains("execute(") ||
                               definition.contains("sp_executesql");

        let input_validation = if definition.contains("isnull") || definition.contains("len(") {
            ValidationLevel::Basic
        } else if definition.contains("try") && definition.contains("catch") {
            ValidationLevel::Comprehensive
        } else {
            ValidationLevel::None
        };

        let privilege_escalation_risk = if dynamic_sql_usage && !definition.contains("with execute as") {
            RiskLevel::High
        } else {
            RiskLevel::Low
        };

        let data_exposure_risk = if definition.contains("select") && !definition.contains("where") {
            RiskLevel::Medium
        } else {
            RiskLevel::Low
        };

        let mut injection_vulnerabilities = Vec::new();
        if dynamic_sql_usage {
            injection_vulnerabilities.push("Dynamic SQL without parameterized queries".to_string());
        }

        let security_level = if privilege_escalation_risk == RiskLevel::High || data_exposure_risk == RiskLevel::High {
            SecurityLevel::Restricted
        } else if privilege_escalation_risk == RiskLevel::Medium || data_exposure_risk == RiskLevel::Medium {
            SecurityLevel::Confidential
        } else {
            SecurityLevel::Internal
        };

        let mut security_recommendations = Vec::new();
        if dynamic_sql_usage {
            security_recommendations.push("Use parameterized queries instead of dynamic SQL".to_string());
            security_recommendations.push("Implement proper input validation and sanitization".to_string());
        }

        ProcedureSecurity {
            security_level,
            dynamic_sql_usage,
            input_validation,
            privilege_escalation_risk,
            data_exposure_risk,
            injection_vulnerabilities,
            security_recommendations,
        }
    }

    fn extract_business_logic(procedure: &Procedure) -> Vec<ProcedureBusinessLogic> {
        let mut logic = Vec::new();
        let definition = procedure.definition.to_lowercase();

        if definition.contains("insert") && definition.contains("update") {
            logic.push(ProcedureBusinessLogic {
                logic_type: BusinessLogicType::DataProcessing,
                description: "Processes and transforms data according to business rules".to_string(),
                affected_entities: vec!["target_tables".to_string()],
                business_rules: vec!["Data validation and transformation rules".to_string()],
                error_handling: vec!["Transaction rollback on errors".to_string()],
            });
        }

        if definition.contains("if") && definition.contains("raiserror") {
            logic.push(ProcedureBusinessLogic {
                logic_type: BusinessLogicType::BusinessRuleEnforcement,
                description: "Enforces business rules and constraints".to_string(),
                affected_entities: vec!["validated_entities".to_string()],
                business_rules: vec!["Custom business rule validation".to_string()],
                error_handling: vec!["Custom error messages for rule violations".to_string()],
            });
        }

        if definition.contains("while") || definition.contains("loop") {
            logic.push(ProcedureBusinessLogic {
                logic_type: BusinessLogicType::WorkflowManagement,
                description: "Manages workflow and process automation".to_string(),
                affected_entities: vec!["workflow_entities".to_string()],
                business_rules: vec!["Workflow state transition rules".to_string()],
                error_handling: vec!["Workflow rollback procedures".to_string()],
            });
        }

        logic
    }

    fn analyze_issues(procedure: &Procedure, complexity: &ProcedureComplexity, security: &ProcedureSecurity) -> Vec<ProcedureIssue> {
        let mut issues = Vec::new();
        let definition = procedure.definition.to_lowercase();

        // Performance issues
        if complexity.complexity_score > 0.8 {
            issues.push(ProcedureIssue {
                issue_type: ProcedureIssueType::PerformanceProblem,
                severity: IssueSeverity::High,
                description: format!("High complexity score ({:.2}) indicates performance issues", complexity.complexity_score),
                line_number: None,
                impact: "Slow execution and high resource usage".to_string(),
                suggested_fix: Some("Break down into smaller, simpler procedures".to_string()),
            });
        }

        // Security issues
        if security.dynamic_sql_usage {
            issues.push(ProcedureIssue {
                issue_type: ProcedureIssueType::SecurityVulnerability,
                severity: IssueSeverity::Critical,
                description: "Uses dynamic SQL which is vulnerable to injection attacks".to_string(),
                line_number: None,
                impact: "Potential SQL injection and data breach".to_string(),
                suggested_fix: Some("Replace with parameterized queries or stored procedures".to_string()),
            });
        }

        // Code quality issues
        if complexity.cyclomatic_complexity > 10 {
            issues.push(ProcedureIssue {
                issue_type: ProcedureIssueType::CodeQualityIssue,
                severity: IssueSeverity::Medium,
                description: format!("High cyclomatic complexity ({}) makes code hard to test and maintain", complexity.cyclomatic_complexity),
                line_number: None,
                impact: "Increased maintenance cost and bug likelihood".to_string(),
                suggested_fix: Some("Refactor into smaller, focused functions".to_string()),
            });
        }

        // Error handling issues
        if !definition.contains("try") && !definition.contains("raiserror") {
            issues.push(ProcedureIssue {
                issue_type: ProcedureIssueType::ErrorHandlingGap,
                severity: IssueSeverity::Medium,
                description: "Missing comprehensive error handling".to_string(),
                line_number: None,
                impact: "Unpredictable behavior on errors".to_string(),
                suggested_fix: Some("Add TRY-CATCH blocks and proper error handling".to_string()),
            });
        }

        issues
    }

    fn generate_recommendations(
        issues: &[ProcedureIssue],
        performance: &ProcedurePerformance,
        security: &ProcedureSecurity,
    ) -> Vec<ProcedureRecommendation> {
        let mut recommendations = Vec::new();

        // Recommendations based on issues
        for issue in issues {
            match issue.issue_type {
                ProcedureIssueType::PerformanceProblem => {
                    recommendations.push(ProcedureRecommendation {
                        recommendation_type: RecommendationType::PerformanceOptimization,
                        description: "Optimize procedure performance through refactoring".to_string(),
                        benefit: "Faster execution and reduced resource usage".to_string(),
                        effort: EffortLevel::High,
                        sql_commands: vec![
                            "-- Analyze execution plan".to_string(),
                            "-- Identify bottlenecks".to_string(),
                            "-- Implement optimizations".to_string(),
                        ],
                        expected_improvement: Some(0.5),
                    });
                }
                ProcedureIssueType::SecurityVulnerability => {
                    recommendations.push(ProcedureRecommendation {
                        recommendation_type: RecommendationType::SecurityEnhancement,
                        description: "Address security vulnerabilities in procedure".to_string(),
                        benefit: "Prevents security breaches and data exposure".to_string(),
                        effort: EffortLevel::High,
                        sql_commands: vec![
                            "-- Implement parameterized queries".to_string(),
                            "-- Add input validation".to_string(),
                            "-- Use least privilege execution context".to_string(),
                        ],
                        expected_improvement: Some(0.9),
                    });
                }
                _ => {}
            }
        }

        // Performance optimization recommendations
        for optimization in &performance.optimization_opportunities {
            recommendations.push(ProcedureRecommendation {
                recommendation_type: RecommendationType::PerformanceOptimization,
                description: optimization.description.clone(),
                benefit: format!("Expected improvement: {:.1}%", optimization.expected_improvement * 100.0),
                effort: optimization.implementation_effort.clone(),
                sql_commands: vec![
                    format!("-- Implement {}", optimization.optimization_type.to_string().to_lowercase()),
                ],
                expected_improvement: Some(optimization.expected_improvement),
            });
        }

        // Security recommendations
        for recommendation in &security.security_recommendations {
            recommendations.push(ProcedureRecommendation {
                recommendation_type: RecommendationType::SecurityEnhancement,
                description: recommendation.clone(),
                benefit: "Enhanced security posture".to_string(),
                effort: EffortLevel::Medium,
                sql_commands: vec![
                    format!("-- {}", recommendation),
                ],
                expected_improvement: Some(0.8),
            });
        }

        recommendations
    }

    fn calculate_complexity_score(lines: usize, cyclomatic: usize, nesting: usize, statements: usize) -> f64 {
        let lines_score = (lines as f64 / 200.0).min(1.0) * 0.3;
        let cyclomatic_score = (cyclomatic as f64 / 20.0).min(1.0) * 0.4;
        let nesting_score = (nesting as f64 / 5.0).min(1.0) * 0.2;
        let statements_score = (statements as f64 / 100.0).min(1.0) * 0.1;

        (lines_score + cyclomatic_score + nesting_score + statements_score).min(1.0)
    }

    fn calculate_maintainability_index(lines: usize, cyclomatic: usize, statements: usize) -> f64 {
        // Simplified maintainability index calculation
        let volume = (lines as f64).ln() + (statements as f64).ln();
        let complexity = (cyclomatic as f64).ln();
        let maintainability = 171.0 - 5.2 * volume.ln() - 0.23 * complexity - 16.2 * (statements as f64 / lines as f64).ln();

        maintainability.max(0.0).min(171.0) / 171.0 // Normalize to 0.0-1.0
    }

    fn calculate_dependency_depth(procedures_called: &[String], database: &Database) -> usize {
        let mut max_depth = 0;

        for proc_name in procedures_called {
            if let Some(called_proc) = database.get_procedure(None, proc_name) {
                let analysis = Self::analyze_procedure(called_proc, database);
                max_depth = max_depth.max(analysis.dependencies.dependency_depth + 1);
            }
        }

        max_depth
    }
}

pub struct ProcedureDocumentationService;

impl ProcedureDocumentationService {
    pub fn generate_procedure_documentation(analysis: &ProcedureAnalysis) -> String {
        let mut doc = format!("### Stored Procedure: {}\n\n", analysis.procedure_name);

        if let Some(schema) = &analysis.schema_name {
            doc.push_str(&format!("**Schema:** {}\n", schema));
        }

        if let Some(description) = &analysis.metadata.description {
            doc.push_str(&format!("**Description:** {}\n", description));
        }

        doc.push_str(&format!("**Complexity Score:** {:.2}%\n", analysis.complexity.complexity_score * 100.0));
        doc.push_str(&format!("**Maintainability Index:** {:.2}%\n", analysis.complexity.maintainability_index * 100.0));
        doc.push_str(&format!("**Security Level:** {}\n", Self::security_level_name(&analysis.security_assessment.security_level)));

        if let Some(owner) = &analysis.metadata.business_owner {
            doc.push_str(&format!("**Business Owner:** {}\n", owner));
        }

        if let Some(usage) = &analysis.metadata.usage_frequency {
            doc.push_str(&format!("**Usage Frequency:** {}\n", usage));
        }

        if !analysis.parameters.is_empty() {
            doc.push_str("\n#### Parameters\n\n");
            doc.push_str("| Parameter | Type | Mode | Nullable | Default | Description |\n");
            doc.push_str("|-----------|------|------|----------|---------|-------------|\n");

            for param in &analysis.parameters {
                doc.push_str(&format!("| {} | {} | {} | {} | {} | {} |\n",
                    param.name,
                    param.data_type,
                    Self::parameter_mode_name(&param.parameter_mode),
                    if param.is_nullable { "Yes" } else { "No" },
                    param.default_value.as_deref().unwrap_or(""),
                    param.description.as_deref().unwrap_or("")));
            }
        }

        doc.push_str("\n#### Complexity Metrics\n\n");
        doc.push_str(&format!("- **Lines of Code:** {}\n", analysis.complexity.lines_of_code));
        doc.push_str(&format!("- **Cyclomatic Complexity:** {}\n", analysis.complexity.cyclomatic_complexity));
        doc.push_str(&format!("- **Nesting Depth:** {}\n", analysis.complexity.nesting_depth));
        doc.push_str(&format!("- **Number of Variables:** {}\n", analysis.complexity.number_of_variables));
        doc.push_str(&format!("- **Number of Statements:** {}\n", analysis.complexity.number_of_statements));

        doc.push_str("\n#### Dependencies\n\n");
        doc.push_str(&format!("- **Tables Accessed:** {}\n", analysis.dependencies.tables_accessed.join(", ")));
        doc.push_str(&format!("- **Views Accessed:** {}\n", analysis.dependencies.views_accessed.join(", ")));
        doc.push_str(&format!("- **Procedures Called:** {}\n", analysis.dependencies.procedures_called.join(", ")));
        doc.push_str(&format!("- **Functions Called:** {}\n", analysis.dependencies.functions_called.join(", ")));
        doc.push_str(&format!("- **Dependency Depth:** {}\n", analysis.dependencies.dependency_depth));
        doc.push_str(&format!("- **Recursive:** {}\n", if analysis.dependencies.is_recursive { "Yes" } else { "No" }));

        doc.push_str("\n#### Performance Characteristics\n\n");
        doc.push_str(&format!("- **Temporary Table Usage:** {}\n", if analysis.performance_characteristics.temp_table_usage { "Yes" } else { "No" }));
        doc.push_str(&format!("- **Cursor Usage:** {}\n", if analysis.performance_characteristics.cursor_usage { "Yes" } else { "No" }));

        if let Some(memory) = &analysis.performance_characteristics.memory_usage_estimate {
            doc.push_str(&format!("- **Memory Usage Estimate:** {}\n", memory));
        }

        if !analysis.business_logic.is_empty() {
            doc.push_str("\n#### Business Logic\n\n");
            for logic in &analysis.business_logic {
                doc.push_str(&format!("- **{}:** {}\n", Self::business_logic_type_name(&logic.logic_type), logic.description));

                if !logic.business_rules.is_empty() {
                    doc.push_str("  - **Business Rules:**\n");
                    for rule in &logic.business_rules {
                        doc.push_str(&format!("    - {}\n", rule));
                    }
                }
            }
        }

        if !analysis.security_assessment.injection_vulnerabilities.is_empty() {
            doc.push_str("\n#### Security Issues\n\n");
            for vulnerability in &analysis.security_assessment.injection_vulnerabilities {
                doc.push_str(&format!("- 🚨 **Vulnerability:** {}\n", vulnerability));
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

    fn parameter_mode_name(mode: &ParameterMode) -> &'static str {
        match mode {
            ParameterMode::In => "IN",
            ParameterMode::Out => "OUT",
            ParameterMode::InOut => "INOUT",
        }
    }

    fn security_level_name(level: &SecurityLevel) -> &'static str {
        match level {
            SecurityLevel::Public => "Public",
            SecurityLevel::Internal => "Internal",
            SecurityLevel::Confidential => "Confidential",
            SecurityLevel::Restricted => "Restricted",
            SecurityLevel::Classified => "Classified",
        }
    }

    fn business_logic_type_name(logic_type: &BusinessLogicType) -> &'static str {
        match logic_type {
            BusinessLogicType::DataProcessing => "Data Processing",
            BusinessLogicType::BusinessRuleEnforcement => "Business Rule Enforcement",
            BusinessLogicType::WorkflowManagement => "Workflow Management",
            BusinessLogicType::Reporting => "Reporting",
            BusinessLogicType::Maintenance => "Maintenance",
            BusinessLogicType::Integration => "Integration",
        }
    }

    fn issue_type_name(issue_type: &ProcedureIssueType) -> &'static str {
        match issue_type {
            ProcedureIssueType::PerformanceProblem => "Performance Problem",
            ProcedureIssueType::SecurityVulnerability => "Security Vulnerability",
            ProcedureIssueType::CodeQualityIssue => "Code Quality Issue",
            ProcedureIssueType::MaintainabilityProblem => "Maintainability Problem",
            ProcedureIssueType::LogicError => "Logic Error",
            ProcedureIssueType::ResourceLeak => "Resource Leak",
            ProcedureIssueType::ErrorHandlingGap => "Error Handling Gap",
        }
    }

    fn recommendation_type_name(rec_type: &RecommendationType) -> &'static str {
        match rec_type {
            RecommendationType::PerformanceOptimization => "Performance Optimization",
            RecommendationType::SecurityEnhancement => "Security Enhancement",
            RecommendationType::CodeRefactoring => "Code Refactoring",
            RecommendationType::ErrorHandlingImprovement => "Error Handling Improvement",
            RecommendationType::DocumentationUpdate => "Documentation Update",
            RecommendationType::TestingAddition => "Testing Addition",
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
pub struct ProcedureDependencyGraph {
    pub nodes: HashSet<String>,
    pub edges: Vec<ProcedureDependencyEdge>,
    pub execution_paths: Vec<Vec<String>>,
    pub circular_dependencies: Vec<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProcedureDependencyEdge {
    pub from: String,
    pub to: String,
    pub dependency_type: ProcedureDependencyType,
    pub execution_order: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ProcedureDependencyType {
    ProcedureCall,
    FunctionCall,
    TableAccess,
    ViewAccess,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_procedure_complexity_calculation() {
        let procedure = Procedure {
            name: "complex_proc".to_string(),
            schema_name: Some("dbo".to_string()),
            parameters: vec![],
            definition: r#"
                IF @condition = 1
                BEGIN
                    IF @other_condition = 1
                    BEGIN
                        SELECT * FROM table1;
                        IF @third_condition = 1
                        BEGIN
                            UPDATE table2 SET col = 1;
                        END
                    END
                END
            "#.to_string(),
            metadata: ProcedureMetadata::default(),
        };

        let complexity = ProcedureAnalysisService::calculate_complexity(&procedure);

        assert!(complexity.lines_of_code > 0);
        assert!(complexity.cyclomatic_complexity >= 4); // Multiple IF statements
        assert!(complexity.nesting_depth >= 3);
        assert!(complexity.complexity_score > 0.0);
    }

    #[test]
    fn test_security_assessment() {
        let procedure = Procedure {
            name: "insecure_proc".to_string(),
            schema_name: Some("dbo".to_string()),
            parameters: vec![],
            definition: r#"
                DECLARE @sql NVARCHAR(MAX) = 'SELECT * FROM ' + @table_name;
                EXEC(@sql);
            "#.to_string(),
            metadata: ProcedureMetadata::default(),
        };

        let security = ProcedureAnalysisService::assess_security(&procedure);

        assert!(security.dynamic_sql_usage);
        assert_eq!(security.privilege_escalation_risk, RiskLevel::High);
        assert!(!security.injection_vulnerabilities.is_empty());
    }

    #[test]
    fn test_business_logic_extraction() {
        let procedure = Procedure {
            name: "business_proc".to_string(),
            schema_name: Some("dbo".to_string()),
            parameters: vec![],
            definition: r#"
                BEGIN TRY
                    INSERT INTO orders (customer_id, amount)
                    VALUES (@customer_id, @amount);

                    UPDATE inventory
                    SET quantity = quantity - @quantity
                    WHERE product_id = @product_id;

                    IF @@ERROR <> 0
                        RAISERROR('Business rule violation', 16, 1);
                END TRY
                BEGIN CATCH
                    ROLLBACK TRANSACTION;
                    THROW;
                END CATCH
            "#.to_string(),
            metadata: ProcedureMetadata::default(),
        };

        let business_logic = ProcedureAnalysisService::extract_business_logic(&procedure);

        assert!(business_logic.iter().any(|logic| matches!(logic.logic_type, BusinessLogicType::DataProcessing)));
        assert!(business_logic.iter().any(|logic| matches!(logic.logic_type, BusinessLogicType::BusinessRuleEnforcement)));
    }
}
