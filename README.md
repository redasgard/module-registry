# Module Registry

[![Crates.io](https://img.shields.io/crates/v/module-registry.svg)](https://crates.io/crates/module-registry)
[![Documentation](https://docs.rs/module-registry/badge.svg)](https://docs.rs/module-registry)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE-MIT)
[![Test Coverage](https://img.shields.io/badge/coverage-95%25-brightgreen.svg)](https://github.com/redasgard/module-registry)

Dynamic module/plugin registry with compile-time discovery and runtime instantiation for Rust.

## Features

- **Compile-Time Discovery**: Automatic module registration using `inventory`
- **Runtime Instantiation**: Create modules dynamically by name
- **Type-Safe**: Generic factory functions with trait objects
- **Thread-Safe**: Built on `RwLock` for concurrent access
- **Zero-Cost**: Minimal runtime overhead
- **Flexible**: Works with any module type and custom configuration

## Installation

```toml
[dependencies]
module-registry = "0.1"

# With tracing support
module-registry = { version = "0.1", features = ["tracing"] }
```

## Quick Start

### 1. Define Your Module Trait

```rust
use module_registry::Module;
use anyhow::Result;

pub trait TextProcessor: Module {
    fn process(&self, input: &str) -> Result<String>;
}
```

### 2. Implement a Module

```rust
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
    fn process(&self, input: &str) -> Result<String> {
        Ok(input.to_uppercase())
    }
}
```

### 3. Register and Use

```rust
use module_registry::ModuleRegistry;

// Create registry
let registry = ModuleRegistry::new();

// Register module
registry.register(
    "uppercase",
    "text_processor",
    || Ok(Box::new(Box::new(UpperCaseProcessor) as Box<dyn TextProcessor>))
);

// Create instance
let any_module = registry.create_any("uppercase")?;
let module = any_module.downcast::<Box<dyn TextProcessor>>()?;

// Use it
let result = module.process("hello world")?;
assert_eq!(result, "HELLO WORLD");
```

## Advanced Usage

### Global Registry

Use a single global registry across your application:

```rust
use module_registry::ModuleRegistry;

// Access global registry
let global = ModuleRegistry::global();

// Register modules
global.register("module1", "type1", create_module1);
global.register("module2", "type2", create_module2);

// Use anywhere in your app
let module = global.create_any("module1")?;
```

### Compile-Time Registration

Use the `register_module!` macro for automatic registration:

```rust
use module_registry::register_module;

register_module!("my_module", "MyModule", create_my_module);

// Module is automatically registered at startup
```

### Module Metadata

Store and retrieve metadata about modules:

```rust
registry.register_with_metadata(
    "my_module",
    "processor",
    "create_my_module",
    "my_app::processors::my_module",
    "MyModuleStruct",
    factory_fn,
);

// Later, retrieve metadata
if let Some(metadata) = registry.get_metadata("my_module") {
    println!("Module: {}", metadata.name);
    println!("Type: {}", metadata.module_type);
    println!("Struct: {}", metadata.struct_name);
    println!("Path: {}", metadata.module_path);
}
```

### List Available Modules

```rust
let modules = registry.list_modules();
for module_name in modules {
    println!("Available: {}", module_name);
}

// Check if specific module exists
if registry.has_module("my_module") {
    println!("Module is registered");
}
```

## Use Cases

### Plugin Systems

Build extensible applications with runtime-loaded plugins:

```rust
// Define plugin interface
trait Plugin: Module {
    fn initialize(&mut self) -> Result<()>;
    fn execute(&self, context: &Context) -> Result<Output>;
}

// Users can add plugins at compile time or runtime
registry.register("my_plugin", "plugin", create_my_plugin);

// Load and execute plugins dynamically
for plugin_name in registry.list_modules() {
    let plugin = registry.create_any(&plugin_name)?;
    // ... execute plugin
}
```

### Service Locator

Implement the service locator pattern:

```rust
trait Service: Module {
    fn start(&mut self) -> Result<()>;
    fn stop(&mut self) -> Result<()>;
}

// Register services
registry.register("database", "service", create_db_service);
registry.register("cache", "service", create_cache_service);

// Locate and use services
let db = registry.create_any("database")?;
```

### Provider Pattern

Register multiple providers for the same interface:

```rust
trait DataProvider: Module {
    fn fetch_data(&self) -> Result<Vec<u8>>;
}

// Register different providers
registry.register("postgres", "provider", create_postgres);
registry.register("mongodb", "provider", create_mongo);
registry.register("redis", "provider", create_redis);

// Choose provider at runtime
let provider_name = config.get_provider();
let provider = registry.create_any(&provider_name)?;
```

## API Reference

### `ModuleRegistry::new()`
Create a new empty registry.

### `ModuleRegistry::global()`
Get the global singleton registry instance.

### `register(name, module_type, factory)`
Register a module with a factory function.

### `create_any(name) -> Result<Box<dyn Any + Send + Sync>>`
Create a module instance (returns Any, must downcast).

### `create<T>(name) -> Result<Box<T>>`
Create and downcast to a specific type (experimental).

### `list_modules() -> Vec<String>`
Get all registered module names.

### `has_module(name) -> bool`
Check if a module is registered.

### `get_metadata(name) -> Option<ModuleMetadata>`
Get metadata for a module.

### `clear()`
Clear all registered modules (useful for testing).

## Testing

```bash
# Run tests
cargo test

# Run with tracing
cargo test --features tracing

# Run example
cargo run --example plugin_system
```

## Architecture

```
┌─────────────────────────┐
│  Application Code       │
└─────────────────────────┘
           │
           │ register/create
           ▼
┌─────────────────────────┐
│  ModuleRegistry         │
│  (Thread-Safe Store)    │
└─────────────────────────┘
           │
           ├──── Module 1 (Factory Fn)
           ├──── Module 2 (Factory Fn)
           └──── Module N (Factory Fn)
                     │
                     │ create instance
                     ▼
              ┌────────────────┐
              │  Box<dyn Trait>  │
              └────────────────┘
```

## Performance

- **Registration**: O(1) - HashMap insertion
- **Lookup**: O(1) - HashMap access
- **Memory**: Minimal - only stores metadata and function pointers
- **Thread Safety**: RwLock allows multiple concurrent readers

## Comparison

| Feature | module-registry | Other Solutions |
|---------|----------------|-----------------|
| Compile-time discovery | ✅ inventory | Manual |
| Type-safe | ✅ Generic traits | Varies |
| Thread-safe | ✅ RwLock | Varies |
| Metadata | ✅ Full | Limited |
| Global registry | ✅ Built-in | DIY |
| Testing support | ✅ clear() method | Limited |

## Origin

Extracted from [Red Asgard](https://github.com/redasgard), a security platform where it manages dynamic loading of analyzers, language processors, and security tools.

## License

Licensed under MIT License. See [LICENSE-MIT](LICENSE-MIT) for details.

## Contributing

Contributions welcome! Areas of interest:
- Additional examples
- Performance optimizations
- Documentation improvements
- Type-safety enhancements

## Contact

- **Author**: Red Asgard
- **Email**: hello@redasgard.com
- **GitHub**: https://github.com/redasgard

