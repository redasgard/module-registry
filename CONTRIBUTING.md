# Contributing to Module Registry

Thank you for your interest in contributing to Module Registry! This document provides guidelines and information for contributors.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [How to Contribute](#how-to-contribute)
- [Development Setup](#development-setup)
- [Testing](#testing)
- [Security](#security)
- [Documentation](#documentation)
- [Release Process](#release-process)

## Code of Conduct

This project follows the [Contributor Covenant Code of Conduct](CODE_OF_CONDUCT.md). By participating, you agree to uphold this code.

## Getting Started

### Prerequisites

- Rust 1.70+ (latest stable recommended)
- Git
- Understanding of plugin systems and dynamic loading
- Familiarity with Rust macros and procedural programming
- Basic knowledge of module systems and dependency injection

### Fork and Clone

1. Fork the repository on GitHub
2. Clone your fork locally:
   ```bash
   git clone https://github.com/YOUR_USERNAME/module-registry.git
   cd module-registry
   ```
3. Add the upstream remote:
   ```bash
   git remote add upstream https://github.com/redasgard/module-registry.git
   ```

## How to Contribute

### Reporting Issues

Before creating an issue, please:

1. **Search existing issues** to avoid duplicates
2. **Check the documentation** in the `docs/` folder
3. **Verify the issue** with the latest version
4. **Test with minimal examples**

When creating an issue, include:

- **Clear description** of the problem
- **Steps to reproduce** with code examples
- **Expected vs actual behavior**
- **Environment details** (OS, Rust version, plugin setup)
- **Module-specific details** (if related to specific modules)

### Suggesting Enhancements

For feature requests:

1. **Check existing issues** and roadmap
2. **Describe the use case** clearly
3. **Explain the architectural benefit**
4. **Consider implementation complexity**
5. **Provide module examples** if applicable

### Pull Requests

#### Before You Start

1. **Open an issue first** for significant changes
2. **Discuss the approach** with maintainers
3. **Ensure the change aligns** with project goals
4. **Consider plugin compatibility** implications

#### PR Process

1. **Create a feature branch** from `main`:
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make your changes** following our guidelines

3. **Test thoroughly**:
   ```bash
   cargo test
   cargo test --features tracing
   cargo clippy
   cargo fmt
   ```

4. **Update documentation** if needed

5. **Commit with clear messages**:
   ```bash
   git commit -m "Add support for async module initialization"
   ```

6. **Push and create PR**:
   ```bash
   git push origin feature/your-feature-name
   ```

#### PR Requirements

- **All tests pass** (CI will check)
- **Code is formatted** (`cargo fmt`)
- **No clippy warnings** (`cargo clippy`)
- **Documentation updated** if needed
- **Clear commit messages**
- **PR description** explains the change
- **Plugin compatibility** maintained

## Development Setup

### Project Structure

```
module-registry/
├── src/                 # Source code
│   ├── lib.rs          # Main library interface
│   ├── registry.rs     # Registry management
│   ├── macros.rs       # Procedural macros
│   ├── security.rs     # Security features
│   └── types.rs        # Type definitions
├── tests/              # Integration tests
├── examples/           # Usage examples
└── docs/               # Documentation
```

### Running Tests

```bash
# Run all tests
cargo test

# Run with tracing
cargo test --features tracing

# Run specific test
cargo test test_module_registration

# Run examples
cargo run --example plugin_system
```

### Code Style

We follow standard Rust conventions:

- **Format code**: `cargo fmt`
- **Check linting**: `cargo clippy`
- **Use meaningful names**
- **Add documentation** for public APIs
- **Write tests** for new functionality
- **Consider macro performance**

## Testing

### Test Categories

1. **Unit Tests**: Test individual functions
2. **Integration Tests**: Test complete workflows
3. **Macro Tests**: Test procedural macros
4. **Plugin Tests**: Test with real plugins
5. **Security Tests**: Test security features

### Adding Tests

When adding new functionality:

1. **Write unit tests** for each function
2. **Add integration tests** for workflows
3. **Test macros** with various inputs
4. **Test plugin compatibility**
5. **Test security features**

Example test structure:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_registration() {
        let registry = ModuleRegistry::new();
        
        // Test module registration
        let result = registry.register_module("test_module", &capabilities);
        assert!(result.is_ok());
    }

    #[test]
    fn test_macro_derivation() {
        #[derive(ModuleCapabilities)]
        struct TestModule {
            name: String,
            capabilities: Vec<String>,
        }
        
        // Test macro functionality
        let module = TestModule {
            name: "test".to_string(),
            capabilities: vec!["test_capability".to_string()],
        };
        
        // Test macro-generated functionality
        assert_eq!(module.name, "test");
    }
}
```

## Security

### Security Considerations

Module Registry is a security-critical library. When contributing:

1. **Understand plugin security** before making changes
2. **Test with malicious plugins** (safely)
3. **Consider sandboxing** implications
4. **Review security implications** of changes
5. **Test with various plugin types**

### Security Testing

```bash
# Run security tests
cargo test test_plugin_security
cargo test test_capability_validation
cargo test test_malicious_plugin_handling

# Test with examples
cargo run --example plugin_system
```

### Plugin Security

When adding security features:

1. **Research plugin security** best practices
2. **Understand sandboxing** techniques
3. **Test with malicious inputs**
4. **Consider capability-based security**
5. **Document security implications**

### Reporting Security Issues

**Do not open public issues for security vulnerabilities.**

Instead:
1. Email security@redasgard.com
2. Include detailed description
3. Include plugin examples
4. Wait for response before disclosure

## Documentation

### Documentation Standards

- **Public APIs** must have doc comments
- **Examples** in doc comments should be runnable
- **Security implications** should be documented
- **Performance characteristics** should be noted
- **Macro usage** should be documented

### Documentation Structure

```
docs/
├── README.md              # Main documentation
├── getting-started.md      # Quick start guide
├── api-reference.md       # Complete API docs
├── plugin-development.md  # Plugin development guide
├── best-practices.md      # Usage guidelines
└── faq.md                 # Frequently asked questions
```

### Writing Documentation

1. **Use clear, concise language**
2. **Include practical examples**
3. **Explain security implications**
4. **Document macro usage**
5. **Link to related resources**
6. **Keep it up to date**

## Release Process

### Versioning

We follow [Semantic Versioning](https://semver.org/):

- **MAJOR**: Breaking API changes
- **MINOR**: New features (backward compatible)
- **PATCH**: Bug fixes (backward compatible)

### Release Checklist

Before releasing:

- [ ] All tests pass
- [ ] Documentation updated
- [ ] CHANGELOG.md updated
- [ ] Version bumped in Cargo.toml
- [ ] Security review completed
- [ ] Performance benchmarks updated
- [ ] Plugin compatibility tested

### Release Steps

1. **Update version** in `Cargo.toml`
2. **Update CHANGELOG.md**
3. **Create release PR**
4. **Review and merge**
5. **Tag release** on GitHub
6. **Publish to crates.io**

## Areas for Contribution

### High Priority

- **Plugin security**: Improve sandboxing and capability-based security
- **Performance improvements**: Optimize macro compilation and plugin loading
- **Async support**: Better async plugin initialization
- **Plugin discovery**: Improve plugin discovery mechanisms

### Medium Priority

- **Configuration options**: More flexible plugin configuration
- **Error handling**: Better error messages and recovery
- **Testing**: More comprehensive test coverage
- **Documentation**: Improve examples and guides

### Low Priority

- **CLI tools**: Command-line utilities for plugin management
- **IDE integration**: Editor plugins for development
- **Visualization**: Plugin dependency visualization tools
- **Hot reloading**: Runtime plugin reloading

## Plugin Development

### Plugin Categories

1. **Security Plugins**: Security analysis and validation
2. **Analysis Plugins**: Code analysis and processing
3. **Integration Plugins**: External service integration
4. **Utility Plugins**: General-purpose utilities

### Plugin Development Process

1. **Design**: Plan the plugin architecture
2. **Implement**: Create the plugin code
3. **Test**: Test with the registry
4. **Validate**: Ensure security and compatibility
5. **Document**: Document the plugin and its capabilities
6. **Publish**: Make the plugin available

### Plugin Testing

```rust
// Test new plugin
#[test]
fn test_new_plugin() {
    let registry = ModuleRegistry::new();
    
    // Test plugin registration
    let result = registry.register_plugin("new_plugin", &capabilities);
    assert!(result.is_ok());
    
    // Test plugin functionality
    let plugin = registry.get_plugin("new_plugin");
    assert!(plugin.is_some());
}
```

## Getting Help

### Resources

- **Documentation**: Check the `docs/` folder
- **Examples**: Look at `examples/` folder
- **Issues**: Search existing GitHub issues
- **Discussions**: Use GitHub Discussions for questions

### Contact

- **Email**: hello@redasgard.com
- **GitHub**: [@redasgard](https://github.com/redasgard)
- **Security**: security@redasgard.com

## Recognition

Contributors will be:

- **Listed in CONTRIBUTORS.md**
- **Mentioned in release notes** for significant contributions
- **Credited in documentation** for major features
- **Acknowledged** for plugin development

Thank you for contributing to Module Registry! 🔌🛡️
