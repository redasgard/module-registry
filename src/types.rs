//! Type definitions for module registry

use serde::{Deserialize, Serialize};
use std::any::Any;
use std::collections::HashMap;
use anyhow::Result;

use crate::constants::*;

/// Base trait that all modules must implement
pub trait Module: Send + Sync {
    /// Get the module's unique name
    fn name(&self) -> &str;

    /// Get the module type (e.g., "processor", "provider", "plugin")
    fn module_type(&self) -> &str;
}

/// Module signature for cryptographic verification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleSignature {
    /// SHA-256 hash of the module code
    pub code_hash: String,
    /// Digital signature of the code hash
    pub signature: String,
    /// Public key used for verification
    pub public_key: String,
    /// Timestamp when signed
    pub timestamp: u64,
    /// Signature algorithm used
    pub algorithm: String,
}

/// Module permissions for sandboxing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModulePermissions {
    /// Can access filesystem
    pub filesystem_access: bool,
    /// Can make network calls
    pub network_access: bool,
    /// Can spawn processes
    pub process_spawn: bool,
    /// Can access environment variables
    pub env_access: bool,
    /// Can access system resources
    pub system_access: bool,
    /// Maximum memory usage in MB
    pub memory_limit_mb: u64,
    /// Maximum CPU usage percentage
    pub cpu_limit_percent: u8,
    /// Maximum execution time in seconds
    pub timeout_seconds: u64,
}

impl Default for ModulePermissions {
    fn default() -> Self {
        Self {
            filesystem_access: false,
            network_access: false,
            process_spawn: false,
            env_access: false,
            system_access: false,
            memory_limit_mb: DEFAULT_MEMORY_LIMIT_MB,
            cpu_limit_percent: DEFAULT_CPU_LIMIT_PERCENT,
            timeout_seconds: DEFAULT_TIMEOUT_SECONDS,
        }
    }
}

/// Code review status for modules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CodeReviewStatus {
    /// Not reviewed yet
    Pending,
    /// Under review
    InProgress,
    /// Approved by reviewer
    Approved { reviewer: String, timestamp: u64 },
    /// Rejected by reviewer
    Rejected { reviewer: String, reason: String, timestamp: u64 },
}

/// Supply chain verification data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupplyChainInfo {
    /// Source repository URL
    pub source_url: String,
    /// Commit hash
    pub commit_hash: String,
    /// Build timestamp
    pub build_timestamp: u64,
    /// Dependencies with versions
    pub dependencies: HashMap<String, String>,
    /// Build environment info
    pub build_environment: String,
    /// Verifier signature
    pub verifier_signature: Option<String>,
}

/// Sandbox configuration for module isolation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxConfig {
    /// Enable sandboxing
    pub enabled: bool,
    /// Isolate from host filesystem
    pub filesystem_isolation: bool,
    /// Isolate network access
    pub network_isolation: bool,
    /// Isolate process environment
    pub process_isolation: bool,
    /// Read-only filesystem
    pub read_only_fs: bool,
    /// Allowed file paths (if not read-only)
    pub allowed_paths: Vec<String>,
    /// Denied file paths
    pub denied_paths: Vec<String>,
}

impl Default for SandboxConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            filesystem_isolation: true,
            network_isolation: true,
            process_isolation: true,
            read_only_fs: true,
            allowed_paths: Vec::new(),
            denied_paths: DEFAULT_DENIED_PATHS.iter().map(|s| s.to_string()).collect(),
        }
    }
}

/// Module metadata for registration with security features
#[derive(Debug, Clone)]
pub struct ModuleMetadata {
    pub name: String,
    pub module_type: String,
    pub instantiate_fn_name: String,
    pub module_path: String,
    pub struct_name: String,
    /// Cryptographic signature
    pub signature: Option<ModuleSignature>,
    /// Module permissions
    pub permissions: ModulePermissions,
    /// Code review status
    pub review_status: CodeReviewStatus,
    /// Supply chain information
    pub supply_chain: Option<SupplyChainInfo>,
    /// Security sandbox configuration
    pub sandbox_config: SandboxConfig,
}

/// Security report for a module
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityReport {
    pub name: String,
    pub has_signature: bool,
    pub signature_verified: bool,
    pub is_approved: bool,
    pub has_supply_chain: bool,
    pub supply_chain_verified: bool,
    pub permissions: ModulePermissions,
    pub sandbox_enabled: bool,
}

/// Factory function type for module instantiation
/// Returns Box<dyn Any + Send + Sync> so it can work with any trait object
pub type ModuleFactory = fn() -> Result<Box<dyn Any + Send + Sync>, anyhow::Error>;

/// Registration entry for inventory collection
pub struct ModuleRegistration {
    pub name: &'static str,
    pub module_type: &'static str,
    pub instantiate_fn_name: &'static str,
    pub module_path: &'static str,
    pub struct_name: &'static str,
    pub factory: ModuleFactory,
}

impl ModuleMetadata {
    /// Create a new module metadata
    pub fn new(
        name: String,
        module_type: String,
        instantiate_fn_name: String,
        module_path: String,
        struct_name: String,
    ) -> Self {
        Self {
            name,
            module_type,
            instantiate_fn_name,
            module_path,
            struct_name,
            signature: None,
            permissions: ModulePermissions::default(),
            review_status: CodeReviewStatus::Pending,
            supply_chain: None,
            sandbox_config: SandboxConfig::default(),
        }
    }

    /// Create a secure module metadata
    pub fn secure(
        name: String,
        module_type: String,
        instantiate_fn_name: String,
        module_path: String,
        struct_name: String,
        signature: Option<ModuleSignature>,
        permissions: ModulePermissions,
        supply_chain: Option<SupplyChainInfo>,
    ) -> Self {
        Self {
            name,
            module_type,
            instantiate_fn_name,
            module_path,
            struct_name,
            signature,
            permissions,
            review_status: CodeReviewStatus::Pending,
            supply_chain,
            sandbox_config: SandboxConfig::default(),
        }
    }

    /// Check if the module has valid signature
    pub fn has_valid_signature(&self) -> bool {
        self.signature.is_some()
    }

    /// Check if the module is approved
    pub fn is_approved(&self) -> bool {
        matches!(self.review_status, CodeReviewStatus::Approved { .. })
    }

    /// Check if the module has supply chain info
    pub fn has_supply_chain(&self) -> bool {
        self.supply_chain.is_some()
    }

    /// Get a summary of the module metadata
    pub fn summary(&self) -> String {
        format!(
            "Module: {} (type: {}) - signature: {}, approved: {}, supply_chain: {}, sandbox: {}",
            self.name,
            self.module_type,
            self.has_valid_signature(),
            self.is_approved(),
            self.has_supply_chain(),
            self.sandbox_config.enabled
        )
    }
}
