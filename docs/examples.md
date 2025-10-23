# Examples - Module Registry

## Overview

This document provides comprehensive examples for using the Module Registry in various scenarios.

## Basic Examples

### Simple Module Registration

```rust
use module_registry::{ModuleRegistry, ModuleConfig, ModuleType};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new module registry
    let registry = ModuleRegistry::new();
    
    // Register a simple module
    let config = ModuleConfig::new()
        .with_name("data-processor")
        .with_version("1.0.0")
        .with_type(ModuleType::DataProcessor)
        .add_capability("processing");
    
    registry.register(config)?;
    
    // Check if module is registered
    if registry.is_registered("data-processor") {
        println!("✅ Module 'data-processor' is registered");
    }
    
    Ok(())
}
```

### Module Discovery

```rust
use module_registry::{ModuleRegistry, ModuleConfig, ModuleType, DiscoveryQuery};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let registry = ModuleRegistry::new();
    
    // Register multiple modules
    let modules = vec![
        ("data-processor", ModuleType::DataProcessor),
        ("data-analyzer", ModuleType::Analyzer),
        ("data-storage", ModuleType::Storage),
    ];
    
    for (name, module_type) in modules {
        let config = ModuleConfig::new()
            .with_name(name)
            .with_version("1.0.0")
            .with_type(module_type);
        
        registry.register(config)?;
    }
    
    // Discover modules by type
    let query = DiscoveryQuery::new()
        .with_type_filter(ModuleType::DataProcessor);
    
    let found_modules = registry.discover(query)?;
    println!("Found {} data processor modules", found_modules.len());
    
    Ok(())
}
```

### Module Instantiation

```rust
use module_registry::{ModuleRegistry, ModuleConfig, ModuleType};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let registry = ModuleRegistry::new();
    
    // Register a module
    let config = ModuleConfig::new()
        .with_name("data-processor")
        .with_version("1.0.0")
        .with_type(ModuleType::DataProcessor);
    
    registry.register(config)?;
    
    // Create a module instance
    let instance = registry.create("data-processor")?;
    
    // Use the module
    let data = b"Hello, World!";
    let result = instance.process(data)?;
    
    println!("Processed data: {:?}", result);
    
    Ok(())
}
```

## Advanced Examples

### Module with Dependencies

```rust
use module_registry::{ModuleRegistry, ModuleConfig, ModuleType};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let registry = ModuleRegistry::new();
    
    // Register base module first
    let base_config = ModuleConfig::new()
        .with_name("base-module")
        .with_version("1.0.0")
        .with_type(ModuleType::DataProcessor);
    
    registry.register(base_config)?;
    
    // Register dependent module
    let dependent_config = ModuleConfig::new()
        .with_name("dependent-module")
        .with_version("1.0.0")
        .with_type(ModuleType::DataProcessor)
        .add_dependency("base-module");
    
    registry.register(dependent_config)?;
    
    // Create instances
    let base_instance = registry.create("base-module")?;
    let dependent_instance = registry.create("dependent-module")?;
    
    // Use modules
    let data = b"input data";
    let base_result = base_instance.process(data)?;
    let dependent_result = dependent_instance.process(&base_result)?;
    
    println!("Base result: {:?}", base_result);
    println!("Dependent result: {:?}", dependent_result);
    
    Ok(())
}
```

### Capability-Based Discovery

```rust
use module_registry::{ModuleRegistry, ModuleConfig, ModuleType, CapabilityQuery};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let registry = ModuleRegistry::new();
    
    // Register modules with different capabilities
    let modules = vec![
        ("encryption-module", vec!["encryption", "decryption"]),
        ("compression-module", vec!["compression", "decompression"]),
        ("monitoring-module", vec!["monitoring", "logging"]),
    ];
    
    for (name, capabilities) in modules {
        let config = ModuleConfig::new()
            .with_name(name)
            .with_version("1.0.0")
            .with_type(ModuleType::Security);
        
        for capability in capabilities {
            config.add_capability(capability);
        }
        
        registry.register(config)?;
    }
    
    // Discover modules by capabilities
    let query = CapabilityQuery::new()
        .with_required_capabilities(vec!["encryption"])
        .with_optional_capabilities(vec!["monitoring"]);
    
    let found_modules = registry.discover_capabilities(query)?;
    println!("Found {} modules with encryption capability", found_modules.len());
    
    Ok(())
}
```

### Semantic Discovery

