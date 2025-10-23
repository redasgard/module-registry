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

pub mod constants;
pub mod macros;
pub mod registry;
pub mod security;
pub mod types;

// Re-export main types and functions
pub use constants::*;
pub use macros::*;
pub use registry::*;
pub use security::*;
pub use types::*;

// Re-export the main ModuleRegistry struct
pub use registry::ModuleRegistry;

// Re-export inventory collection
inventory::collect!(ModuleRegistration);