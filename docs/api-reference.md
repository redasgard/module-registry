# API Reference - Module Registry

## Overview

This document provides comprehensive API reference for the Module Registry module.

## Core Types

### ModuleRegistry

The main registry for managing modules.

```rust
pub struct ModuleRegistry {
    // Private fields
}

impl ModuleRegistry {
    /// Create a new module registry
    pub fn new() -> Self;
    
    /// Register a module
    pub fn register(&self, module: ModuleConfig) -> Result<(), RegistryError>;
    
    /// Unregister a module
    pub fn unregister(&self, name: &str) -> Result<(), RegistryError>;
    
    /// Get a module by name
    pub fn get(&self, name: &str) -> Result<Box<dyn Module>, RegistryError>;
    
    /// List all registered modules
    pub fn list(&self) -> Vec<ModuleInfo>;
    
    /// Check if a module is registered
    pub fn is_registered(&self, name: &str) -> bool;
}
```

### ModuleConfig

Configuration for module registration.

```rust
pub struct ModuleConfig {
    pub name: String,
    pub version: String,
    pub module_type: ModuleType,
    pub dependencies: Vec<String>,
    pub capabilities: Vec<String>,
    pub metadata: HashMap<String, String>,
}

impl ModuleConfig {
    /// Create a new module configuration
    pub fn new() -> Self;
    
    /// Set the module name
    pub fn with_name(mut self, name: &str) -> Self;
    
    /// Set the module version
    pub fn with_version(mut self, version: &str) -> Self;
    
    /// Set the module type
    pub fn with_type(mut self, module_type: ModuleType) -> Self;
    
    /// Add a dependency
    pub fn add_dependency(mut self, dependency: &str) -> Self;
    
    /// Add a capability
    pub fn add_capability(mut self, capability: &str) -> Self;
    
    /// Set metadata
    pub fn with_metadata(mut self, key: &str, value: &str) -> Self;
}
```

### Module

Trait for all modules.

```rust
pub trait Module: Send + Sync {
    /// Get the module name
    fn name(&self) -> &str;
    
    /// Get the module version
    fn version(&self) -> &str;
    
    /// Get the module type
    fn module_type(&self) -> ModuleType;
    
    /// Get the module capabilities
    fn capabilities(&self) -> &[String];
    
    /// Initialize the module
    fn initialize(&mut self) -> Result<(), ModuleError>;
    
    /// Cleanup the module
    fn cleanup(&mut self) -> Result<(), ModuleError>;
    
    /// Process data
    fn process(&self, data: &[u8]) -> Result<Vec<u8>, ModuleError>;
}
```

### ModuleType

Enumeration of module types.

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum ModuleType {
    /// Data processing module
    DataProcessor,
    /// Analysis module
    Analyzer,
    /// Storage module
    Storage,
    /// Network module
    Network,
    /// Security module
    Security,
    /// Custom module type
    Custom(String),
}
```

### ModuleInfo

Information about a registered module.

```rust
pub struct ModuleInfo {
    pub name: String,
    pub version: String,
    pub module_type: ModuleType,
    pub dependencies: Vec<String>,
    pub capabilities: Vec<String>,
    pub metadata: HashMap<String, String>,
    pub registration_time: SystemTime,
    pub last_accessed: Option<SystemTime>,
}
```

## Registration API

### Basic Registration

```rust
use module_registry::{ModuleRegistry, ModuleConfig, ModuleType};

let registry = ModuleRegistry::new();

// Register a basic module
let config = ModuleConfig::new()
    .with_name("data-processor")
    .with_version("1.0.0")
    .with_type(ModuleType::DataProcessor)
    .add_capability("processing")
    .add_capability("validation");

registry.register(config)?;
```

### Advanced Registration

```rust
use module_registry::{ModuleRegistry, ModuleConfig, ModuleType};

let registry = ModuleRegistry::new();

// Register a module with dependencies and metadata
let config = ModuleConfig::new()
    .with_name("advanced-processor")
    .with_version("2.0.0")
    .with_type(ModuleType::DataProcessor)
    .add_dependency("base-processor")
    .add_dependency("validation-module")
    .add_capability("high-performance")
    .add_capability("encryption")
    .with_metadata("author", "John Doe")
    .with_metadata("license", "MIT")
    .with_metadata("description", "Advanced data processor");

registry.register(config)?;
```

### Conditional Registration

```rust
use module_registry::{ModuleRegistry, ConditionalRegistration};

let registry = ModuleRegistry::new()
    .with_conditional_registration(true);

// Register a module conditionally
let conditional_config = ConditionalRegistration::new()
    .with_environment_condition("production")
    .with_capability_condition("high-performance")
    .with_dependency_condition("database-module");