```rust
use module_registry::{ModuleRegistry, ModuleConfig, ModuleType, SemanticQuery};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let registry = ModuleRegistry::new();
    
    // Register modules with metadata
    let modules = vec![
        ("high-performance-processor", "High-performance data processor"),
        ("low-latency-processor", "Low-latency data processor"),
        ("batch-processor", "Batch data processor"),
    ];
    
    for (name, description) in modules {
        let config = ModuleConfig::new()
            .with_name(name)
            .with_version("1.0.0")
            .with_type(ModuleType::DataProcessor)
            .with_metadata("description", description);
        
        registry.register(config)?;
    }
    
    // Discover modules semantically
    let query = SemanticQuery::new()
        .with_intent("data processing")
        .with_context("high performance")
        .with_requirements(vec!["performance", "scalability"]);
    
    let found_modules = registry.discover_semantic(query)?;
    println!("Found {} high-performance modules", found_modules.len());
    
    Ok(())
}
```

## Configuration Examples

### Basic Configuration

```rust
use module_registry::{ModuleRegistry, Configuration};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let registry = ModuleRegistry::new();
    
    // Configure the registry
    let config = Configuration::new()
        .with_caching(true)
        .with_monitoring(true)
        .with_logging(true);
    
    registry.configure(config)?;
    
    println!("Registry configured with basic settings");
    
    Ok(())
}
```

### Advanced Configuration

```rust
use module_registry::{ModuleRegistry, AdvancedConfiguration};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let registry = ModuleRegistry::new();
    
    // Configure the registry with advanced settings
    let config = AdvancedConfiguration::new()
        .with_caching_strategy("LRU")
        .with_cache_size(1000)
        .with_monitoring_level("detailed")
        .with_logging_level("debug")
        .with_security_hardening(true);
    
    registry.configure_advanced(config)?;
    
    println!("Registry configured with advanced settings");
    
    Ok(())
}
```

### Environment-Specific Configuration

```rust
use module_registry::{ModuleRegistry, EnvironmentConfiguration};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let registry = ModuleRegistry::new();
    
    // Configure for development environment
    let dev_config = EnvironmentConfiguration::new()
        .with_environment("development")
        .with_debug_mode(true)
        .with_verbose_logging(true)
        .with_mock_services(true);
    
    // Configure for production environment
    let prod_config = EnvironmentConfiguration::new()
        .with_environment("production")
        .with_debug_mode(false)
        .with_verbose_logging(false)
        .with_real_services(true);
    
    registry.configure_environment("development", dev_config)?;
    registry.configure_environment("production", prod_config)?;
    
    println!("Registry configured for multiple environments");
    
    Ok(())
}
```

## Performance Examples

### Caching Configuration

```rust
use module_registry::{ModuleRegistry, CachingConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let registry = ModuleRegistry::new();
    
    // Configure caching
    let cache_config = CachingConfig::new()
        .with_cache_type("LRU")
        .with_cache_size(1000)
        .with_cache_ttl(3600)
        .with_cache_compression(true);
    
    registry.configure_caching(cache_config)?;
    
    println!("Registry configured with caching");
    
    Ok(())
}
```

### Performance Optimization

```rust
use module_registry::{ModuleRegistry, PerformanceOptimizationConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let registry = ModuleRegistry::new();
    
    // Configure performance optimization
    let optimization_config = PerformanceOptimizationConfig::new()
        .with_algorithm_optimization(true)
        .with_data_structure_optimization(true)
        .with_memory_optimization(true)
        .with_cpu_optimization(true);
    
    registry.configure_performance_optimization(optimization_config)?;
    
    println!("Registry configured with performance optimization");
    
    Ok(())
}
```

### Resource Management

```rust
use module_registry::{ModuleRegistry, ResourceManagementConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let registry = ModuleRegistry::new();
    
    // Configure resource management
    let resource_config = ResourceManagementConfig::new()
        .with_memory_limit(1024 * 1024 * 1024) // 1GB
        .with_cpu_limit(80) // 80%
        .with_io_limit(1000) // 1000 IOPS
        .with_network_limit(100 * 1024 * 1024); // 100MB/s
    
    registry.configure_resource_management(resource_config)?;
    
    println!("Registry configured with resource management");
    
    Ok(())
}
```

## Security Examples

### Basic Security

```rust
use module_registry::{ModuleRegistry, SecurityConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let registry = ModuleRegistry::new();
    
    // Configure basic security
    let security_config = SecurityConfig::new()
        .with_secure_registration(true)
        .with_secure_instantiation(true)
        .with_access_control(true);
    
    registry.configure_security(security_config)?;
    
    println!("Registry configured with basic security");
    
    Ok(())
}
```

### Advanced Security

