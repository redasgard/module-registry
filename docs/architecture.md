# Architecture

## System Overview

Module Registry implements a **type-safe plugin architecture** with compile-time discovery and runtime instantiation using Rust's type system and the `inventory` crate.

```
┌─────────────────────────────────────────────────────────────┐
│                  Compile Time                                │
├─────────────────────────────────────────────────────────────┤
│                                                               │
│  Module A                  Module B                Module N  │
│  ┌──────────┐             ┌──────────┐           ┌──────────┐│
│  │impl Trait│             │impl Trait│           │impl Trait││
│  └────┬─────┘             └────┬─────┘           └────┬─────┘│
│       │                        │                       │      │
│       │ register_module!       │ register_module!      │      │
│       └────────┬───────────────┴────────────┬──────────┘      │
│                │                            │                 │
│                ▼                            ▼                 │
│         ┌──────────────────────────────────────┐             │
│         │       inventory::collect!            │             │
│         │  (Compile-Time Registration)         │             │
│         └──────────────────────────────────────┘             │
│                                                               │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│                    Runtime                                   │
├─────────────────────────────────────────────────────────────┤
│                                                               │
│  ┌──────────────────────────────────────────────────────┐   │
│  │            ModuleRegistry                             │   │
│  │         (Thread-Safe Storage)                         │   │
│  ├──────────────────────────────────────────────────────┤   │
│  │  RwLock<HashMap<String, (Metadata, Factory)>>        │   │
│  │                                                       │   │
│  │  ┌──────────────┐  ┌──────────────┐  ┌──────────┐   │   │
│  │  │Module A      │  │Module B      │  │Module N  │   │   │
│  │  ├──────────────┤  ├──────────────┤  ├──────────┤   │   │
│  │  │Metadata      │  │Metadata      │  │Metadata  │   │   │
│  │  │Factory Fn    │  │Factory Fn    │  │Factory Fn│   │   │
│  │  └──────────────┘  └──────────────┘  └──────────┘   │   │
│  └──────────────────────────────────────────────────────┘   │
│                           │                                  │
│                           │ create("module_name")            │
│                           ▼                                  │
│  ┌──────────────────────────────────────────────────────┐   │
│  │  Factory Function Execution                          │   │
│  │  Returns: Box<dyn Any + Send + Sync>                 │   │
│  └──────────────────────────────────────────────────────┘   │
│                           │                                  │
│                           │ downcast                         │
│                           ▼                                  │
│                  Box<dyn YourTrait>                          │
│                                                               │
└─────────────────────────────────────────────────────────────┘
```

## Core Components

### 1. ModuleRegistry

Thread-safe storage and lookup for registered modules.

**Structure:**
```rust
pub struct ModuleRegistry {
    modules: RwLock<HashMap<String, (ModuleMetadata, ModuleFactory)>>,
}
```

**Key Methods:**
```rust
// Global singleton
pub fn global() -> &'static Self

// Registration
pub fn register(&self, name: &str, module_type: &str, factory: ModuleFactory)

// Instantiation
pub fn create_any(&self, name: &str) -> Result<Box<dyn Any + Send + Sync>>
pub fn create<T>(&self, name: &str) -> Result<Box<T>>

// Introspection
pub fn list_modules(&self) -> Vec<String>
pub fn has_module(&self, name: &str) -> bool
pub fn get_metadata(&self, name: &str) -> Option<ModuleMetadata>
```

**Location:** `src/lib.rs`

### 2. Module Trait

Base trait that all modules must implement.

**Definition:**
```rust
pub trait Module: Send + Sync {
    fn name(&self) -> &str;
    fn module_type(&self) -> &str;
}
```

**Purpose:**
- Provide common interface for all modules
- Enable trait object usage
- Ensure thread safety (Send + Sync)

### 3. Factory Pattern

Modules are instantiated through factory functions.

**Factory Type:**
```rust
pub type ModuleFactory = fn() -> Result<Box<dyn Any + Send + Sync>>;
```

**Why `Box<dyn Any>`?**
- Allows factory to return any trait object type
- User downcasts to their specific trait type
- Type-safe with proper error handling

**Example Factory:**
```rust
fn create_my_module() -> Result<Box<dyn Any + Send + Sync>> {
    // Create the concrete module
    let module = MyModule::new()?;
    
    // Box as trait object
    let trait_object = Box::new(module) as Box<dyn MyTrait>;
    
    // Box as Any for storage
    Ok(Box::new(trait_object))
}
```

### 4. ModuleMetadata