registry.register_conditional(conditional_config)?;
```

## Discovery API

### Basic Discovery

```rust
use module_registry::{ModuleRegistry, DiscoveryQuery};

let registry = ModuleRegistry::new();

// Discover modules by name
let query = DiscoveryQuery::new()
    .with_name_pattern("processor")
    .with_type_filter(ModuleType::DataProcessor);

let modules = registry.discover(query)?;
```

### Capability-Based Discovery

```rust
use module_registry::{ModuleRegistry, CapabilityQuery};

let registry = ModuleRegistry::new();

// Discover modules by capabilities
let query = CapabilityQuery::new()
    .with_required_capabilities(vec!["encryption", "compression"])
    .with_optional_capabilities(vec!["monitoring", "logging"]);

let modules = registry.discover_capabilities(query)?;
```

### Semantic Discovery

```rust
use module_registry::{ModuleRegistry, SemanticQuery};

let registry = ModuleRegistry::new();

// Discover modules semantically
let query = SemanticQuery::new()
    .with_intent("data processing")
    .with_context("high volume")
    .with_requirements(vec!["performance", "scalability"]);

let modules = registry.discover_semantic(query)?;
```

## Instantiation API

### Basic Instantiation

```rust
use module_registry::{ModuleRegistry, InstantiationConfig};

let registry = ModuleRegistry::new();

// Create a module instance
let instance = registry.create("data-processor")?;

// Use the module
let result = instance.process(b"input data")?;
```

### Advanced Instantiation

```rust
use module_registry::{ModuleRegistry, AdvancedInstantiationConfig};

let registry = ModuleRegistry::new();

// Create a module instance with advanced configuration
let config = AdvancedInstantiationConfig::new()
    .with_initialization(true)
    .with_validation(true)
    .with_monitoring(true);

let instance = registry.create_advanced("data-processor", config)?;
```

### Lazy Instantiation

```rust
use module_registry::{ModuleRegistry, LazyInstantiation};

let registry = ModuleRegistry::new()
    .with_lazy_instantiation(true);

// Create a lazy module instance
let lazy_module = registry.create_lazy("data-processor")?;

// Module is only instantiated when first used
let result = lazy_module.process(b"input data")?;
```

## Configuration API

### Basic Configuration

```rust
use module_registry::{ModuleRegistry, Configuration};

let registry = ModuleRegistry::new();

// Configure the registry
let config = Configuration::new()
    .with_caching(true)
    .with_monitoring(true)
    .with_logging(true);

registry.configure(config)?;
```

### Advanced Configuration

```rust
use module_registry::{ModuleRegistry, AdvancedConfiguration};

let registry = ModuleRegistry::new();

// Configure the registry with advanced settings
let config = AdvancedConfiguration::new()
    .with_caching_strategy("LRU")
    .with_cache_size(1000)
    .with_monitoring_level("detailed")
    .with_logging_level("debug")
    .with_security_hardening(true);

registry.configure_advanced(config)?;
```

### Environment Configuration

```rust
use module_registry::{ModuleRegistry, EnvironmentConfiguration};

let registry = ModuleRegistry::new();

// Configure for different environments
let dev_config = EnvironmentConfiguration::new()
    .with_environment("development")
    .with_debug_mode(true)
    .with_verbose_logging(true);

let prod_config = EnvironmentConfiguration::new()
    .with_environment("production")
    .with_debug_mode(false)
    .with_verbose_logging(false);

registry.configure_environment("development", dev_config)?;
registry.configure_environment("production", prod_config)?;
```

## Monitoring API

### Basic Monitoring

```rust
use module_registry::{ModuleRegistry, MonitoringConfig};

let registry = ModuleRegistry::new();

// Configure basic monitoring
let config = MonitoringConfig::new()
    .with_metrics_collection(true)
    .with_logging(true)
    .with_alerting(true);

registry.configure_monitoring(config)?;
```

### Advanced Monitoring

```rust
use module_registry::{ModuleRegistry, AdvancedMonitoringConfig};

let registry = ModuleRegistry::new();

// Configure advanced monitoring
let config = AdvancedMonitoringConfig::new()
    .with_metrics_collection(true)
    .with_performance_metrics(true)
    .with_security_metrics(true)
    .with_health_metrics(true)
    .with_logging(true)
    .with_structured_logging(true)
    .with_alerting(true)
    .with_real_time_alerting(true);

registry.configure_advanced_monitoring(config)?;
```

### Health Monitoring

```rust
use module_registry::{ModuleRegistry, HealthMonitoringConfig};

let registry = ModuleRegistry::new();

