use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashMap;
use std::sync::Arc;

/// Thread-safe cache for compiled regex patterns
pub struct RegexCache {
    cache: HashMap<String, Arc<Regex>>,
}

impl RegexCache {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    /// Get or compile a regex pattern
    pub fn get_or_compile(&mut self, pattern: &str) -> anyhow::Result<Arc<Regex>> {
        if let Some(regex) = self.cache.get(pattern) {
            return Ok(Arc::clone(regex));
        }

        let regex = Regex::new(pattern)
            .map_err(|e| anyhow::anyhow!("Failed to compile regex '{}': {}", pattern, e))?;
        let regex = Arc::new(regex);
        self.cache.insert(pattern.to_string(), Arc::clone(&regex));
        Ok(regex)
    }
}

/// Global regex cache instance
static REGEX_CACHE: Lazy<std::sync::Mutex<RegexCache>> = 
    Lazy::new(|| std::sync::Mutex::new(RegexCache::new()));

/// Get a compiled regex from the global cache
pub fn get_regex(pattern: &str) -> anyhow::Result<Arc<Regex>> {
    REGEX_CACHE
        .lock()
        .map_err(|e| anyhow::anyhow!("Failed to acquire regex cache lock: {}", e))?
        .get_or_compile(pattern)
}

/// Pre-compiled common regex patterns
pub mod patterns {
    use super::*;

    /// Regex for matching TypeScript/JavaScript imports
    pub static IMPORT_REGEX: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r#"import\s+(?:[\w\s{},*]+\s+from\s+)?['"]([^'"]+)['"]"#).unwrap()
    });

    /// Regex for matching file extensions
    pub static FILE_EXT_REGEX: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"\.(ts|tsx|js|jsx|vue)$").unwrap()
    });

    /// Regex for matching component names
    pub static COMPONENT_NAME_REGEX: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"^[A-Z][a-zA-Z0-9_]*$").unwrap()
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regex_cache() {
        let pattern = r"\d+";
        let regex1 = get_regex(pattern).unwrap();
        let regex2 = get_regex(pattern).unwrap();
        
        // Should be the same Arc instance
        assert!(Arc::ptr_eq(&regex1, &regex2));
    }

    #[test]
    fn test_import_regex() {
        let code = r#"import React from "react""#;
        assert!(patterns::IMPORT_REGEX.is_match(code));
    }
}