Stores information about registered modules.

**Structure:**
```rust
pub struct ModuleMetadata {
    pub name: String,                    // Module identifier
    pub module_type: String,             // Category (processor, provider, etc.)
    pub instantiate_fn_name: String,     // Factory function name
    pub module_path: String,             // Source file path
    pub struct_name: String,             // Struct name
}
```

**Use Cases:**
- Documentation generation
- Debugging and diagnostics
- Runtime introspection
- Dependency tracking

### 5. Inventory Integration

Automatic compile-time registration using `inventory` crate.

**ModuleRegistration Structure:**
```rust
pub struct ModuleRegistration {
    pub name: &'static str,
    pub module_type: &'static str,
    pub instantiate_fn_name: &'static str,
    pub module_path: &'static str,
    pub struct_name: &'static str,
    pub factory: ModuleFactory,
}

inventory::collect!(ModuleRegistration);
```

**How It Works:**
1. `inventory::submit!` at compile time registers modules
2. `inventory::iter` at runtime collects all registrations
3. Global registry is initialized on first access

### 6. register_module! Macro

Convenience macro for module registration.

**Definition:**
```rust
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
```

**Usage:**
```rust
register_module!("my_module", "MyModule", create_my_module);
```

## Registration Flow

### Compile-Time Registration

```
Module Source File
    │
    ├─ Define struct: `struct MyModule;`
    ├─ Implement Module trait
    ├─ Implement custom trait
    ├─ Define factory function: `fn create_my_module() -> Result<...>`
    └─ Call macro: `register_module!("my_module", "MyModule", create_my_module);`
         │
         ▼
    inventory::submit! → Registers in static storage
                              │
                              ▼
                         (Compile-time only)
```

### Runtime Initialization

```
First call to ModuleRegistry::global()
    │
    ├─ OnceLock ensures single initialization
    │
    └─> Create new ModuleRegistry
         │
         ├─> Iterate inventory::iter::<ModuleRegistration>
         │    │
         │    └─> For each registration:
         │         ├─ Create ModuleMetadata from registration
         │         └─ Insert into HashMap: (name → (metadata, factory))
         │
         └─> Return initialized registry

(Subsequent calls return cached instance)
```

## Instantiation Flow

### Creating a Module Instance

```
User calls: registry.create_any("my_module")
    │
    ├─ 1. Acquire read lock on HashMap
    │
    ├─ 2. Look up (metadata, factory) by name
    │    └─> Error if not found
    │
    ├─ 3. Call factory function
    │    factory() → Result<Box<dyn Any + Send + Sync>>
    │    └─> May fail with custom error
    │
    └─ 4. Return Box<dyn Any + Send + Sync>

User downcasts to specific type:
    │
    └─> any_module.downcast::<Box<dyn MyTrait>>()?
         │
         ├─ Success: Box<Box<dyn MyTrait>>
         │           └─> Dereference: Box<dyn MyTrait>
         │
         └─ Failure: Type mismatch error
```

## Type System Design

### Why Double Boxing?

```rust
// Module implementation
struct MyModule;

// Step 1: Box as specific trait object
let trait_box: Box<dyn MyTrait> = Box::new(MyModule);

// Step 2: Box as Any for registry storage
let any_box: Box<dyn Any + Send + Sync> = Box::new(trait_box);

// Step 3: Store in registry
registry.insert(name, any_box);

// Step 4: Retrieve and downcast
let any_box = registry.get(name)?;
let trait_box: Box<Box<dyn MyTrait>> = any_box.downcast()?;
let module: Box<dyn MyTrait> = *trait_box;  // Dereference
```

**Rationale:**
- Registry must store different trait types → needs `Any`
- Traits must be `Send + Sync` → registry is thread-safe
- Downcasting validates type at runtime → type safety

### Alternative: Direct Trait Object Storage

**Not possible without specialization:**
```rust
// Cannot do this generically without specialization
fn register<T: MyTrait>(name: &str, instance: T) {
    registry.insert(name, Box::new(instance) as Box<dyn Any>);
    // Problem: How to downcast back to Box<dyn MyTrait>?
}
```

**Solution:** Factory pattern with explicit casting
```rust
fn create_module() -> Result<Box<dyn Any + Send + Sync>> {
    let module = MyModule;
    let trait_obj = Box::new(module) as Box<dyn MyTrait>;
    Ok(Box::new(trait_obj))
}
```

## Thread Safety

### RwLock vs Mutex

**Choice: RwLock**
```rust
modules: RwLock<HashMap<...>>
```

