//! Security-related functionality for module registry

use anyhow::Result;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::constants::*;
use crate::types::*;

/// Security validator for modules
pub struct SecurityValidator;

impl SecurityValidator {
    /// Verify module signature
    pub fn verify_signature(metadata: &ModuleMetadata) -> Result<bool> {
        match &metadata.signature {
            Some(sig) => {
                // Check if signature is not expired
                let current_time = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
                
                if current_time - sig.timestamp > SIGNATURE_EXPIRY_SECONDS {
                    return Ok(false);
                }

                // Verify signature algorithm
                if sig.algorithm != DEFAULT_SIGNATURE_ALGORITHM {
                    return Ok(false);
                }

                // In a real implementation, verify the actual signature
                // For now, just check that signature exists and is not empty
                Ok(!sig.signature.is_empty() && !sig.public_key.is_empty())
            }
            None => Ok(false), // No signature means not verified
        }
    }

    /// Check if module has required permissions
    pub fn check_permissions(metadata: &ModuleMetadata, required_permission: &str) -> Result<bool> {
        match required_permission {
            "filesystem_access" => Ok(metadata.permissions.filesystem_access),
            "network_access" => Ok(metadata.permissions.network_access),
            "process_spawn" => Ok(metadata.permissions.process_spawn),
            "env_access" => Ok(metadata.permissions.env_access),
            "system_access" => Ok(metadata.permissions.system_access),
            _ => Ok(false),
        }
    }

    /// Check if module passed code review
    pub fn is_approved(metadata: &ModuleMetadata) -> Result<bool> {
        Ok(matches!(metadata.review_status, CodeReviewStatus::Approved { .. }))
    }

    /// Verify supply chain information
    pub fn verify_supply_chain(metadata: &ModuleMetadata) -> Result<bool> {
        match &metadata.supply_chain {
            Some(chain) => {
                // Verify source URL is valid
                if chain.source_url.is_empty() {
                    return Ok(false);
                }

                // Verify commit hash is not empty
                if chain.commit_hash.is_empty() {
                    return Ok(false);
                }

                // Verify build timestamp is reasonable
                let current_time = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
                
                if chain.build_timestamp > current_time {
                    return Ok(false);
                }

                // In a real implementation, verify the verifier signature
                Ok(true)
            }
            None => Ok(false), // No supply chain info means not verified
        }
    }

    /// Perform comprehensive security check
    pub fn comprehensive_check(metadata: &ModuleMetadata) -> SecurityCheckResult {
        let mut issues = Vec::new();
        let mut warnings = Vec::new();

        // Check signature
        match Self::verify_signature(metadata) {
            Ok(true) => {
                // Signature is valid
            }
            Ok(false) => {
                issues.push(SecurityIssue {
                    severity: SecuritySeverity::High,
                    message: "Module signature verification failed".to_string(),
                    component: "signature".to_string(),
                });
            }
            Err(e) => {
                warnings.push(SecurityWarning {
                    message: format!("Failed to verify signature: {}", e),
                    component: "signature".to_string(),
                });
            }
        }

        // Check approval status
        match Self::is_approved(metadata) {
            Ok(true) => {
                // Module is approved
            }
            Ok(false) => {
                issues.push(SecurityIssue {
                    severity: SecuritySeverity::Medium,
                    message: "Module not approved by code review".to_string(),
                    component: "review".to_string(),
                });
            }
            Err(e) => {
                warnings.push(SecurityWarning {
                    message: format!("Failed to check approval status: {}", e),
                    component: "review".to_string(),
                });
            }
        }

        // Check supply chain
        match Self::verify_supply_chain(metadata) {
            Ok(true) => {
                // Supply chain is verified
            }
            Ok(false) => {
                issues.push(SecurityIssue {
                    severity: SecuritySeverity::Medium,
                    message: "Supply chain verification failed".to_string(),
                    component: "supply_chain".to_string(),
                });
            }
            Err(e) => {
                warnings.push(SecurityWarning {
                    message: format!("Failed to verify supply chain: {}", e),
                    component: "supply_chain".to_string(),
                });
            }
        }

        // Check permissions
        if metadata.permissions.system_access && !metadata.sandbox_config.enabled {
            issues.push(SecurityIssue {
                severity: SecuritySeverity::High,
                message: "System access granted without sandboxing".to_string(),
                component: "permissions".to_string(),
            });
        }

        let is_secure = issues.is_empty();
        let risk_level = Self::calculate_risk_level(&issues);

        SecurityCheckResult {
            is_secure,
            risk_level,
            issues,
            warnings,
            check_timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }

    /// Calculate risk level based on security issues
    fn calculate_risk_level(issues: &[SecurityIssue]) -> SecurityRiskLevel {
        if issues.iter().any(|i| matches!(i.severity, SecuritySeverity::Critical)) {
            SecurityRiskLevel::Critical
        } else if issues.iter().any(|i| matches!(i.severity, SecuritySeverity::High)) {
            SecurityRiskLevel::High
        } else if issues.iter().any(|i| matches!(i.severity, SecuritySeverity::Medium)) {
            SecurityRiskLevel::Medium
        } else if !issues.is_empty() {
            SecurityRiskLevel::Low
        } else {
            SecurityRiskLevel::None
        }
    }
}

/// Security check result
#[derive(Debug, Clone)]
pub struct SecurityCheckResult {
    pub is_secure: bool,
    pub risk_level: SecurityRiskLevel,
    pub issues: Vec<SecurityIssue>,
    pub warnings: Vec<SecurityWarning>,
    pub check_timestamp: u64,
}

/// Security issue severity
#[derive(Debug, Clone, PartialEq)]
pub enum SecuritySeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Security risk level
#[derive(Debug, Clone, PartialEq)]
pub enum SecurityRiskLevel {
    None,
    Low,
    Medium,
    High,
    Critical,
}

/// Security issue
#[derive(Debug, Clone)]
pub struct SecurityIssue {
    pub severity: SecuritySeverity,
    pub message: String,
    pub component: String,
}

/// Security warning
#[derive(Debug, Clone)]
pub struct SecurityWarning {
    pub message: String,
    pub component: String,
}

impl SecurityCheckResult {
    /// Get a summary of the security check
    pub fn summary(&self) -> String {
        format!(
            "Security check {}: {} issues, {} warnings, risk level: {:?}",
            if self.is_secure { "PASSED" } else { "FAILED" },
            self.issues.len(),
            self.warnings.len(),
            self.risk_level
        )
    }

    /// Check if the result indicates a security risk
    pub fn has_security_risk(&self) -> bool {
        matches!(self.risk_level, SecurityRiskLevel::Medium | SecurityRiskLevel::High | SecurityRiskLevel::Critical)
    }

    /// Get all critical issues
    pub fn get_critical_issues(&self) -> Vec<&SecurityIssue> {
        self.issues.iter().filter(|i| matches!(i.severity, SecuritySeverity::Critical)).collect()
    }

    /// Get all high-severity issues
    pub fn get_high_severity_issues(&self) -> Vec<&SecurityIssue> {
        self.issues.iter().filter(|i| matches!(i.severity, SecuritySeverity::High | SecuritySeverity::Critical)).collect()
    }
}
