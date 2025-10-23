//! Example demonstrating a plugin system using module-registry

use anyhow::Result;
use module_registry::{Module, ModuleRegistry};

// Define a plugin interface
trait Plugin: Module {
    fn initialize(&self) -> Result<()>;
    fn execute(&self, input: &str) -> Result<String>;
    fn version(&self) -> &str;
}

// Implement a simple plugin
struct EchoPlugin {
    name: String,
    version: String,
}

impl Module for EchoPlugin {
    fn name(&self) -> &str {
        &self.name
    }

    fn module_type(&self) -> &str {
        "plugin"
    }
}

impl Plugin for EchoPlugin {
    fn initialize(&self) -> Result<()> {
        println!("  Initializing {} v{}", self.name, self.version);
        Ok(())
    }

    fn execute(&self, input: &str) -> Result<String> {
        Ok(format!("[Echo] {}", input))
    }

    fn version(&self) -> &str {
        &self.version
    }
}

// Another plugin implementation
struct ReversePlugin {
    name: String,
}

impl Module for ReversePlugin {
    fn name(&self) -> &str {
        &self.name
    }

    fn module_type(&self) -> &str {
        "plugin"
    }
}

impl Plugin for ReversePlugin {
    fn initialize(&self) -> Result<()> {
        println!("  Initializing {}", self.name);
        Ok(())
    }

    fn execute(&self, input: &str) -> Result<String> {
        Ok(input.chars().rev().collect())
    }

    fn version(&self) -> &str {
        "1.0.0"
    }
}

// Factory functions
fn create_echo_plugin() -> Result<Box<dyn std::any::Any + Send + Sync>> {
    Ok(Box::new(Box::new(EchoPlugin {
        name: "echo".to_string(),
        version: "1.0.0".to_string(),
    }) as Box<dyn Plugin>))
}

fn create_reverse_plugin() -> Result<Box<dyn std::any::Any + Send + Sync>> {
    Ok(Box::new(Box::new(ReversePlugin {
        name: "reverse".to_string(),
    }) as Box<dyn Plugin>))
}

fn main() -> Result<()> {
    println!("=== Module Registry: Plugin System Example ===\n");

    // Example 1: Create and populate registry
    println!("1. Creating Plugin Registry");
    println!("----------------------------");

    let registry = ModuleRegistry::new();
    println!("Registry created with {} plugins", registry.count());

    // Example 2: Register plugins
    println!("\n2. Registering Plugins");
    println!("----------------------");

    registry.register_with_metadata(
        "echo",
        "plugin",
        "create_echo_plugin",
        "example::plugins::echo",
        "EchoPlugin",
        create_echo_plugin,
    );

    registry.register_with_metadata(
        "reverse",
        "plugin",
        "create_reverse_plugin",
        "example::plugins::reverse",
        "ReversePlugin",
        create_reverse_plugin,
    );

    println!("Registered {} plugins", registry.count());

    // Example 3: List available plugins
    println!("\n3. Available Plugins");
    println!("--------------------");

    for plugin_name in registry.list_modules() {
        if let Some(metadata) = registry.get_metadata(&plugin_name) {
            println!("  - {} ({})", metadata.name, metadata.module_type);
            println!("    Struct: {}", metadata.struct_name);
            println!("    Path: {}", metadata.module_path);
        }
    }

    // Example 4: Initialize plugins
    println!("\n4. Initializing Plugins");
    println!("-----------------------");

    for plugin_name in registry.list_modules() {
        let any_module = registry.create_any(&plugin_name)?;
        let plugin = any_module
            .downcast::<Box<dyn Plugin>>()
            .map_err(|_| anyhow::anyhow!("Type mismatch"))?;

        plugin.initialize()?;
    }

    // Example 5: Execute plugins
    println!("\n5. Executing Plugins");
    println!("--------------------");

    let test_input = "Hello, World!";
    println!("Input: \"{}\"", test_input);

    for plugin_name in registry.list_modules() {
        let any_module = registry.create_any(&plugin_name)?;
        let plugin = any_module
            .downcast::<Box<dyn Plugin>>()
            .map_err(|_| anyhow::anyhow!("Type mismatch"))?;

        let output = plugin.execute(test_input)?;
        println!("  {} → \"{}\"", plugin.name(), output);
    }

    // Example 6: Check plugin existence
    println!("\n6. Plugin Existence Checks");
    println!("--------------------------");

    println!("Has 'echo' plugin? {}", registry.has_module("echo"));
    println!("Has 'reverse' plugin? {}", registry.has_module("reverse"));
    println!("Has 'missing' plugin? {}", registry.has_module("missing"));

    // Example 7: Global registry
    println!("\n7. Using Global Registry");
    println!("------------------------");

    let global = ModuleRegistry::global();
    global.register("global_echo", "plugin", create_echo_plugin);

    println!("Global registry has {} modules", global.count());

    let any_module = global.create_any("global_echo")?;
    let plugin = any_module
        .downcast::<Box<dyn Plugin>>()
        .map_err(|_| anyhow::anyhow!("Type mismatch"))?;

    let result = plugin.execute("Global test")?;
    println!("Global plugin output: {}", result);

    // Cleanup
    global.clear();

    println!("\n=== Example completed successfully ===");

    Ok(())
}

