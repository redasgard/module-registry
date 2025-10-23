//! Constants for module registry

// Default permission values
pub const DEFAULT_MEMORY_LIMIT_MB: u64 = 128;
pub const DEFAULT_CPU_LIMIT_PERCENT: u8 = 50;
pub const DEFAULT_TIMEOUT_SECONDS: u64 = 30;

// Security constants
pub const SIGNATURE_EXPIRY_SECONDS: u64 = 365 * 24 * 60 * 60; // 1 year
pub const DEFAULT_SIGNATURE_ALGORITHM: &str = "SHA256-RSA";

// Sandbox defaults
pub const DEFAULT_DENIED_PATHS: &[&str] = &["/etc", "/usr/bin", "/bin"];

// Registry limits
pub const MAX_MODULE_NAME_LENGTH: usize = 256;
pub const MAX_MODULE_TYPE_LENGTH: usize = 128;
pub const MAX_PATH_LENGTH: usize = 4096;
