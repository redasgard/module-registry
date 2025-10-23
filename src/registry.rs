//! Module registry implementation

use anyhow::{Context, Result};
use std::any::Any;
use std::collections::HashMap;
use std::sync::{OnceLock, RwLock};

use crate::security::{SecurityValidator, SecurityCheckResult};
use crate::types::*;

// Optional tracing support
#[cfg(feature = "tracing")]
use tracing::info;

#[cfg(not(feature = "tracing"))]
macro_rules! info {
    ($($arg:tt)*) => {};
}

/// Generic module registry
///
/// Thread-safe registry for storing and instantiating modules at runtime.
/// Modules are registered with a factory function and can be created by name.
pub struct ModuleRegistry {
    modules: RwLock<HashMap<String, (ModuleMetadata, ModuleFactory)>>,
}

impl ModuleRegistry {
    /// Create a new empty registry
    pub fn new() -> Self {
        Self {
            modules: RwLock::new(HashMap::new()),
        }
    }

    /// Get the global registry instance
    pub fn global() -> &'static Self {
        static REGISTRY: OnceLock<ModuleRegistry> = OnceLock::new();
        REGISTRY.get_or_init(|| {
            let registry = Self::new();

            // Load inventory-registered modules
            for reg in inventory::iter::<ModuleRegistration> {
                let metadata = ModuleMetadata::new(
                    reg.name.to_string(),
                    reg.module_type.to_string(),
                    reg.instantiate_fn_name.to_string(),
                    reg.module_path.to_string(),
                    reg.struct_name.to_string(),
                );
                registry
                    .modules
                    .write()
                    .unwrap()
                    .insert(metadata.name.clone(), (metadata, reg.factory));
            }

            info!(
                "Module registry initialized with {} modules",
                registry.modules.read().unwrap().len()
            );

            registry
        })
    }

    /// Register a module with a factory function
    ///
    /// The factory function should return a Box<dyn YourTrait> cast to Box<dyn Any + Send + Sync>
    pub fn register(&self, name: &str, module_type: &str, factory: ModuleFactory) {
        self.register_with_metadata(
            name,
            module_type,
            "factory",
            module_path!(),
            "Module",
            factory,
        );
    }

    /// Register a module with full metadata
    pub fn register_with_metadata(
        &self,
        name: &str,
        module_type: &str,
        instantiate_fn: &str,
        module_path: &str,
        struct_name: &str,
        factory: ModuleFactory,
    ) {
        let metadata = ModuleMetadata::new(
            name.to_string(),
            module_type.to_string(),
            instantiate_fn.to_string(),
            module_path.to_string(),
            struct_name.to_string(),
        );

        let mut modules = self.modules.write().expect("Failed to acquire write lock");
        modules.insert(name.to_string(), (metadata, factory));

        info!("Registered module: {} (type: {})", name, module_type);
    }

    /// Create a module instance by name
    ///
    /// Returns Box<dyn Any + Send + Sync> which you must downcast to your trait type
    pub fn create_any(&self, name: &str) -> Result<Box<dyn Any + Send + Sync>> {
        let modules = self.modules.read().expect("Failed to acquire read lock");

        let (_metadata, factory) = modules
            .get(name)
            .ok_or_else(|| anyhow::anyhow!("Module not found: {}", name))?;

        info!("Creating module: {}", name);

        factory().with_context(|| format!("Failed to instantiate module: {}", name))
    }

    /// Create and downcast a module to a specific trait type
    pub fn create<T: 'static>(&self, name: &str) -> Result<Box<T>> {
        let any_module = self.create_any(name)?;

        any_module
            .downcast::<T>()
            .map_err(|_| anyhow::anyhow!("Module type mismatch for: {}", name))
    }

    /// Get all registered module names
    pub fn list_modules(&self) -> Vec<String> {
        self.modules
            .read()
            .expect("Failed to acquire read lock")
            .keys()
            .cloned()
            .collect()
    }

    /// Get all registered module names (alias for compatibility)
    pub fn get_module_names(&self) -> Vec<String> {
        self.list_modules()
    }

    /// Check if a module is registered
    pub fn has_module(&self, name: &str) -> bool {
        self.modules
            .read()
            .expect("Failed to acquire read lock")
            .contains_key(name)
    }

    /// Get metadata for a module
    pub fn get_metadata(&self, name: &str) -> Option<ModuleMetadata> {
        self.modules
            .read()
            .expect("Failed to acquire read lock")
            .get(name)
            .map(|(metadata, _)| metadata.clone())
    }

    /// Clear all registered modules (for testing)
    pub fn clear(&self) {
        self.modules
            .write()
            .expect("Failed to acquire write lock")
            .clear();
    }

    /// Get count of registered modules
    pub fn count(&self) -> usize {
        self.modules
            .read()
            .expect("Failed to acquire read lock")
            .len()
    }

    /// Verify module signature
    pub fn verify_module_signature(&self, name: &str) -> Result<bool> {
        let modules = self.modules.read().expect("Failed to acquire read lock");
        let (metadata, _) = modules
            .get(name)
            .ok_or_else(|| anyhow::anyhow!("Module not found: {}", name))?;

        SecurityValidator::verify_signature(metadata)
    }

    /// Check if module has required permissions
    pub fn check_module_permissions(&self, name: &str, required_permission: &str) -> Result<bool> {
        let modules = self.modules.read().expect("Failed to acquire read lock");
        let (metadata, _) = modules
            .get(name)
            .ok_or_else(|| anyhow::anyhow!("Module not found: {}", name))?;

        SecurityValidator::check_permissions(metadata, required_permission)
    }

    /// Check if module passed code review
    pub fn is_module_approved(&self, name: &str) -> Result<bool> {
        let modules = self.modules.read().expect("Failed to acquire read lock");
        let (metadata, _) = modules
            .get(name)
            .ok_or_else(|| anyhow::anyhow!("Module not found: {}", name))?;

        SecurityValidator::is_approved(metadata)
    }

    /// Verify supply chain information
    pub fn verify_supply_chain(&self, name: &str) -> Result<bool> {
        let modules = self.modules.read().expect("Failed to acquire read lock");
        let (metadata, _) = modules
            .get(name)
            .ok_or_else(|| anyhow::anyhow!("Module not found: {}", name))?;

        SecurityValidator::verify_supply_chain(metadata)
    }

    /// Create module with security checks
    pub fn create_secure(&self, name: &str) -> Result<Box<dyn Any + Send + Sync>> {
        // Verify signature
        if !self.verify_module_signature(name)? {
            return Err(anyhow::anyhow!("Module signature verification failed: {}", name));
        }

        // Check if module is approved
        if !self.is_module_approved(name)? {
            return Err(anyhow::anyhow!("Module not approved: {}", name));
        }

        // Verify supply chain
        if !self.verify_supply_chain(name)? {
            return Err(anyhow::anyhow!("Supply chain verification failed: {}", name));
        }

        // Create module with sandboxing
        self.create_with_sandbox(name)
    }

    /// Create module with sandbox configuration
    pub fn create_with_sandbox(&self, name: &str) -> Result<Box<dyn Any + Send + Sync>> {
        let modules = self.modules.read().expect("Failed to acquire read lock");
        let (metadata, factory) = modules
            .get(name)
            .ok_or_else(|| anyhow::anyhow!("Module not found: {}", name))?;

        // Apply sandbox configuration
        if metadata.sandbox_config.enabled {
            info!("Creating sandboxed module: {}", name);
            // In a real implementation, set up sandbox environment
            // For now, just log the sandbox config
            info!("Sandbox config: {:?}", metadata.sandbox_config);
        }

        info!("Creating module: {}", name);
        factory().with_context(|| format!("Failed to instantiate module: {}", name))
    }

    /// Register module with security metadata
    pub fn register_secure(
        &self,
        name: &str,
        module_type: &str,
        factory: ModuleFactory,
        signature: Option<ModuleSignature>,
        permissions: ModulePermissions,
        supply_chain: Option<SupplyChainInfo>,
    ) {
        let metadata = ModuleMetadata::secure(
            name.to_string(),
            module_type.to_string(),
            "factory".to_string(),
            module_path!().to_string(),
            "Module".to_string(),
            signature,
            permissions,
            supply_chain,
        );

        let mut modules = self.modules.write().expect("Failed to acquire write lock");
        modules.insert(name.to_string(), (metadata, factory));

        info!("Registered secure module: {} (type: {})", name, module_type);
    }

    /// Update code review status
    pub fn update_review_status(
        &self,
        name: &str,
        status: CodeReviewStatus,
    ) -> Result<()> {
        let mut modules = self.modules.write().expect("Failed to acquire write lock");
        let (metadata, factory) = modules
            .get_mut(name)
            .ok_or_else(|| anyhow::anyhow!("Module not found: {}", name))?;

        metadata.review_status = status;
        info!("Updated review status for module: {}", name);
        Ok(())
    }

    /// Get security report for all modules
    pub fn get_security_report(&self) -> HashMap<String, SecurityReport> {
        let modules = self.modules.read().expect("Failed to acquire read lock");
        let mut report = HashMap::new();

        for (name, (metadata, _)) in modules.iter() {
            let security_report = SecurityReport {
                name: name.clone(),
                has_signature: metadata.signature.is_some(),
                signature_verified: metadata.signature.is_some(),
                is_approved: matches!(metadata.review_status, CodeReviewStatus::Approved { .. }),
                has_supply_chain: metadata.supply_chain.is_some(),
                supply_chain_verified: metadata.supply_chain.is_some(),
                permissions: metadata.permissions.clone(),
                sandbox_enabled: metadata.sandbox_config.enabled,
            };
            report.insert(name.clone(), security_report);
        }

        report
    }

    /// Perform comprehensive security check on all modules
    pub fn security_audit(&self) -> HashMap<String, SecurityCheckResult> {
        let modules = self.modules.read().expect("Failed to acquire read lock");
        let mut audit_results = HashMap::new();

        for (name, (metadata, _)) in modules.iter() {
            let security_check = SecurityValidator::comprehensive_check(metadata);
            audit_results.insert(name.clone(), security_check);
        }

        audit_results
    }
}

impl Default for ModuleRegistry {
    fn default() -> Self {
        Self::new()
    }
}
