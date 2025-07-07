use serde::{Deserialize, Serialize};
use std::path::Path;

pub mod regex_cache;
pub mod path_utils;
pub mod cache;

pub use regex_cache::*;
pub use path_utils::*;
pub use cache::*;

/// Common DTS generation configuration used across multiple plugins
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", untagged)]
pub enum Dts {
    /// Generate DTS file with specified path
    FilePath(String),
    /// Enable/disable DTS generation
    Bool(bool),
    /// Detailed DTS configuration
    Config {
        /// Enable DTS generation
        enabled: bool,
        /// Output file path
        file_path: Option<String>,
        /// Additional configuration options
        #[serde(flatten)]
        extra: serde_json::Value,
    },
}

impl Dts {
    /// Check if DTS generation is enabled
    #[inline]
    pub fn is_enabled(&self) -> bool {
        match self {
            Dts::Bool(enabled) => *enabled,
            Dts::FilePath(_) => true,
            Dts::Config { enabled, .. } => *enabled,
        }
    }

    /// Get the DTS file path if specified
    #[inline]
    pub fn file_path(&self) -> Option<&str> {
        match self {
            Dts::FilePath(path) => Some(path),
            Dts::Config { file_path, .. } => file_path.as_deref(),
            _ => None,
        }
    }
}

impl Default for Dts {
    fn default() -> Self {
        Dts::Bool(true)
    }
}

/// Convert string to PascalCase
#[inline]
pub fn to_pascal_case(s: &str) -> String {
    s.split('_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase(),
            }
        })
        .collect()
}

/// Get filename from path without extension
#[inline]
pub fn get_filename_by_path(path: &str) -> String {
    Path::new(path)
        .file_stem()
        .and_then(|stem| stem.to_str())
        .unwrap_or("")
        .to_string()
}

/// Safe JSON parsing with error handling
#[cold]
pub fn parse_options<T: for<'de> Deserialize<'de>>(options: &str) -> anyhow::Result<T> {
    serde_json::from_str(options)
        .map_err(|e| anyhow::anyhow!("Failed to parse plugin options: {}", e))
}

/// Initialize plugin with proper error handling
#[cold]
pub fn init_plugin<T: for<'de> Deserialize<'de>>(options: &str) -> anyhow::Result<T> {
    if options.is_empty() {
        return Err(anyhow::anyhow!("Plugin options cannot be empty"));
    }
    parse_options(options)
}

/// Performance monitoring utilities
pub mod perf {
    use std::time::Instant;

    /// Simple performance timer
    pub struct Timer {
        start: Instant,
        name: String,
    }

    impl Timer {
        #[inline]
        pub fn new(name: &str) -> Self {
            Self {
                start: Instant::now(),
                name: name.to_string(),
            }
        }

        #[inline]
        pub fn elapsed_ms(&self) -> f64 {
            self.start.elapsed().as_secs_f64() * 1000.0
        }
    }

    impl Drop for Timer {
        #[cold]
        fn drop(&mut self) {
            if cfg!(debug_assertions) {
                println!("[PERF] {}: {:.2}ms", self.name, self.elapsed_ms());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_pascal_case() {
        assert_eq!(to_pascal_case("hello_world"), "HelloWorld");
        assert_eq!(to_pascal_case("test_case"), "TestCase");
        assert_eq!(to_pascal_case("simple"), "Simple");
    }

    #[test]
    fn test_get_filename_by_path() {
        assert_eq!(get_filename_by_path("path/to/file.rs"), "file");
        assert_eq!(get_filename_by_path("simple.txt"), "simple");
        assert_eq!(get_filename_by_path("no_extension"), "no_extension");
    }

    #[test]
    fn test_dts_config() {
        let dts = Dts::Bool(true);
        assert!(dts.is_enabled());
        assert_eq!(dts.file_path(), None);

        let dts = Dts::FilePath("types.d.ts".to_string());
        assert!(dts.is_enabled());
        assert_eq!(dts.file_path(), Some("types.d.ts"));
    }
}