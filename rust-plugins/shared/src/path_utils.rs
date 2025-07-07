use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Efficient path filtering utility
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathFilter {
    include: HashSet<String>,
    exclude: HashSet<String>,
}

impl PathFilter {
    /// Create a new path filter
    pub fn new(include: &[String], exclude: &[String]) -> Self {
        Self {
            include: include.iter().cloned().collect(),
            exclude: exclude.iter().cloned().collect(),
        }
    }

    /// Check if a path should be included
    #[inline]
    pub fn should_include(&self, path: &str) -> bool {
        // Early return for exclude patterns
        if self.exclude.iter().any(|pattern| path.contains(pattern)) {
            return false;
        }

        // If no include patterns, include everything not excluded
        if self.include.is_empty() {
            return true;
        }

        // Check include patterns
        self.include.iter().any(|pattern| path.contains(pattern))
    }

    /// Execute filtering on a path
    #[inline]
    pub fn execute(&self, path: &str) -> bool {
        self.should_include(path)
    }
}

/// File extension utilities
pub mod extensions {
    use once_cell::sync::Lazy;
    use std::collections::HashSet;

    /// Common TypeScript/JavaScript file extensions
    pub static TS_JS_EXTENSIONS: Lazy<HashSet<&'static str>> = Lazy::new(|| {
        ["ts", "tsx", "js", "jsx", "vue", "svelte"].into_iter().collect()
    });

    /// Common image file extensions
    pub static IMAGE_EXTENSIONS: Lazy<HashSet<&'static str>> = Lazy::new(|| {
        ["png", "jpg", "jpeg", "gif", "svg", "webp", "avif"].into_iter().collect()
    });

    /// Check if file has TypeScript/JavaScript extension
    #[inline]
    pub fn is_ts_js_file(path: &str) -> bool {
        std::path::Path::new(path)
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| TS_JS_EXTENSIONS.contains(ext))
            .unwrap_or(false)
    }

    /// Check if file has image extension
    #[inline]
    pub fn is_image_file(path: &str) -> bool {
        std::path::Path::new(path)
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| IMAGE_EXTENSIONS.contains(ext))
            .unwrap_or(false)
    }
}

/// Path normalization utilities
pub mod normalization {
    use std::path::{Path, PathBuf};

    /// Normalize a path and convert to string
    #[inline]
    pub fn normalize_path(path: &str) -> String {
        Path::new(path)
            .components()
            .collect::<PathBuf>()
            .to_string_lossy()
            .into_owned()
    }

    /// Get relative path from base
    #[inline]
    pub fn get_relative_path(base: &str, target: &str) -> Option<String> {
        let base_path = Path::new(base);
        let target_path = Path::new(target);
        
        target_path
            .strip_prefix(base_path)
            .ok()
            .map(|p| p.to_string_lossy().into_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_filter() {
        let filter = PathFilter::new(
            &["src".to_string()],
            &["node_modules".to_string()],
        );

        assert!(filter.should_include("src/main.rs"));
        assert!(!filter.should_include("node_modules/lib.js"));
        assert!(!filter.should_include("other/file.rs"));
    }

    #[test]
    fn test_file_extensions() {
        assert!(extensions::is_ts_js_file("test.ts"));
        assert!(extensions::is_ts_js_file("component.tsx"));
        assert!(!extensions::is_ts_js_file("image.png"));
        
        assert!(extensions::is_image_file("logo.png"));
        assert!(!extensions::is_image_file("script.js"));
    }
}