use std::collections::{HashMap, HashSet};
use serde::{Deserialize, Serialize};

use crate::core::domain::entity::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PermissionAnalysis {
    pub object_name: String,
    pub object_type: ObjectType,
    pub grantee: String,
    pub permission_type: PermissionType,
    pub grantor: Option<String>,
    pub is_grantable: bool,
    pub security_level: SecurityLevel,
    pub access_patterns: Vec<AccessPattern>,
    pub risk_assessment: RiskAssessment,
    pub recommendations: Vec<PermissionRecommendation>,
    pub metadata: PermissionMetadata,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ObjectType {
    Table,
    View,
    Procedure,
    Function,
    Schema,
    Database,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PermissionType {
    Select,
    Insert,
    Update,
    Delete,
    Execute,
    Alter,
    Create,
    Drop,
    Grant,
    Revoke,
    All,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SecurityLevel {
    Public,
    Internal,
    Confidential,
    Restricted,
    Classified,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AccessPattern {
    pub pattern_type: AccessPatternType,
    pub frequency: AccessFrequency,
    pub typical_users: Vec<String>,
    pub business_justification: String,
    pub data_sensitivity: DataSensitivity,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AccessPatternType {
    ReadOnly,
    ReadWrite,
    Administrative,
    Maintenance,
    Audit,
    Emergency,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AccessFrequency {
    Constant,
    Frequent,
    Occasional,
    Rare,
    Never,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DataSensitivity {
    Public,
    Internal,
    Sensitive,
    Confidential,
    Restricted,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RiskAssessment {
    pub overall_risk: RiskLevel,
    pub risk_factors: Vec<RiskFactor>,
    pub compliance_status: ComplianceStatus,
    pub audit_requirements: Vec<String>,
    pub mitigation_strategies: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RiskFactor {
    pub factor_type: RiskFactorType,
    pub severity: RiskLevel,
    pub description: String,
    pub evidence: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RiskFactorType {
    OverPrivileged,
    DirectObjectAccess,
    MissingAudit,
    WeakAuthentication,
    DataExposure,
    PrivilegeEscalation,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ComplianceStatus {
    Compliant,
    NonCompliant,
    UnderReview,
    Exempt,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PermissionRecommendation {
    pub recommendation_type: RecommendationType,
    pub description: String,
    pub urgency: UrgencyLevel,
    pub impact: ImpactLevel,
    pub sql_commands: Vec<String>,
    pub expected_benefit: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RecommendationType {
    RevokePermission,
    GrantLimitedPermission,
    ImplementRowLevelSecurity,
    AddAuditLogging,
    CreateSecurityPolicy,
    ReviewAccessPatterns,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UrgencyLevel {
    Immediate,
    High,
    Medium,
    Low,
    Optional,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ImpactLevel {
    Critical,
    High,
    Medium,
    Low,
    Minimal,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PermissionMetadata {
    pub granted_date: Option<String>,
    pub last_used: Option<String>,
    pub review_date: Option<String>,
    pub business_owner: Option<String>,
    pub technical_owner: Option<String>,
    pub compliance_framework: Option<String>,
    pub tags: Vec<String>,
    pub custom_properties: HashMap<String, String>,
}

pub struct PermissionAnalysisService;

impl PermissionAnalysisService {
    pub fn analyze_permission(permission: &Permission, database: &Database) -> PermissionAnalysis {
        let mut recommendations = Vec::new();

        let risk_assessment = Self::assess_risk(permission, database);
        let access_patterns = Self::analyze_access_patterns(permission, database);
        let security_level = Self::determine_security_level(permission, database);

        if risk_assessment.overall_risk >= RiskLevel::High {
            recommendations.extend(Self::generate_risk_mitigations(&risk_assessment));
        }

        recommendations.extend(Self::generate_general_recommendations(permission, &access_patterns));

        PermissionAnalysis {
            object_name: permission.object_name.clone(),
            object_type: permission.object_type.clone(),
            grantee: permission.grantee.clone(),
            permission_type: permission.permission_type.clone(),
            grantor: permission.grantor.clone(),
            is_grantable: permission.is_grantable,
            security_level,
            access_patterns,
            risk_assessment,
            recommendations,
            metadata: PermissionMetadata {
                granted_date: permission.metadata.granted_date.clone(),
                last_used: permission.metadata.last_used.clone(),
                review_date: permission.metadata.review_date.clone(),
                business_owner: permission.metadata.business_owner.clone(),
                technical_owner: permission.metadata.technical_owner.clone(),
                compliance_framework: permission.metadata.compliance_framework.clone(),
                tags: permission.metadata.tags.clone(),
                custom_properties: permission.metadata.custom_properties.clone(),
            },
        }
    }

    pub fn analyze_database_permissions(database: &Database) -> DatabasePermissionReport {
        let mut permission_analyses = Vec::new();
        let mut security_score = 0.0;
        let mut risk_distribution = HashMap::new();
        let mut privileged_users = HashSet::new();

        for permission in database.get_all_permissions() {
            let analysis = Self::analyze_permission(permission, database);
            permission_analyses.push(analysis.clone());

            // Update security score
            security_score += Self::calculate_permission_score(&analysis);

            // Track risk distribution
            *risk_distribution.entry(analysis.risk_assessment.overall_risk.clone()).or_insert(0) += 1;

            // Track privileged users
            if matches!(analysis.permission_type, PermissionType::All | PermissionType::Alter | PermissionType::Drop) {
                privileged_users.insert(analysis.grantee.clone());
            }
        }

        let average_security_score = if !permission_analyses.is_empty() {
            security_score / permission_analyses.len() as f64
        } else {
            0.0
        };

        DatabasePermissionReport {
            total_permissions: permission_analyses.len(),
            average_security_score,
            risk_distribution,
            privileged_users: privileged_users.into_iter().collect(),
            critical_permissions: permission_analyses.iter()
                .filter(|p| p.risk_assessment.overall_risk == RiskLevel::Critical)
                .map(|p| p.object_name.clone())
                .collect(),
            permission_analyses,
            security_recommendations: Self::generate_database_recommendations(&permission_analyses),
        }
    }

    fn assess_risk(permission: &Permission, database: &Database) -> RiskAssessment {
        let mut risk_factors = Vec::new();
        let mut overall_risk = RiskLevel::Low;

        // Check for over-privileged access
        if matches!(permission.permission_type, PermissionType::All) {
            risk_factors.push(RiskFactor {
                factor_type: RiskFactorType::OverPrivileged,
                severity: RiskLevel::High,
                description: "User has ALL permissions on object".to_string(),
                evidence: format!("Grantee '{}' has unrestricted access to '{}'", permission.grantee, permission.object_name),
            });
            overall_risk = RiskLevel::High;
        }

        // Check for direct object access without proper controls
        if matches!(permission.object_type, ObjectType::Table) &&
           matches!(permission.permission_type, PermissionType::Select | PermissionType::Update | PermissionType::Delete) &&
           Self::has_sensitive_data(&permission.object_name, database) {
            risk_factors.push(RiskFactor {
                factor_type: RiskFactorType::DataExposure,
                severity: RiskLevel::Medium,
                description: "Direct access to sensitive data without row-level security".to_string(),
                evidence: format!("Table '{}' contains sensitive data but lacks row-level access controls", permission.object_name),
            });
            if overall_risk == RiskLevel::Low {
                overall_risk = RiskLevel::Medium;
            }
        }

        // Check for grantable permissions
        if permission.is_grantable {
            risk_factors.push(RiskFactor {
                factor_type: RiskFactorType::PrivilegeEscalation,
                severity: RiskLevel::Medium,
                description: "User can grant permissions to others".to_string(),
                evidence: format!("Grantee '{}' can propagate permissions on '{}'", permission.grantee, permission.object_name),
            });
            if overall_risk == RiskLevel::Low {
                overall_risk = RiskLevel::Medium;
            }
        }

        // Determine compliance status
        let compliance_status = Self::check_compliance(permission, &risk_factors);

        // Generate mitigation strategies
        let mitigation_strategies = Self::generate_mitigations(&risk_factors);

        // Generate audit requirements
        let audit_requirements = Self::generate_audit_requirements(&risk_factors);

        RiskAssessment {
            overall_risk,
            risk_factors,
            compliance_status,
            audit_requirements,
            mitigation_strategies,
        }
    }

    fn analyze_access_patterns(permission: &Permission, database: &Database) -> Vec<AccessPattern> {
        let mut patterns = Vec::new();

        // Analyze based on permission type and object type
        match (&permission.permission_type, &permission.object_type) {
            (PermissionType::Select, ObjectType::Table) => {
                patterns.push(AccessPattern {
                    pattern_type: AccessPatternType::ReadOnly,
                    frequency: AccessFrequency::Frequent,
                    typical_users: vec!["application_users".to_string(), "analysts".to_string()],
                    business_justification: "Data retrieval for business operations".to_string(),
                    data_sensitivity: Self::assess_data_sensitivity(&permission.object_name, database),
                });
            }
            (PermissionType::Execute, ObjectType::Procedure) => {
                patterns.push(AccessPattern {
                    pattern_type: AccessPatternType::ReadWrite,
                    frequency: AccessFrequency::Frequent,
                    typical_users: vec!["application_services".to_string()],
                    business_justification: "Execute stored procedures for business logic".to_string(),
                    data_sensitivity: DataSensitivity::Internal,
                });
            }
            (PermissionType::Alter, _) => {
                patterns.push(AccessPattern {
                    pattern_type: AccessPatternType::Administrative,
                    frequency: AccessFrequency::Rare,
                    typical_users: vec!["database_administrators".to_string(), "developers".to_string()],
                    business_justification: "Schema modifications and maintenance".to_string(),
                    data_sensitivity: DataSensitivity::Restricted,
                });
            }
            _ => {
                patterns.push(AccessPattern {
                    pattern_type: AccessPatternType::Maintenance,
                    frequency: AccessFrequency::Occasional,
                    typical_users: vec!["support_staff".to_string()],
                    business_justification: "General database maintenance".to_string(),
                    data_sensitivity: DataSensitivity::Internal,
                });
            }
        }

        patterns
    }

    fn determine_security_level(permission: &Permission, database: &Database) -> SecurityLevel {
        // Determine security level based on data sensitivity and permission type
        let data_sensitivity = Self::assess_data_sensitivity(&permission.object_name, database);

        match (&permission.permission_type, &data_sensitivity) {
            (PermissionType::Select, DataSensitivity::Public) => SecurityLevel::Public,
            (PermissionType::Select, DataSensitivity::Internal) => SecurityLevel::Internal,
            (PermissionType::Select, DataSensitivity::Sensitive) => SecurityLevel::Confidential,
            (PermissionType::Select, DataSensitivity::Confidential) => SecurityLevel::Restricted,
            (PermissionType::Select, DataSensitivity::Restricted) => SecurityLevel::Classified,
            (PermissionType::All | PermissionType::Alter | PermissionType::Drop, _) => SecurityLevel::Classified,
            _ => SecurityLevel::Internal,
        }
    }

    fn generate_risk_mitigations(risk_assessment: &RiskAssessment) -> Vec<PermissionRecommendation> {
        let mut recommendations = Vec::new();

        for risk_factor in &risk_assessment.risk_factors {
            match risk_factor.factor_type {
                RiskFactorType::OverPrivileged => {
                    recommendations.push(PermissionRecommendation {
                        recommendation_type: RecommendationType::RevokePermission,
                        description: "Replace ALL permissions with specific granular permissions".to_string(),
                        urgency: UrgencyLevel::High,
                        impact: ImpactLevel::High,
                        sql_commands: vec![
                            "-- Analyze current usage patterns first".to_string(),
                            "-- Then replace with specific permissions".to_string(),
                        ],
                        expected_benefit: "Reduced attack surface and better access control".to_string(),
                    });
                }
                RiskFactorType::DataExposure => {
                    recommendations.push(PermissionRecommendation {
                        recommendation_type: RecommendationType::ImplementRowLevelSecurity,
                        description: "Implement row-level security policies".to_string(),
                        urgency: UrgencyLevel::High,
                        impact: ImpactLevel::Critical,
                        sql_commands: vec![
                            "-- Create security policy function".to_string(),
                            "-- Apply policy to table".to_string(),
                        ],
                        expected_benefit: "Prevents unauthorized data access at row level".to_string(),
                    });
                }
                RiskFactorType::PrivilegeEscalation => {
                    recommendations.push(PermissionRecommendation {
                        recommendation_type: RecommendationType::ReviewAccessPatterns,
                        description: "Review and limit GRANT permissions".to_string(),
                        urgency: UrgencyLevel::Medium,
                        impact: ImpactLevel::Medium,
                        sql_commands: vec![
                            "-- Audit current permission grants".to_string(),
                            "-- Remove unnecessary WITH GRANT OPTION".to_string(),
                        ],
                        expected_benefit: "Prevents unauthorized permission propagation".to_string(),
                    });
                }
                _ => {}
            }
        }

        recommendations
    }

    fn generate_general_recommendations(permission: &Permission, access_patterns: &[AccessPattern]) -> Vec<PermissionRecommendation> {
        let mut recommendations = Vec::new();

        // Add audit logging recommendation
        if !matches!(permission.object_type, ObjectType::View) &&
           matches!(permission.permission_type, PermissionType::Update | PermissionType::Delete | PermissionType::Insert) {
            recommendations.push(PermissionRecommendation {
                recommendation_type: RecommendationType::AddAuditLogging,
                description: "Implement audit logging for data modifications".to_string(),
                urgency: UrgencyLevel::Medium,
                impact: ImpactLevel::Medium,
                sql_commands: vec![
                    "-- Create audit trigger".to_string(),
                    "-- Log changes to audit table".to_string(),
                ],
                expected_benefit: "Provides change tracking and compliance".to_string(),
            });
        }

        // Regular review recommendation
        recommendations.push(PermissionRecommendation {
            recommendation_type: RecommendationType::ReviewAccessPatterns,
            description: "Schedule regular permission reviews".to_string(),
            urgency: UrgencyLevel::Low,
            impact: ImpactLevel::Low,
            sql_commands: vec![
                "-- Review permission usage and necessity".to_string(),
                "-- Document business justification".to_string(),
            ],
            expected_benefit: "Maintains security posture and compliance".to_string(),
        });

        recommendations
    }

    fn has_sensitive_data(object_name: &str, database: &Database) -> bool {
        // Check if table contains sensitive data based on column names and types
        if let Some(table) = database.get_table(None, object_name) {
            table.columns.iter().any(|col|
                col.name.to_lowercase().contains("password") ||
                col.name.to_lowercase().contains("ssn") ||
                col.name.to_lowercase().contains("credit_card") ||
                col.name.to_lowercase().contains("salary") ||
                col.name.to_lowercase().contains("medical")
            )
        } else {
            false
        }
    }

    fn assess_data_sensitivity(object_name: &str, database: &Database) -> DataSensitivity {
        if Self::has_sensitive_data(object_name, database) {
            DataSensitivity::Confidential
        } else if object_name.to_lowercase().contains("financial") ||
                  object_name.to_lowercase().contains("hr") ||
                  object_name.to_lowercase().contains("personnel") {
            DataSensitivity::Sensitive
        } else {
            DataSensitivity::Internal
        }
    }

    fn check_compliance(permission: &Permission, risk_factors: &[RiskFactor]) -> ComplianceStatus {
        // Simplified compliance check - in practice, this would check against specific frameworks
        if risk_factors.iter().any(|rf| rf.severity == RiskLevel::Critical) {
            ComplianceStatus::NonCompliant
        } else if risk_factors.iter().any(|rf| rf.severity == RiskLevel::High) {
            ComplianceStatus::UnderReview
        } else {
            ComplianceStatus::Compliant
        }
    }

    fn generate_mitigations(risk_factors: &[RiskFactor]) -> Vec<String> {
        let mut mitigations = Vec::new();

        for factor in risk_factors {
            match factor.factor_type {
                RiskFactorType::OverPrivileged => {
                    mitigations.push("Implement principle of least privilege".to_string());
                    mitigations.push("Use role-based access control".to_string());
                }
                RiskFactorType::DataExposure => {
                    mitigations.push("Implement row-level security".to_string());
                    mitigations.push("Use views to restrict data access".to_string());
                }
                RiskFactorType::PrivilegeEscalation => {
                    mitigations.push("Remove GRANT OPTION from permissions".to_string());
                    mitigations.push("Implement approval workflow for permission grants".to_string());
                }
                _ => {}
            }
        }

        mitigations
    }

    fn generate_audit_requirements(risk_factors: &[RiskFactor]) -> Vec<String> {
        let mut requirements = Vec::new();

        if risk_factors.iter().any(|rf| matches!(rf.factor_type, RiskFactorType::DataExposure)) {
            requirements.push("Daily audit log review".to_string());
            requirements.push("Monthly access pattern analysis".to_string());
        }

        if risk_factors.iter().any(|rf| matches!(rf.factor_type, RiskFactorType::PrivilegeEscalation)) {
            requirements.push("Permission change auditing".to_string());
            requirements.push("Quarterly privilege review".to_string());
        }

        if requirements.is_empty() {
            requirements.push("Annual permission review".to_string());
        }

        requirements
    }

    fn calculate_permission_score(analysis: &PermissionAnalysis) -> f64 {
        let base_score = match analysis.risk_assessment.overall_risk {
            RiskLevel::Low => 0.8,
            RiskLevel::Medium => 0.6,
            RiskLevel::High => 0.4,
            RiskLevel::Critical => 0.2,
        };

        let grantable_penalty = if analysis.is_grantable { 0.1 } else { 0.0 };

        (base_score - grantable_penalty).max(0.0)
    }

    fn generate_database_recommendations(analyses: &[PermissionAnalysis]) -> Vec<String> {
        let mut recommendations = Vec::new();

        let critical_permissions = analyses.iter()
            .filter(|a| a.risk_assessment.overall_risk == RiskLevel::Critical)
            .count();

        if critical_permissions > 0 {
            recommendations.push(format!("Address {} critical permission risks immediately", critical_permissions));
        }

        let privileged_users: HashSet<_> = analyses.iter()
            .filter(|a| matches!(a.permission_type, PermissionType::All | PermissionType::Alter))
            .map(|a| &a.grantee)
            .collect();

        if privileged_users.len() > 5 {
            recommendations.push("Too many users with elevated privileges - implement role-based access".to_string());
        }

        if analyses.iter().any(|a| a.is_grantable) {
            recommendations.push("Review users with GRANT permissions to prevent privilege escalation".to_string());
        }

        recommendations
    }
}

pub struct PermissionDocumentationService;

impl PermissionDocumentationService {
    pub fn generate_permission_documentation(analysis: &PermissionAnalysis) -> String {
        let mut doc = format!("### Permission: {} on {}\n\n", analysis.grantee, analysis.object_name);

        doc.push_str(&format!("**Object Type:** {}\n", Self::object_type_name(&analysis.object_type)));
        doc.push_str(&format!("**Permission Type:** {}\n", Self::permission_type_name(&analysis.permission_type)));
        doc.push_str(&format!("**Security Level:** {}\n", Self::security_level_name(&analysis.security_level)));
        doc.push_str(&format!("**Risk Level:** {}\n", Self::risk_level_name(&analysis.risk_assessment.overall_risk)));
        doc.push_str(&format!("**Compliance Status:** {}\n", Self::compliance_status_name(&analysis.risk_assessment.compliance_status)));

        if let Some(grantor) = &analysis.grantor {
            doc.push_str(&format!("**Granted By:** {}\n", grantor));
        }

        doc.push_str(&format!("**Grantable:** {}\n", if analysis.is_grantable { "Yes" } else { "No" }));

        if let Some(date) = &analysis.metadata.granted_date {
            doc.push_str(&format!("**Granted Date:** {}\n", date));
        }

        if let Some(owner) = &analysis.metadata.business_owner {
            doc.push_str(&format!("**Business Owner:** {}\n", owner));
        }

        if !analysis.access_patterns.is_empty() {
            doc.push_str("\n#### Access Patterns\n\n");
            for pattern in &analysis.access_patterns {
                doc.push_str(&format!("- **{}** ({}) - {}\n",
                    Self::pattern_type_name(&pattern.pattern_type),
                    Self::frequency_name(&pattern.frequency),
                    pattern.business_justification));
                doc.push_str(&format!("  - Data Sensitivity: {}\n", Self::data_sensitivity_name(&pattern.data_sensitivity)));
                doc.push_str(&format!("  - Typical Users: {}\n", pattern.typical_users.join(", ")));
            }
        }

        if !analysis.risk_assessment.risk_factors.is_empty() {
            doc.push_str("\n#### Risk Factors\n\n");
            for factor in &analysis.risk_assessment.risk_factors {
                doc.push_str(&format!("- **{}** ({}) - {}\n",
                    Self::risk_factor_name(&factor.factor_type),
                    Self::risk_level_name(factor.severity),
                    factor.description));
                doc.push_str(&format!("  - Evidence: {}\n", factor.evidence));
            }

            if !analysis.risk_assessment.mitigation_strategies.is_empty() {
                doc.push_str("\n**Mitigation Strategies:**\n");
                for strategy in &analysis.risk_assessment.mitigation_strategies {
                    doc.push_str(&format!("- {}\n", strategy));
                }
            }

            if !analysis.risk_assessment.audit_requirements.is_empty() {
                doc.push_str("\n**Audit Requirements:**\n");
                for requirement in &analysis.risk_assessment.audit_requirements {
                    doc.push_str(&format!("- {}\n", requirement));
                }
            }
        }

        if !analysis.recommendations.is_empty() {
            doc.push_str("\n#### Recommendations\n\n");
            for rec in &analysis.recommendations {
                doc.push_str(&format!("- **{}** ({}) - {}\n",
                    Self::recommendation_type_name(&rec.recommendation_type),
                    Self::urgency_name(&rec.urgency),
                    rec.description));
                doc.push_str(&format!("  - Impact: {}\n", Self::impact_name(&rec.impact)));
                doc.push_str(&format!("  - Benefit: {}\n", rec.expected_benefit));

                if !rec.sql_commands.is_empty() {
                    doc.push_str("  - SQL Commands:\n");
                    for cmd in &rec.sql_commands {
                        doc.push_str(&format!("    ```sql\n    {}\n    ```\n", cmd));
                    }
                }
            }
        }

        doc.push_str("\n---\n\n");
        doc
    }

    fn object_type_name(object_type: &ObjectType) -> &'static str {
        match object_type {
            ObjectType::Table => "Table",
            ObjectType::View => "View",
            ObjectType::Procedure => "Procedure",
            ObjectType::Function => "Function",
            ObjectType::Schema => "Schema",
            ObjectType::Database => "Database",
        }
    }

    fn permission_type_name(permission_type: &PermissionType) -> &'static str {
        match permission_type {
            PermissionType::Select => "SELECT",
            PermissionType::Insert => "INSERT",
            PermissionType::Update => "UPDATE",
            PermissionType::Delete => "DELETE",
            PermissionType::Execute => "EXECUTE",
            PermissionType::Alter => "ALTER",
            PermissionType::Create => "CREATE",
            PermissionType::Drop => "DROP",
            PermissionType::Grant => "GRANT",
            PermissionType::Revoke => "REVOKE",
            PermissionType::All => "ALL",
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

    fn risk_level_name(level: &RiskLevel) -> &'static str {
        match level {
            RiskLevel::Low => "Low",
            RiskLevel::Medium => "Medium",
            RiskLevel::High => "High",
            RiskLevel::Critical => "Critical",
        }
    }

    fn compliance_status_name(status: &ComplianceStatus) -> &'static str {
        match status {
            ComplianceStatus::Compliant => "Compliant",
            ComplianceStatus::NonCompliant => "Non-Compliant",
            ComplianceStatus::UnderReview => "Under Review",
            ComplianceStatus::Exempt => "Exempt",
        }
    }

    fn pattern_type_name(pattern_type: &AccessPatternType) -> &'static str {
        match pattern_type {
            AccessPatternType::ReadOnly => "Read Only",
            AccessPatternType::ReadWrite => "Read Write",
            AccessPatternType::Administrative => "Administrative",
            AccessPatternType::Maintenance => "Maintenance",
            AccessPatternType::Audit => "Audit",
            AccessPatternType::Emergency => "Emergency",
        }
    }

    fn frequency_name(frequency: &AccessFrequency) -> &'static str {
        match frequency {
            AccessFrequency::Constant => "Constant",
            AccessFrequency::Frequent => "Frequent",
            AccessFrequency::Occasional => "Occasional",
            AccessFrequency::Rare => "Rare",
            AccessFrequency::Never => "Never",
        }
    }

    fn data_sensitivity_name(sensitivity: &DataSensitivity) -> &'static str {
        match sensitivity {
            DataSensitivity::Public => "Public",
            DataSensitivity::Internal => "Internal",
            DataSensitivity::Sensitive => "Sensitive",
            DataSensitivity::Confidential => "Confidential",
            DataSensitivity::Restricted => "Restricted",
        }
    }

    fn risk_factor_name(factor_type: &RiskFactorType) -> &'static str {
        match factor_type {
            RiskFactorType::OverPrivileged => "Over Privileged",
            RiskFactorType::DirectObjectAccess => "Direct Object Access",
            RiskFactorType::MissingAudit => "Missing Audit",
            RiskFactorType::WeakAuthentication => "Weak Authentication",
            RiskFactorType::DataExposure => "Data Exposure",
            RiskFactorType::PrivilegeEscalation => "Privilege Escalation",
        }
    }

    fn recommendation_type_name(rec_type: &RecommendationType) -> &'static str {
        match rec_type {
            RecommendationType::RevokePermission => "Revoke Permission",
            RecommendationType::GrantLimitedPermission => "Grant Limited Permission",
            RecommendationType::ImplementRowLevelSecurity => "Implement Row Level Security",
            RecommendationType::AddAuditLogging => "Add Audit Logging",
            RecommendationType::CreateSecurityPolicy => "Create Security Policy",
            RecommendationType::ReviewAccessPatterns => "Review Access Patterns",
        }
    }

    fn urgency_name(urgency: &UrgencyLevel) -> &'static str {
        match urgency {
            UrgencyLevel::Immediate => "Immediate",
            UrgencyLevel::High => "High",
            UrgencyLevel::Medium => "Medium",
            UrgencyLevel::Low => "Low",
            UrgencyLevel::Optional => "Optional",
        }
    }

    fn impact_name(impact: &ImpactLevel) -> &'static str {
        match impact {
            ImpactLevel::Critical => "Critical",
            ImpactLevel::High => "High",
            ImpactLevel::Medium => "Medium",
            ImpactLevel::Low => "Low",
            ImpactLevel::Minimal => "Minimal",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DatabasePermissionReport {
    pub total_permissions: usize,
    pub average_security_score: f64,
    pub risk_distribution: HashMap<RiskLevel, usize>,
    pub privileged_users: Vec<String>,
    pub critical_permissions: Vec<String>,
    pub permission_analyses: Vec<PermissionAnalysis>,
    pub security_recommendations: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permission_risk_assessment() {
        let permission = Permission {
            object_name: "sensitive_table".to_string(),
            object_type: ObjectType::Table,
            grantee: "user1".to_string(),
            permission_type: PermissionType::All,
            grantor: Some("admin".to_string()),
            is_grantable: true,
            metadata: PermissionMetadata::default(),
        };

        let database = Database::new("test_db");
        let analysis = PermissionAnalysisService::analyze_permission(&permission, &database);

        assert_eq!(analysis.risk_assessment.overall_risk, RiskLevel::High);
        assert!(analysis.risk_assessment.risk_factors.len() > 0);
        assert!(analysis.is_grantable);
    }

    #[test]
    fn test_data_sensitivity_assessment() {
        let mut table = Table::new("user_financial_data");
        table.add_column(Column::new("salary", DataType::Decimal { precision: 10, scale: 2 }));

        let mut database = Database::new("test_db");
        database.add_table(table);

        let sensitivity = PermissionAnalysisService::assess_data_sensitivity("user_financial_data", &database);
        assert_eq!(sensitivity, DataSensitivity::Confidential);
    }

    #[test]
    fn test_security_level_determination() {
        let permission = Permission {
            object_name: "public_table".to_string(),
            object_type: ObjectType::Table,
            grantee: "user1".to_string(),
            permission_type: PermissionType::Select,
            grantor: Some("admin".to_string()),
            is_grantable: false,
            metadata: PermissionMetadata::default(),
        };

        let database = Database::new("test_db");
        let security_level = PermissionAnalysisService::determine_security_level(&permission, &database);

        assert_eq!(security_level, SecurityLevel::Public);
    }
}
