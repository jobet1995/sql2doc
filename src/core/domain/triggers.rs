use std::collections::{HashMap, HashSet};
use serde::{Deserialize, Serialize};

use crate::core::domain::entity::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TriggerAnalysis {
    pub trigger_name: String,
    pub table_name: String,
    pub timing: TriggerTiming,
    pub events: Vec<TriggerEvent>,
    pub scope: TriggerScope,
    pub definition: String,
    pub business_logic: Vec<BusinessLogic>,
    pub data_integrity_rules: Vec<DataIntegrityRule>,
    pub performance_impact: TriggerPerformanceImpact,
    pub security_implications: Vec<SecurityImplication>,
    pub issues: Vec<TriggerIssue>,
    pub recommendations: Vec<TriggerRecommendation>,
    pub metadata: TriggerMetadata,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TriggerTiming {
    Before,
    After,
    InsteadOf,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TriggerEvent {
    Insert,
    Update,
    Delete,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TriggerScope {
    Row,
    Statement,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BusinessLogic {
    pub logic_type: BusinessLogicType,
    pub description: String,
    pub affected_columns: Vec<String>,
    pub conditions: Vec<String>,
    pub actions: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BusinessLogicType {
    AuditTrail,
    DataValidation,
    CascadeOperations,
    ComputedColumns,
    Notifications,
    WorkflowManagement,
    CustomLogic,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataIntegrityRule {
    pub rule_type: IntegrityRuleType,
    pub description: String,
    pub enforced_columns: Vec<String>,
    pub validation_logic: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum IntegrityRuleType {
    CheckConstraint,
    ReferentialIntegrity,
    DomainValidation,
    BusinessRule,
    DataConsistency,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TriggerPerformanceImpact {
    pub execution_cost: PerformanceCost,
    pub frequency_estimate: ExecutionFrequency,
    pub locking_behavior: LockingBehavior,
    pub cascading_triggers: Vec<String>,
    pub optimization_opportunities: Vec<PerformanceOptimization>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PerformanceCost {
    Low,
    Medium,
    High,
    VeryHigh,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ExecutionFrequency {
    Rare,
    Occasional,
    Frequent,
    VeryFrequent,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LockingBehavior {
    Minimal,
    Moderate,
    Extensive,
    Exclusive,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PerformanceOptimization {
    pub optimization_type: TriggerOptimizationType,
    pub description: String,
    pub expected_improvement: f64,
    pub implementation_effort: EffortLevel,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TriggerOptimizationType {
    ConditionOptimization,
    BulkProcessing,
    IndexCreation,
    TriggerConsolidation,
    AsynchronousProcessing,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SecurityImplication {
    pub implication_type: SecurityImplicationType,
    pub description: String,
    pub risk_level: RiskLevel,
    pub mitigation_suggestions: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SecurityImplicationType {
    PrivilegeEscalation,
    DataExposure,
    InjectionRisk,
    DenialOfService,
    AuditBypass,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TriggerIssue {
    pub issue_type: TriggerIssueType,
    pub severity: IssueSeverity,
    pub description: String,
    pub impact: String,
    pub suggested_fix: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TriggerIssueType {
    RecursiveTrigger,
    MissingErrorHandling,
    PerformanceProblem,
    LogicError,
    SecurityRisk,
    MaintainabilityIssue,
    ConflictingTrigger,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TriggerRecommendation {
    pub recommendation_type: TriggerRecommendationType,
    pub description: String,
    pub benefit: String,
    pub effort: EffortLevel,
    pub sql_commands: Vec<String>,
    pub expected_improvement: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TriggerRecommendationType {
    AddErrorHandling,
    OptimizeConditions,
    ConsolidateTriggers,
    AddLogging,
    ImplementValidation,
    ReviewSecurity,
    PerformanceTuning,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TriggerMetadata {
    pub description: Option<String>,
    pub created_by: Option<String>,
    pub created_at: Option<String>,
    pub last_modified: Option<String>,
    pub version: Option<String>,
    pub business_owner: Option<String>,
    pub technical_owner: Option<String>,
    pub tags: Vec<String>,
    pub custom_properties: HashMap<String, String>,
}

pub struct TriggerAnalysisService;

impl TriggerAnalysisService {
    pub fn analyze_trigger(trigger: &Trigger, table: &Table, database: &Database) -> TriggerAnalysis {
        let mut issues = Vec::new();
        let mut recommendations = Vec::new();

        // Analyze business logic
        let business_logic = Self::extract_business_logic(trigger);

        // Analyze data integrity rules
        let data_integrity_rules = Self::extract_integrity_rules(trigger, table);

        // Analyze performance impact
        let performance_impact = Self::analyze_performance_impact(trigger, table, database);

        // Analyze security implications
        let security_implications = Self::analyze_security_implications(trigger);

        // Check for issues
        issues.extend(Self::analyze_issues(trigger, table, database));

        // Generate recommendations
        recommendations.extend(Self::generate_recommendations(trigger, &issues, &performance_impact));

        TriggerAnalysis {
            trigger_name: trigger.name.clone(),
            table_name: table.name.clone(),
            timing: trigger.timing.clone(),
            events: trigger.events.clone(),
            scope: trigger.scope.clone(),
            definition: trigger.definition.clone(),
            business_logic,
            data_integrity_rules,
            performance_impact,
            security_implications,
            issues,
            recommendations,
            metadata: TriggerMetadata {
                description: trigger.metadata.description.clone(),
                created_by: trigger.metadata.created_by.clone(),
                created_at: trigger.metadata.created_at.clone(),
                last_modified: trigger.metadata.last_modified.clone(),
                version: trigger.metadata.version.clone(),
                business_owner: trigger.metadata.business_owner.clone(),
                technical_owner: trigger.metadata.technical_owner.clone(),
                tags: trigger.metadata.tags.clone(),
                custom_properties: trigger.metadata.custom_properties.clone(),
            },
        }
    }

    pub fn analyze_trigger_dependencies(database: &Database) -> TriggerDependencyGraph {
        let mut graph = TriggerDependencyGraph {
            nodes: HashSet::new(),
            edges: Vec::new(),
            cascading_chains: Vec::new(),
            recursive_triggers: Vec::new(),
        };

        // Add all triggers and tables as nodes
        for table in database.get_all_tables() {
            graph.nodes.insert(format!("table:{}", table.name));

            for trigger in &table.triggers {
                graph.nodes.insert(format!("trigger:{}.{}", table.name, trigger.name));
            }
        }

        // Analyze trigger dependencies
        for table in database.get_all_tables() {
            for trigger in &table.triggers {
                // Check for table references in trigger definition
                let definition = trigger.definition.to_lowercase();

                for other_table in database.get_all_tables() {
                    if other_table.name != table.name && definition.contains(&other_table.name.to_lowercase()) {
                        graph.edges.push(TriggerDependencyEdge {
                            from: format!("trigger:{}.{}", table.name, trigger.name),
                            to: format!("table:{}", other_table.name),
                            dependency_type: TriggerDependencyType::TableAccess,
                            execution_order: 1,
                        });
                    }
                }
            }
        }

        // Detect cascading trigger chains
        graph.cascading_chains = Self::detect_cascading_chains(&graph);

        // Detect recursive triggers
        graph.recursive_triggers = Self::detect_recursive_triggers(&graph);

        graph
    }

    fn extract_business_logic(trigger: &Trigger) -> Vec<BusinessLogic> {
        let mut logic = Vec::new();
        let definition = trigger.definition.to_lowercase();

        // Detect audit trail logic
        if definition.contains("inserted") && definition.contains("updated") {
            logic.push(BusinessLogic {
                logic_type: BusinessLogicType::AuditTrail,
                description: "Maintains audit trail for data changes".to_string(),
                affected_columns: vec!["updated_at".to_string(), "updated_by".to_string()],
                conditions: vec!["Any update operation".to_string()],
                actions: vec!["Update timestamp and user tracking columns".to_string()],
            });
        }

        // Detect data validation logic
        if definition.contains("if") || definition.contains("case") {
            logic.push(BusinessLogic {
                logic_type: BusinessLogicType::DataValidation,
                description: "Validates data according to business rules".to_string(),
                affected_columns: vec!["validated_columns".to_string()], // Would be extracted from actual logic
                conditions: vec!["Data modification attempts".to_string()],
                actions: vec!["Validate and potentially reject invalid data".to_string()],
            });
        }

        // Detect cascade operations
        if definition.contains("delete") && definition.contains("from") {
            logic.push(BusinessLogic {
                logic_type: BusinessLogicType::CascadeOperations,
                description: "Performs cascading operations on related data".to_string(),
                affected_columns: vec!["foreign_key_columns".to_string()],
                conditions: vec!["Parent record deletion/modification".to_string()],
                actions: vec!["Cascade changes to related records".to_string()],
            });
        }

        // Detect computed columns
        if definition.contains("set") && definition.contains("=") {
            logic.push(BusinessLogic {
                logic_type: BusinessLogicType::ComputedColumns,
                description: "Maintains computed or derived column values".to_string(),
                affected_columns: vec!["computed_columns".to_string()],
                conditions: vec!["Data changes affecting computed values".to_string()],
                actions: vec!["Recalculate and update computed columns".to_string()],
            });
        }

        logic
    }

    fn extract_integrity_rules(trigger: &Trigger, table: &Table) -> Vec<DataIntegrityRule> {
        let mut rules = Vec::new();
        let definition = trigger.definition.to_lowercase();

        // Check constraints via triggers
        if definition.contains("if") && definition.contains("raiserror") || definition.contains("signal") {
            rules.push(DataIntegrityRule {
                rule_type: IntegrityRuleType::CheckConstraint,
                description: "Enforces check constraints through trigger logic".to_string(),
                enforced_columns: vec!["validated_columns".to_string()],
                validation_logic: "Custom validation logic in trigger body".to_string(),
            });
        }

        // Referential integrity enforcement
        if definition.contains("select") && definition.contains("where") && definition.contains("exists") {
            rules.push(DataIntegrityRule {
                rule_type: IntegrityRuleType::ReferentialIntegrity,
                description: "Enforces referential integrity rules".to_string(),
                enforced_columns: vec!["foreign_key_columns".to_string()],
                validation_logic: "Existence checks for referenced records".to_string(),
            });
        }

        rules
    }

    fn analyze_performance_impact(trigger: &Trigger, table: &Table, database: &Database) -> TriggerPerformanceImpact {
        let definition = trigger.definition.to_lowercase();

        // Estimate execution cost
        let execution_cost = if definition.contains("select") && definition.contains("join") {
            PerformanceCost::High
        } else if definition.contains("select") || definition.contains("update") {
            PerformanceCost::Medium
        } else {
            PerformanceCost::Low
        };

        // Estimate execution frequency based on events
        let frequency_estimate = if trigger.events.contains(&TriggerEvent::Update) {
            ExecutionFrequency::VeryFrequent
        } else if trigger.events.contains(&TriggerEvent::Insert) {
            ExecutionFrequency::Frequent
        } else {
            ExecutionFrequency::Occasional
        };

        // Analyze locking behavior
        let locking_behavior = if matches!(trigger.scope, TriggerScope::Row) {
            LockingBehavior::Minimal
        } else {
            LockingBehavior::Moderate
        };

        // Find cascading triggers
        let cascading_triggers = Self::find_cascading_triggers(trigger, table, database);

        // Generate optimization opportunities
        let optimization_opportunities = Self::generate_optimization_opportunities(trigger, definition);

        TriggerPerformanceImpact {
            execution_cost,
            frequency_estimate,
            locking_behavior,
            cascading_triggers,
            optimization_opportunities,
        }
    }

    fn analyze_security_implications(trigger: &Trigger) -> Vec<SecurityImplication> {
        let mut implications = Vec::new();
        let definition = trigger.definition.to_lowercase();

        // Check for privilege escalation
        if definition.contains("execute") && definition.contains("as") {
            implications.push(SecurityImplication {
                implication_type: SecurityImplicationType::PrivilegeEscalation,
                description: "Trigger may execute with elevated privileges".to_string(),
                risk_level: RiskLevel::High,
                mitigation_suggestions: vec![
                    "Review EXECUTE AS clause".to_string(),
                    "Ensure minimal required permissions".to_string(),
                    "Audit privilege usage".to_string(),
                ],
            });
        }

        // Check for SQL injection risks
        if definition.contains("exec(") || definition.contains("execute(") {
            implications.push(SecurityImplication {
                implication_type: SecurityImplicationType::InjectionRisk,
                description: "Dynamic SQL execution may be vulnerable to injection".to_string(),
                risk_level: RiskLevel::Critical,
                mitigation_suggestions: vec![
                    "Use parameterized queries".to_string(),
                    "Validate input parameters".to_string(),
                    "Implement proper escaping".to_string(),
                ],
            });
        }

        // Check for audit bypass
        if definition.contains("disable trigger") || definition.contains("alter trigger") {
            implications.push(SecurityImplication {
                implication_type: SecurityImplicationType::AuditBypass,
                description: "Trigger may allow bypassing audit mechanisms".to_string(),
                risk_level: RiskLevel::Medium,
                mitigation_suggestions: vec![
                    "Implement trigger protection".to_string(),
                    "Log trigger state changes".to_string(),
                    "Regular security audits".to_string(),
                ],
            });
        }

        implications
    }

    fn analyze_issues(trigger: &Trigger, table: &Table, database: &Database) -> Vec<TriggerIssue> {
        let mut issues = Vec::new();
        let definition = trigger.definition.to_lowercase();

        // Check for recursive triggers
        if Self::is_recursive_trigger(trigger, table, database) {
            issues.push(TriggerIssue {
                issue_type: TriggerIssueType::RecursiveTrigger,
                severity: IssueSeverity::High,
                description: "Trigger may cause recursive execution".to_string(),
                impact: "Performance degradation and potential infinite loops".to_string(),
                suggested_fix: Some("Add RECURSIVE_TRIGGERS check or restructure logic".to_string()),
            });
        }

        // Check for missing error handling
        if !definition.contains("begin try") && !definition.contains("exception") && !definition.contains("raiserror") {
            issues.push(TriggerIssue {
                issue_type: TriggerIssueType::MissingErrorHandling,
                severity: IssueSeverity::Medium,
                description: "Trigger lacks proper error handling".to_string(),
                impact: "Unpredictable behavior on errors".to_string(),
                suggested_fix: Some("Add TRY-CATCH blocks or exception handlers".to_string()),
            });
        }

        // Check for performance problems
        if definition.contains("cursor") || definition.contains("while") {
            issues.push(TriggerIssue {
                issue_type: TriggerIssueType::PerformanceProblem,
                severity: IssueSeverity::High,
                description: "Trigger uses cursors or loops which are slow".to_string(),
                impact: "Significant performance degradation".to_string(),
                suggested_fix: Some("Replace with set-based operations".to_string()),
            });
        }

        // Check for logic errors
        if definition.contains("select") && trigger.events.contains(&TriggerEvent::Insert) &&
           !definition.contains("inserted") {
            issues.push(TriggerIssue {
                issue_type: TriggerIssueType::LogicError,
                severity: IssueSeverity::Medium,
                description: "INSERT trigger references base table instead of INSERTED".to_string(),
                impact: "Incorrect trigger behavior".to_string(),
                suggested_fix: Some("Use INSERTED table for INSERT triggers".to_string()),
            });
        }

        issues
    }

    fn generate_recommendations(
        trigger: &Trigger,
        issues: &[TriggerIssue],
        performance_impact: &TriggerPerformanceImpact,
    ) -> Vec<TriggerRecommendation> {
        let mut recommendations = Vec::new();

        // Recommendations based on issues
        for issue in issues {
            match issue.issue_type {
                TriggerIssueType::MissingErrorHandling => {
                    recommendations.push(TriggerRecommendation {
                        recommendation_type: TriggerRecommendationType::AddErrorHandling,
                        description: "Add comprehensive error handling to trigger".to_string(),
                        benefit: "Prevents silent failures and provides better debugging".to_string(),
                        effort: EffortLevel::Medium,
                        sql_commands: vec![
                            "-- Add error handling structure".to_string(),
                            "BEGIN TRY".to_string(),
                            "    -- Trigger logic here".to_string(),
                            "END TRY".to_string(),
                            "BEGIN CATCH".to_string(),
                            "    -- Error handling logic".to_string(),
                            "END CATCH".to_string(),
                        ],
                        expected_improvement: Some(0.1),
                    });
                }
                TriggerIssueType::PerformanceProblem => {
                    recommendations.push(TriggerRecommendation {
                        recommendation_type: TriggerRecommendationType::PerformanceTuning,
                        description: "Optimize trigger performance by removing cursors and loops".to_string(),
                        benefit: "Significant performance improvement".to_string(),
                        effort: EffortLevel::High,
                        sql_commands: vec![
                            "-- Replace cursor-based logic with set-based operations".to_string(),
                            "-- Example: Use UPDATE with JOIN instead of cursor".to_string(),
                        ],
                        expected_improvement: Some(0.8),
                    });
                }
                _ => {}
            }
        }

        // Performance-based recommendations
        if matches!(performance_impact.execution_cost, PerformanceCost::High) {
            recommendations.push(TriggerRecommendation {
                recommendation_type: TriggerRecommendationType::OptimizeConditions,
                description: "Add conditions to prevent unnecessary trigger execution".to_string(),
                benefit: "Reduces trigger execution overhead".to_string(),
                effort: EffortLevel::Low,
                sql_commands: vec![
                    "-- Add condition to trigger".to_string(),
                    "IF EXISTS (SELECT 1 FROM INSERTED WHERE condition_column = 'value')".to_string(),
                    "BEGIN".to_string(),
                    "    -- Trigger logic here".to_string(),
                    "END".to_string(),
                ],
                expected_improvement: Some(0.6),
            });
        }

        // Security recommendations
        recommendations.push(TriggerRecommendation {
            recommendation_type: TriggerRecommendationType::ReviewSecurity,
            description: "Conduct security review of trigger implementation".to_string(),
            benefit: "Identifies and mitigates security vulnerabilities".to_string(),
            effort: EffortLevel::Medium,
            sql_commands: vec![
                "-- Security review checklist:".to_string(),
                "-- 1. Check for SQL injection vulnerabilities".to_string(),
                "-- 2. Verify privilege escalation risks".to_string(),
                "-- 3. Audit data access patterns".to_string(),
            ],
            expected_improvement: None,
        });

        recommendations
    }

    fn find_cascading_triggers(trigger: &Trigger, table: &Table, database: &Database) -> Vec<String> {
        let mut cascading = Vec::new();

        // Check if trigger modifies other tables that have triggers
        let definition = trigger.definition.to_lowercase();

        for other_table in database.get_all_tables() {
            if definition.contains(&other_table.name.to_lowercase()) && !other_table.triggers.is_empty() {
                for other_trigger in &other_table.triggers {
                    cascading.push(format!("{}.{}", other_table.name, other_trigger.name));
                }
            }
        }

        cascading
    }

    fn generate_optimization_opportunities(trigger: &Trigger, definition: String) -> Vec<PerformanceOptimization> {
        let mut opportunities = Vec::new();

        if definition.contains("select * from") {
            opportunities.push(PerformanceOptimization {
                optimization_type: TriggerOptimizationType::ConditionOptimization,
                description: "Replace SELECT * with specific column selection".to_string(),
                expected_improvement: 0.2,
                implementation_effort: EffortLevel::Low,
            });
        }

        if definition.contains("cursor") {
            opportunities.push(PerformanceOptimization {
                optimization_type: TriggerOptimizationType::BulkProcessing,
                description: "Replace cursor operations with set-based operations".to_string(),
                expected_improvement: 0.7,
                implementation_effort: EffortLevel::High,
            });
        }

        opportunities
    }

    fn is_recursive_trigger(trigger: &Trigger, table: &Table, database: &Database) -> bool {
        let definition = trigger.definition.to_lowercase();

        // Check if trigger references its own table in a way that could cause recursion
        definition.contains(&table.name.to_lowercase()) &&
        (definition.contains("insert") || definition.contains("update") || definition.contains("delete"))
    }

    fn detect_cascading_chains(graph: &TriggerDependencyGraph) -> Vec<Vec<String>> {
        // Simple chain detection - in practice, would use proper graph algorithms
        let mut chains = Vec::new();

        // This is a placeholder implementation
        // A real implementation would use DFS or topological analysis

        chains
    }

    fn detect_recursive_triggers(graph: &TriggerDependencyGraph) -> Vec<String> {
        let mut recursive = Vec::new();

        // Check for self-referencing triggers
        for edge in &graph.edges {
            if edge.from == edge.to {
                recursive.push(edge.from.clone());
            }
        }

        recursive
    }
}

pub struct TriggerDocumentationService;

impl TriggerDocumentationService {
    pub fn generate_trigger_documentation(analysis: &TriggerAnalysis) -> String {
        let mut doc = format!("### Trigger: {}\n\n", analysis.trigger_name);

        doc.push_str(&format!("**Table:** {}\n", analysis.table_name));
        doc.push_str(&format!("**Timing:** {}\n", Self::timing_name(&analysis.timing)));
        doc.push_str(&format!("**Events:** {}\n", analysis.events.iter().map(|e| Self::event_name(e)).collect::<Vec<_>>().join(", ")));
        doc.push_str(&format!("**Scope:** {}\n", Self::scope_name(&analysis.scope)));

        if let Some(description) = &analysis.metadata.description {
            doc.push_str(&format!("**Description:** {}\n", description));
        }

        doc.push_str(&format!("**Performance Cost:** {}\n", Self::cost_name(&analysis.performance_impact.execution_cost)));
        doc.push_str(&format!("**Execution Frequency:** {}\n", Self::frequency_name(&analysis.performance_impact.frequency_estimate)));

        if !analysis.business_logic.is_empty() {
            doc.push_str("\n#### Business Logic\n\n");
            for logic in &analysis.business_logic {
                doc.push_str(&format!("- **{}:** {}\n", Self::logic_type_name(&logic.logic_type), logic.description));

                if !logic.affected_columns.is_empty() {
                    doc.push_str(&format!("  - **Affected Columns:** {}\n", logic.affected_columns.join(", ")));
                }

                if !logic.conditions.is_empty() {
                    doc.push_str("  - **Conditions:**\n");
                    for condition in &logic.conditions {
                        doc.push_str(&format!("    - {}\n", condition));
                    }
                }

                if !logic.actions.is_empty() {
                    doc.push_str("  - **Actions:**\n");
                    for action in &logic.actions {
                        doc.push_str(&format!("    - {}\n", action));
                    }
                }
            }
        }

        if !analysis.data_integrity_rules.is_empty() {
            doc.push_str("\n#### Data Integrity Rules\n\n");
            for rule in &analysis.data_integrity_rules {
                doc.push_str(&format!("- **{}:** {}\n", Self::rule_type_name(&rule.rule_type), rule.description));
                doc.push_str(&format!("  - **Enforced Columns:** {}\n", rule.enforced_columns.join(", ")));
                doc.push_str(&format!("  - **Validation Logic:** {}\n", rule.validation_logic));
            }
        }

        if !analysis.security_implications.is_empty() {
            doc.push_str("\n#### Security Implications\n\n");
            for implication in &analysis.security_implications {
                let risk_icon = match implication.risk_level {
                    RiskLevel::Critical => "🚨",
                    RiskLevel::High => "⚠️",
                    RiskLevel::Medium => "⚡",
                    RiskLevel::Low => "ℹ️",
                };

                doc.push_str(&format!("- {} **{}:** {}\n", risk_icon, Self::implication_type_name(&implication.implication_type), implication.description));

                if !implication.mitigation_suggestions.is_empty() {
                    doc.push_str("  - **Mitigations:**\n");
                    for suggestion in &implication.mitigation_suggestions {
                        doc.push_str(&format!("    - {}\n", suggestion));
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

    fn timing_name(timing: &TriggerTiming) -> &'static str {
        match timing {
            TriggerTiming::Before => "BEFORE",
            TriggerTiming::After => "AFTER",
            TriggerTiming::InsteadOf => "INSTEAD OF",
        }
    }

    fn event_name(event: &TriggerEvent) -> &'static str {
        match event {
            TriggerEvent::Insert => "INSERT",
            TriggerEvent::Update => "UPDATE",
            TriggerEvent::Delete => "DELETE",
        }
    }

    fn scope_name(scope: &TriggerScope) -> &'static str {
        match scope {
            TriggerScope::Row => "ROW",
            TriggerScope::Statement => "STATEMENT",
        }
    }

    fn cost_name(cost: &PerformanceCost) -> &'static str {
        match cost {
            PerformanceCost::Low => "Low",
            PerformanceCost::Medium => "Medium",
            PerformanceCost::High => "High",
            PerformanceCost::VeryHigh => "Very High",
        }
    }

    fn frequency_name(frequency: &ExecutionFrequency) -> &'static str {
        match frequency {
            ExecutionFrequency::Rare => "Rare",
            ExecutionFrequency::Occasional => "Occasional",
            ExecutionFrequency::Frequent => "Frequent",
            ExecutionFrequency::VeryFrequent => "Very Frequent",
        }
    }

    fn logic_type_name(logic_type: &BusinessLogicType) -> &'static str {
        match logic_type {
            BusinessLogicType::AuditTrail => "Audit Trail",
            BusinessLogicType::DataValidation => "Data Validation",
            BusinessLogicType::CascadeOperations => "Cascade Operations",
            BusinessLogicType::ComputedColumns => "Computed Columns",
            BusinessLogicType::Notifications => "Notifications",
            BusinessLogicType::WorkflowManagement => "Workflow Management",
            BusinessLogicType::CustomLogic => "Custom Logic",
        }
    }

    fn rule_type_name(rule_type: &IntegrityRuleType) -> &'static str {
        match rule_type {
            IntegrityRuleType::CheckConstraint => "Check Constraint",
            IntegrityRuleType::ReferentialIntegrity => "Referential Integrity",
            IntegrityRuleType::DomainValidation => "Domain Validation",
            IntegrityRuleType::BusinessRule => "Business Rule",
            IntegrityRuleType::DataConsistency => "Data Consistency",
        }
    }

    fn implication_type_name(implication_type: &SecurityImplicationType) -> &'static str {
        match implication_type {
            SecurityImplicationType::PrivilegeEscalation => "Privilege Escalation",
            SecurityImplicationType::DataExposure => "Data Exposure",
            SecurityImplicationType::InjectionRisk => "Injection Risk",
            SecurityImplicationType::DenialOfService => "Denial of Service",
            SecurityImplicationType::AuditBypass => "Audit Bypass",
        }
    }

    fn issue_type_name(issue_type: &TriggerIssueType) -> &'static str {
        match issue_type {
            TriggerIssueType::RecursiveTrigger => "Recursive Trigger",
            TriggerIssueType::MissingErrorHandling => "Missing Error Handling",
            TriggerIssueType::PerformanceProblem => "Performance Problem",
            TriggerIssueType::LogicError => "Logic Error",
            TriggerIssueType::SecurityRisk => "Security Risk",
            TriggerIssueType::MaintainabilityIssue => "Maintainability Issue",
            TriggerIssueType::ConflictingTrigger => "Conflicting Trigger",
        }
    }

    fn recommendation_type_name(rec_type: &TriggerRecommendationType) -> &'static str {
        match rec_type {
            TriggerRecommendationType::AddErrorHandling => "Add Error Handling",
            TriggerRecommendationType::OptimizeConditions => "Optimize Conditions",
            TriggerRecommendationType::ConsolidateTriggers => "Consolidate Triggers",
            TriggerRecommendationType::AddLogging => "Add Logging",
            TriggerRecommendationType::ImplementValidation => "Implement Validation",
            TriggerRecommendationType::ReviewSecurity => "Review Security",
            TriggerRecommendationType::PerformanceTuning => "Performance Tuning",
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
pub struct TriggerDependencyGraph {
    pub nodes: HashSet<String>,
    pub edges: Vec<TriggerDependencyEdge>,
    pub cascading_chains: Vec<Vec<String>>,
    pub recursive_triggers: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TriggerDependencyEdge {
    pub from: String,
    pub to: String,
    pub dependency_type: TriggerDependencyType,
    pub execution_order: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TriggerDependencyType {
    TableAccess,
    TriggerCascade,
    FunctionCall,
    SequenceReference,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trigger_business_logic_extraction() {
        let trigger = Trigger {
            name: "audit_trigger".to_string(),
            table_name: "users".to_string(),
            timing: TriggerTiming::After,
            events: vec![TriggerEvent::Update],
            scope: TriggerScope::Row,
            definition: r#"
                UPDATE users
                SET updated_at = GETDATE(),
                    updated_by = SYSTEM_USER
                WHERE id = (SELECT id FROM INSERTED)
            "#.to_string(),
            metadata: TriggerMetadata::default(),
        };

        let business_logic = TriggerAnalysisService::extract_business_logic(&trigger);

        assert!(business_logic.iter().any(|logic| matches!(logic.logic_type, BusinessLogicType::AuditTrail)));
        assert!(business_logic.iter().any(|logic| matches!(logic.logic_type, BusinessLogicType::ComputedColumns)));
    }

    #[test]
    fn test_performance_impact_analysis() {
        let trigger = Trigger {
            name: "complex_trigger".to_string(),
            table_name: "orders".to_string(),
            timing: TriggerTiming::After,
            events: vec![TriggerEvent::Insert],
            scope: TriggerScope::Row,
            definition: r#"
                DECLARE @cursor CURSOR
                SET @cursor = CURSOR FOR
                    SELECT id FROM INSERTED

                OPEN @cursor
                FETCH NEXT FROM @cursor INTO @id

                WHILE @@FETCH_STATUS = 0
                BEGIN
                    UPDATE inventory SET quantity = quantity - 1 WHERE product_id = @id
                    FETCH NEXT FROM @cursor INTO @id
                END

                CLOSE @cursor
                DEALLOCATE @cursor
            "#.to_string(),
            metadata: TriggerMetadata::default(),
        };

        let table = Table::new("orders");
        let database = Database::new("test_db");

        let performance_impact = TriggerAnalysisService::analyze_performance_impact(&trigger, &table, &database);

        assert_eq!(performance_impact.execution_cost, PerformanceCost::High);
    }

    #[test]
    fn test_security_implications_analysis() {
        let trigger = Trigger {
            name: "risky_trigger".to_string(),
            table_name: "users".to_string(),
            timing: TriggerTiming::Before,
            events: vec![TriggerEvent::Insert],
            scope: TriggerScope::Row,
            definition: r#"
                EXEC('SELECT * FROM ' + @table_name)
            "#.to_string(),
            metadata: TriggerMetadata::default(),
        };

        let security_implications = TriggerAnalysisService::analyze_security_implications(&trigger);

        assert!(security_implications.iter().any(|imp| matches!(imp.implication_type, SecurityImplicationType::InjectionRisk)));
        assert!(security_implications.iter().any(|imp| imp.risk_level == RiskLevel::Critical));
    }
}