**Rationale:**
- **Many readers, few writers**
- Registration: write-once at initialization
- Queries: read-many during runtime
- Module creation: read-only (no lock contention)

**Performance:**
- Concurrent reads: ✅ No blocking
- Concurrent writes: ❌ Serialized (rare)

### Send + Sync Requirements

**Why required?**
```rust
Box<dyn Any + Send + Sync>
```

- **Send**: Module can be transferred between threads
- **Sync**: Module can be accessed from multiple threads

**Implications:**
- All modules must be thread-safe
- Interior mutability requires `Arc<Mutex<T>>` or similar
- Prevents data races at compile time

## Performance Characteristics

### Registration
- **Time**: O(1) per module
- **Memory**: ~100 bytes per module (metadata + pointer)
- **When**: Compile-time submission, runtime initialization

### Lookup
- **Time**: O(1) average (HashMap)
- **Memory**: No allocation (read lock)
- **Concurrency**: Unlimited parallel reads

### Instantiation
- **Time**: O(factory) - depends on module
- **Memory**: O(module size)
- **Concurrency**: Parallel instantiation possible

### Global Registry
- **Lazy initialization**: First access only
- **Thread-safe**: OnceLock guarantees single init
- **No runtime overhead**: After initialization

## Use Case Patterns

### 1. Plugin System

```
Application
    │
    ├─ Define plugin trait
    ├─ Load plugins from registry
    └─ Execute plugins dynamically

Plugins (compiled with app)
    ├─ Plugin A: register_module!(...)
    ├─ Plugin B: register_module!(...)
    └─ Plugin N: register_module!(...)
```

### 2. Service Locator

```
Services
    ├─ DatabaseService: register_module!("database", ...)
    ├─ CacheService: register_module!("cache", ...)
    └─ LoggingService: register_module!("logging", ...)

Application
    └─> Locate services by name at runtime
```

### 3. Provider Pattern

```
Providers (same interface, different implementations)
    ├─ PostgresProvider: register_module!("postgres", ...)
    ├─ MySQLProvider: register_module!("mysql", ...)
    └─ SQLiteProvider: register_module!("sqlite", ...)

Application
    └─> Select provider from config: registry.create(config.db_provider)
```

### 4. Feature Flags

```
Features (conditionally compiled)
    ├─ #[cfg(feature = "premium")] register_module!("premium_feature", ...)
    ├─ register_module!("free_feature", ...)
    └─ #[cfg(feature = "experimental")] register_module!("experimental", ...)

Application
    └─> Query available features: registry.list_modules()
```

## Limitations

### 1. Static Linking Only

**Limitation:**
- Modules must be compiled into the binary
- No runtime loading of external `.so`/`.dll`

**Workaround:**
- Use Rust's dynamic library support separately
- Consider `libloading` crate for dynamic loading

### 2. Type Erasure

**Limitation:**
- Registry stores `Box<dyn Any>`
- User must know the correct trait type to downcast

**Workaround:**
- Store type information in metadata
- Provide typed wrappers

### 3. No Dependency Resolution

**Limitation:**
- No automatic dependency injection
- No lifecycle management

**Workaround:**
- Implement in factory functions
- Use external DI framework

## Error Handling

### Registration Errors

- **Duplicate names**: Overrides previous registration
- **Invalid metadata**: Caught at compile time
- **Factory panics**: Propagates to caller

### Instantiation Errors

```rust
pub enum RegistryError {
    ModuleNotFound(String),
    FactoryFailed(String),
    TypeMismatch(String),
}
```

**Error Flow:**
```
registry.create("module")?
    │
    ├─ Not found → Error::ModuleNotFound
    ├─ Factory fails → Error::FactoryFailed (with context)
    └─ Downcast fails → Error::TypeMismatch
```

## Testing Strategy

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    // Test registry operations
    - test_registry_creation()
    - test_module_registration()
    - test_module_creation()
    - test_list_modules()
    - test_has_module()
    - test_get_metadata()
    - test_module_not_found()
    - test_clear_registry()
}
```

### Integration Tests

```rust
// tests/integration_test.rs
- Test multiple modules
- Test global registry
- Test inventory integration
- Test concurrent access
```

## Future Enhancements

### v0.2
- Dependency resolution
- Lifecycle hooks (init, shutdown)
- Module versioning

### v0.3
- Dynamic library loading
- Hot module replacement
- Module isolation (sandboxing)

### v0.4
- Typed registry (avoid downcasting)
- Async module initialization
- Module health checks

