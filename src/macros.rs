//! Macros for module registry

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

/// Macro to get the current module path
#[macro_export]
macro_rules! module_path {
    () => {
        concat!(file!(), ":", line!())
    };
}
