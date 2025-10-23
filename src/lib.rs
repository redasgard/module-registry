//! # Module Registry
//!
//! A dynamic module/plugin registry system with compile-time discovery and runtime instantiation.
//!
//! ## Features
//!
//! - **Compile-Time Discovery**: Uses `inventory` crate for automatic module registration
//! - **Runtime Instantiation**: Create modules dynamically by name
//! - **Type-Safe**: Generic factory functions with trait objects
//! - **Thread-Safe**: Built on `RwLock` for concurrent access
//! - **Flexible**: Support for any module type with custom configuration
//!
//! ## Quick Start
//!
//! ```rust
//! use module_registry::{ModuleRegistry, Module};
//! use anyhow::Result;
//!
//! // Define your module trait
//! pub trait TextProcessor: Module {
//!     fn process(&self, input: &str) -> Result<String>;
//! }
//!
//! // Implement a module
//! struct UpperCaseModule;
//!
//! impl Module for UpperCaseModule {
//!     fn name(&self) -> &str {
//!         "uppercase"
//!     }
//!     
//!     fn module_type(&self) -> &str {
//!         "text_processor"
//!     }
//! }
//!
//! impl TextProcessor for UpperCaseModule {
//!     fn process(&self, input: &str) -> Result<String> {
//!         Ok(input.to_uppercase())
//!     }
//! }
//!
//! # fn main() -> Result<()> {
//! // Create registry
//! let registry = ModuleRegistry::new();
//!
//! // Register module (factory returns Box<dyn Any + Send + Sync>)
//! registry.register(
//!     "uppercase",
//!     "text_processor",
//!     || Ok(Box::new(Box::new(UpperCaseModule) as Box<dyn TextProcessor>))
//! );
//!
//! // Create module instance  
//! let any_module = registry.create_any("uppercase")?;
//! let module = any_module.downcast::<Box<dyn TextProcessor>>()
//!     .map_err(|_| anyhow::anyhow!("Type mismatch"))?;
//! assert_eq!(module.name(), "uppercase");
//! # Ok(())
//! # }
//! ```

use anyhow::{Context, Result};
use std::any::Any;
use std::collections::HashMap;
use std::sync::{OnceLock, RwLock};

// Optional tracing support
#[cfg(feature = "tracing")]
use tracing::info;

#[cfg(not(feature = "tracing"))]
macro_rules! info {
    ($($arg:tt)*) => {};
}

/// Base trait that all modules must implement
pub trait Module: Send + Sync {
    /// Get the module's unique name
    fn name(&self) -> &str;

    /// Get the module type (e.g., "processor", "provider", "plugin")
    fn module_type(&self) -> &str;
}

/// Module metadata for registration
#[derive(Debug, Clone)]
pub struct ModuleMetadata {
    pub name: String,
    pub module_type: String,
    pub instantiate_fn_name: String,
    pub module_path: String,
    pub struct_name: String,
}

/// Factory function type for module instantiation
/// Returns Box<dyn Any + Send + Sync> so it can work with any trait object
pub type ModuleFactory = fn() -> Result<Box<dyn Any + Send + Sync>>;

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
                let metadata = ModuleMetadata {
                    name: reg.name.to_string(),
                    module_type: reg.module_type.to_string(),
                    instantiate_fn_name: reg.instantiate_fn_name.to_string(),
                    module_path: reg.module_path.to_string(),
                    struct_name: reg.struct_name.to_string(),
                };
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
        let metadata = ModuleMetadata {
            name: name.to_string(),
            module_type: module_type.to_string(),
            instantiate_fn_name: instantiate_fn.to_string(),
            module_path: module_path.to_string(),
            struct_name: struct_name.to_string(),
        };

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
}

impl Default for ModuleRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Registration entry for inventory collection
pub struct ModuleRegistration {
    pub name: &'static str,
    pub module_type: &'static str,
    pub instantiate_fn_name: &'static str,
    pub module_path: &'static str,
    pub struct_name: &'static str,
    pub factory: ModuleFactory,
}

