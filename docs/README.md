# Module Registry Documentation

Dynamic module/plugin registry with compile-time discovery and runtime instantiation for Rust.

## Documentation Structure

- **[Architecture](./architecture.md)** - Registry design and patterns
- **[Getting Started](./getting-started.md)** - Quick start guide
- **[User Guide](./user-guide.md)** - Comprehensive usage patterns
- **[API Reference](./api-reference.md)** - Detailed API documentation
- **[Plugin Development](./plugin-development.md)** - Creating plugins
- **[Advanced Patterns](./advanced-patterns.md)** - Advanced techniques
- **[Testing Guide](./testing.md)** - Testing plugins
- **[FAQ](./faq.md)** - Frequently asked questions

## Quick Links

- [Why Module Registry?](./why-module-registry.md)
- [Use Cases](./use-cases.md)
- [Best Practices](./best-practices.md)
- [Examples](./examples.md)

## Overview

Module Registry provides a type-safe, thread-safe registry for dynamic module loading and instantiation in Rust applications.

### Key Features

- ✅ **Compile-Time Discovery**: Automatic registration using `inventory`
- ✅ **Runtime Instantiation**: Create modules dynamically by name
- ✅ **Type-Safe**: Generic factory functions with trait objects
- ✅ **Thread-Safe**: Built on `RwLock` for concurrent access
- ✅ **Zero-Cost**: Minimal runtime overhead
- ✅ **Flexible**: Works with any module type

### Quick Example

```rust
use module_registry::{ModuleRegistry, Module};
use anyhow::Result;

// Define module trait
pub trait TextProcessor: Module {
    fn process(&self, input: &str) -> Result<String>;
}

// Implement module
struct UpperCaseProcessor;

impl Module for UpperCaseProcessor {
    fn name(&self) -> &str { "uppercase" }
    fn module_type(&self) -> &str { "text_processor" }
}

impl TextProcessor for UpperCaseProcessor {
    fn process(&self, input: &str) -> Result<String> {
        Ok(input.to_uppercase())
    }
}

fn main() -> Result<()> {
    // Create registry
    let registry = ModuleRegistry::new();
    
    // Register module
    registry.register("uppercase", "text_processor", || {
        Ok(Box::new(Box::new(UpperCaseProcessor) as Box<dyn TextProcessor>))
    });
    
    // Create instance
    let module = registry.create_any("uppercase")?;
    
    Ok(())
}
```

## Use Cases

- Plugin systems
- Service locator pattern
- Provider pattern
- Dynamic feature loading
- Extensible applications

## Support

- **GitHub**: https://github.com/redasgard/module-registry
- **Email**: hello@redasgard.com

## License

MIT License - See [LICENSE-MIT](../LICENSE-MIT)