```rust
use module_registry::{ModuleRegistry, AdvancedSecurityConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let registry = ModuleRegistry::new();
    
    // Configure advanced security
    let security_config = AdvancedSecurityConfig::new()
        .with_secure_registration(true)
        .with_secure_instantiation(true)
        .with_secure_communication(true)
        .with_secure_storage(true)
        .with_access_control(true)
        .with_role_based_access(true)
        .with_audit_logging(true);
    
    registry.configure_advanced_security(security_config)?;
    
    println!("Registry configured with advanced security");
    
    Ok(())
}
```

### Access Control

```rust
use module_registry::{ModuleRegistry, AccessControlConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let registry = ModuleRegistry::new();
    
    // Configure access control
    let access_config = AccessControlConfig::new()
        .with_role_based_access(true)
        .with_permission_validation(true)
        .with_access_logging(true)
        .with_access_monitoring(true);
    
    registry.configure_access_control(access_config)?;
    
    println!("Registry configured with access control");
    
    Ok(())
}
```

## Monitoring Examples

### Basic Monitoring

```rust
use module_registry::{ModuleRegistry, MonitoringConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let registry = ModuleRegistry::new();
    
    // Configure basic monitoring
    let monitoring_config = MonitoringConfig::new()
        .with_metrics_collection(true)
        .with_logging(true)
        .with_alerting(true);
    
    registry.configure_monitoring(monitoring_config)?;
    
    println!("Registry configured with basic monitoring");
    
    Ok(())
}
```

### Advanced Monitoring

```rust
use module_registry::{ModuleRegistry, AdvancedMonitoringConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let registry = ModuleRegistry::new();
    
    // Configure advanced monitoring
    let monitoring_config = AdvancedMonitoringConfig::new()
        .with_metrics_collection(true)
        .with_performance_metrics(true)
        .with_security_metrics(true)
        .with_health_metrics(true)
        .with_logging(true)
        .with_structured_logging(true)
        .with_alerting(true)
        .with_real_time_alerting(true);
    
    registry.configure_advanced_monitoring(monitoring_config)?;
    
    println!("Registry configured with advanced monitoring");
    
    Ok(())
}
```

### Health Monitoring

```rust
use module_registry::{ModuleRegistry, HealthMonitoringConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let registry = ModuleRegistry::new();
    
    // Configure health monitoring
    let health_config = HealthMonitoringConfig::new()
        .with_health_checks(true)
        .with_health_metrics(true)
        .with_health_alerts(true)
        .with_health_dashboard(true);
    
    registry.configure_health_monitoring(health_config)?;
    
    println!("Registry configured with health monitoring");
    
    Ok(())
}
```

## Real-World Examples

### Web Application Module System

```rust
use module_registry::{ModuleRegistry, ModuleConfig, ModuleType};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let registry = ModuleRegistry::new();
    
    // Register web application modules
    let web_modules = vec![
        ("auth-module", ModuleType::Security, vec!["authentication", "authorization"]),
        ("user-module", ModuleType::DataProcessor, vec!["user-management", "profile"]),
        ("content-module", ModuleType::DataProcessor, vec!["content-management", "publishing"]),
        ("api-module", ModuleType::Network, vec!["api-gateway", "routing"]),
    ];
    
    for (name, module_type, capabilities) in web_modules {
        let config = ModuleConfig::new()
            .with_name(name)
            .with_version("1.0.0")
            .with_type(module_type);
        
        for capability in capabilities {
            config.add_capability(capability);
        }
        
        registry.register(config)?;
    }
    
    // Use modules in web application
    let auth_module = registry.create("auth-module")?;
    let user_module = registry.create("user-module")?;
    let content_module = registry.create("content-module")?;
    let api_module = registry.create("api-module")?;
    
    println!("Web application modules registered and ready");
    
    Ok(())
}
```

### Microservice Architecture

```rust
use module_registry::{ModuleRegistry, ModuleConfig, ModuleType};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let registry = ModuleRegistry::new();
    
    // Register microservice modules
    let microservices = vec![
        ("user-service", "User management service"),
        ("order-service", "Order processing service"),
        ("payment-service", "Payment processing service"),
        ("notification-service", "Notification service"),
    ];
    
    for (name, description) in microservices {
        let config = ModuleConfig::new()
            .with_name(name)
            .with_version("1.0.0")
            .with_type(ModuleType::Network)
            .with_metadata("description", description)
            .with_metadata("service_type", "microservice");
        
        registry.register(config)?;
    }
    
    // Use microservices
    let user_service = registry.create("user-service")?;
    let order_service = registry.create("order-service")?;
    let payment_service = registry.create("payment-service")?;
    let notification_service = registry.create("notification-service")?;
    
    println!("Microservice modules registered and ready");
    
    Ok(())
}
```