inventory::collect!(ModuleRegistration);

/// Macro for registering modules with inventory
///
/// # Example
///
/// ```ignore
/// use module_registry::register_module;
///
/// register_module!("my_module", "MyModule", create_my_module);
/// ```
#[macro_export]
macro_rules! register_module {
    ($name:expr, $struct_name:expr, $factory:path) => {
        inventory::submit! {
            $crate::ModuleRegistration {
                name: $name,
                module_type: "module",
                instantiate_fn_name: stringify!($factory),
                module_path: module_path!(),
                struct_name: $struct_name,
                factory: $factory,
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test module trait
    trait TextProcessor: Module {
        fn process(&self, input: &str) -> String;
    }

    // Test module implementation
    struct UpperCaseProcessor;

    impl Module for UpperCaseProcessor {
        fn name(&self) -> &str {
            "uppercase"
        }

        fn module_type(&self) -> &str {
            "text_processor"
        }
    }

    impl TextProcessor for UpperCaseProcessor {
        fn process(&self, input: &str) -> String {
            input.to_uppercase()
        }
    }

    fn create_uppercase() -> Result<Box<dyn Any + Send + Sync>> {
        Ok(Box::new(Box::new(UpperCaseProcessor) as Box<dyn TextProcessor>))
    }

    #[test]
    fn test_registry_creation() {
        let registry = ModuleRegistry::new();
        assert_eq!(registry.count(), 0);
    }

    #[test]
    fn test_module_registration() {
        let registry = ModuleRegistry::new();

        registry.register("uppercase", "text_processor", create_uppercase);

        assert_eq!(registry.count(), 1);
        assert!(registry.has_module("uppercase"));
        assert!(!registry.has_module("nonexistent"));
    }

    #[test]
    fn test_module_creation() {
        let registry = ModuleRegistry::new();
        registry.register("uppercase", "text_processor", create_uppercase);

        let result = registry.create_any("uppercase");
        assert!(result.is_ok());
    }

    #[test]
    fn test_list_modules() {
        let registry = ModuleRegistry::new();

        registry.register("module1", "type1", create_uppercase);
        registry.register("module2", "type2", create_uppercase);

        let names = registry.list_modules();
        assert_eq!(names.len(), 2);
        assert!(names.contains(&"module1".to_string()));
        assert!(names.contains(&"module2".to_string()));
    }

    #[test]
    fn test_get_metadata() {
        let registry = ModuleRegistry::new();

        registry.register_with_metadata(
            "test_module",
            "test_type",
            "create_test",
            "test::path",
            "TestStruct",
            create_uppercase,
        );

        let metadata = registry.get_metadata("test_module");
        assert!(metadata.is_some());

        let meta = metadata.unwrap();
        assert_eq!(meta.name, "test_module");
        assert_eq!(meta.module_type, "test_type");
        assert_eq!(meta.struct_name, "TestStruct");
    }

    #[test]
    fn test_module_not_found() {
        let registry = ModuleRegistry::new();

        let result = registry.create_any("nonexistent");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found"));
    }

    #[test]
    fn test_clear_registry() {
        let registry = ModuleRegistry::new();

        registry.register("module1", "test", create_uppercase);
        assert_eq!(registry.count(), 1);

        registry.clear();
        assert_eq!(registry.count(), 0);
    }

    #[test]
    fn test_get_module_names_alias() {
        let registry = ModuleRegistry::new();

        registry.register("test", "type", create_uppercase);

        // list_modules() and get_module_names() should return the same thing
        assert_eq!(registry.list_modules(), registry.get_module_names());
    }

    #[test]
    fn test_global_registry() {
        let global = ModuleRegistry::global();
        
        // Should be able to access global registry
        let count_before = global.count();

        // Register something
        global.register("test_global", "test", create_uppercase);

        assert_eq!(global.count(), count_before + 1);

        // Clean up
        global.clear();
    }
}