// Configure health monitoring
let config = HealthMonitoringConfig::new()
    .with_health_checks(true)
    .with_health_metrics(true)
    .with_health_alerts(true)
    .with_health_dashboard(true);

registry.configure_health_monitoring(config)?;
```

## Security API

### Basic Security

```rust
use module_registry::{ModuleRegistry, SecurityConfig};

let registry = ModuleRegistry::new();

// Configure basic security
let config = SecurityConfig::new()
    .with_secure_registration(true)
    .with_secure_instantiation(true)
    .with_access_control(true);

registry.configure_security(config)?;
```

### Advanced Security

```rust
use module_registry::{ModuleRegistry, AdvancedSecurityConfig};

let registry = ModuleRegistry::new();

// Configure advanced security
let config = AdvancedSecurityConfig::new()
    .with_secure_registration(true)
    .with_secure_instantiation(true)
    .with_secure_communication(true)
    .with_secure_storage(true)
    .with_access_control(true)
    .with_role_based_access(true)
    .with_audit_logging(true);

registry.configure_advanced_security(config)?;
```

### Access Control

```rust
use module_registry::{ModuleRegistry, AccessControlConfig};

let registry = ModuleRegistry::new();

// Configure access control
let config = AccessControlConfig::new()
    .with_role_based_access(true)
    .with_permission_validation(true)
    .with_access_logging(true)
    .with_access_monitoring(true);

registry.configure_access_control(config)?;
```

## Error Types

### RegistryError

Main error type for registry operations.

```rust
#[derive(Debug, thiserror::Error)]
pub enum RegistryError {
    #[error("Module not found: {name}")]
    ModuleNotFound { name: String },
    
    #[error("Module already registered: {name}")]
    ModuleAlreadyRegistered { name: String },
    
    #[error("Invalid module configuration: {reason}")]
    InvalidConfiguration { reason: String },
    
    #[error("Dependency not found: {dependency}")]
    DependencyNotFound { dependency: String },
    
    #[error("Circular dependency detected: {cycle}")]
    CircularDependency { cycle: String },
    
    #[error("Module instantiation failed: {reason}")]
    InstantiationFailed { reason: String },
    
    #[error("Security violation: {violation}")]
    SecurityViolation { violation: String },
    
    #[error("Internal error: {reason}")]
    Internal { reason: String },
}
```

### ModuleError

Error type for module operations.

```rust
#[derive(Debug, thiserror::Error)]
pub enum ModuleError {
    #[error("Initialization failed: {reason}")]
    InitializationFailed { reason: String },
    
    #[error("Processing failed: {reason}")]
    ProcessingFailed { reason: String },
    
    #[error("Cleanup failed: {reason}")]
    CleanupFailed { reason: String },
    
    #[error("Invalid input: {reason}")]
    InvalidInput { reason: String },
    
    #[error("Resource unavailable: {resource}")]
    ResourceUnavailable { resource: String },
    
    #[error("Internal error: {reason}")]
    Internal { reason: String },
}
```

## Utility Functions

### Module Validation

```rust
use module_registry::validation;

// Validate module configuration
let is_valid = validation::validate_config(&config)?;

// Validate module dependencies
let dependencies_valid = validation::validate_dependencies(&registry, &config)?;

// Validate module capabilities
let capabilities_valid = validation::validate_capabilities(&config)?;
```

### Module Utilities

```rust
use module_registry::utils;

// Get module information
let info = utils::get_module_info(&registry, "module-name")?;

// Check module health
let health = utils::check_module_health(&registry, "module-name")?;

// Get module metrics
let metrics = utils::get_module_metrics(&registry, "module-name")?;
```

### Module Discovery Utilities

```rust
use module_registry::discovery;

// Find modules by pattern
let modules = discovery::find_by_pattern(&registry, "processor*")?;

// Find modules by type
let modules = discovery::find_by_type(&registry, ModuleType::DataProcessor)?;

// Find modules by capability
let modules = discovery::find_by_capability(&registry, "encryption")?;
```

## Conclusion

This API reference provides comprehensive documentation for all the Module Registry APIs. By using these APIs, you can create sophisticated module-based architectures with advanced features like dynamic registration, semantic discovery, and comprehensive monitoring.

Key API categories:

1. **Core Types**: Fundamental types and traits
2. **Registration API**: Module registration and management
3. **Discovery API**: Module discovery and querying
4. **Instantiation API**: Module instantiation and usage
5. **Configuration API**: Registry configuration and customization
6. **Monitoring API**: Monitoring and observability
7. **Security API**: Security and access control
8. **Error Types**: Comprehensive error handling
9. **Utility Functions**: Helper functions and utilities