### Plugin Architecture

```rust
use module_registry::{ModuleRegistry, ModuleConfig, ModuleType};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let registry = ModuleRegistry::new();
    
    // Register plugin modules
    let plugins = vec![
        ("data-processor-plugin", "Data processing plugin"),
        ("encryption-plugin", "Encryption plugin"),
        ("compression-plugin", "Compression plugin"),
        ("monitoring-plugin", "Monitoring plugin"),
    ];
    
    for (name, description) in plugins {
        let config = ModuleConfig::new()
            .with_name(name)
            .with_version("1.0.0")
            .with_type(ModuleType::DataProcessor)
            .with_metadata("description", description)
            .with_metadata("plugin_type", "extension");
        
        registry.register(config)?;
    }
    
    // Use plugins
    let data_processor = registry.create("data-processor-plugin")?;
    let encryption = registry.create("encryption-plugin")?;
    let compression = registry.create("compression-plugin")?;
    let monitoring = registry.create("monitoring-plugin")?;
    
    println!("Plugin modules registered and ready");
    
    Ok(())
}
```

## Error Handling Examples

### Basic Error Handling

```rust
use module_registry::{ModuleRegistry, ModuleConfig, ModuleType, RegistryError};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let registry = ModuleRegistry::new();
    
    // Register a module
    let config = ModuleConfig::new()
        .with_name("data-processor")
        .with_version("1.0.0")
        .with_type(ModuleType::DataProcessor);
    
    match registry.register(config) {
        Ok(_) => println!("✅ Module registered successfully"),
        Err(RegistryError::ModuleAlreadyRegistered { name }) => {
            println!("❌ Module '{}' is already registered", name);
        }
        Err(error) => {
            println!("❌ Registration failed: {}", error);
        }
    }
    
    Ok(())
}
```

### Advanced Error Handling

```rust
use module_registry::{ModuleRegistry, ModuleConfig, ModuleType, RegistryError};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let registry = ModuleRegistry::new();
    
    // Register a module with error handling
    let config = ModuleConfig::new()
        .with_name("data-processor")
        .with_version("1.0.0")
        .with_type(ModuleType::DataProcessor);
    
    match registry.register(config) {
        Ok(_) => {
            println!("✅ Module registered successfully");
            
            // Try to create an instance
            match registry.create("data-processor") {
                Ok(instance) => {
                    println!("✅ Module instance created successfully");
                    
                    // Try to process data
                    let data = b"test data";
                    match instance.process(data) {
                        Ok(result) => {
                            println!("✅ Data processed successfully: {:?}", result);
                        }
                        Err(error) => {
                            println!("❌ Data processing failed: {}", error);
                        }
                    }
                }
                Err(RegistryError::ModuleNotFound { name }) => {
                    println!("❌ Module '{}' not found", name);
                }
                Err(error) => {
                    println!("❌ Module instantiation failed: {}", error);
                }
            }
        }
        Err(RegistryError::ModuleAlreadyRegistered { name }) => {
            println!("❌ Module '{}' is already registered", name);
        }
        Err(RegistryError::InvalidConfiguration { reason }) => {
            println!("❌ Invalid configuration: {}", reason);
        }
        Err(RegistryError::DependencyNotFound { dependency }) => {
            println!("❌ Dependency '{}' not found", dependency);
        }
        Err(RegistryError::CircularDependency { cycle }) => {
            println!("❌ Circular dependency detected: {}", cycle);
        }
        Err(RegistryError::InstantiationFailed { reason }) => {
            println!("❌ Module instantiation failed: {}", reason);
        }
        Err(RegistryError::SecurityViolation { violation }) => {
            println!("❌ Security violation: {}", violation);
        }
        Err(RegistryError::Internal { reason }) => {
            println!("❌ Internal error: {}", reason);
        }
    }
    
    Ok(())
}
```

## Conclusion

These examples demonstrate the comprehensive capabilities of the Module Registry across various scenarios and use cases. By following these examples, you can implement sophisticated module-based architectures with advanced features like dynamic registration, semantic discovery, and comprehensive monitoring.

Key example categories:

1. **Basic Examples**: Simple module registration, discovery, and instantiation
2. **Advanced Examples**: Complex scenarios with dependencies and capabilities
3. **Configuration Examples**: Various configuration options and settings
4. **Performance Examples**: Performance optimization and resource management
5. **Security Examples**: Security configuration and access control
6. **Monitoring Examples**: Monitoring and observability setup
7. **Real-World Examples**: Practical applications and use cases
8. **Error Handling Examples**: Comprehensive error handling strategies
