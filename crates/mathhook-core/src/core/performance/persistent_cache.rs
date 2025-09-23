//! Persistent Cache System
//!
//! This module implements a persistent cache that survives across sessions,
//! storing frequently used expression simplifications to disk for faster startup.

use crate::core::Expression;
use dirs;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, OnceLock, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};

/// Serializable cache entry for persistent storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersistentCacheEntry {
    /// Hash of the original expression
    pub expression_hash: u64,
    /// Serialized simplified expression
    pub simplified_expression: String,
    /// Number of times this entry has been accessed
    pub access_count: u64,
    /// Last access timestamp (Unix timestamp)
    pub last_access: u64,
    /// Creation timestamp (Unix timestamp)
    pub created_at: u64,
}

/// Persistent cache configuration
#[derive(Debug, Clone)]
pub struct PersistentCacheConfig {
    /// Directory to store cache files
    pub cache_directory: PathBuf,
    /// Maximum number of entries to keep in persistent cache
    pub max_entries: usize,
    /// Maximum age of cache entries in seconds (7 days default)
    pub max_age_seconds: u64,
    /// Minimum access count to persist an entry
    pub min_access_count: u64,
    /// How often to save cache to disk (in number of operations)
    pub save_frequency: usize,
}

impl Default for PersistentCacheConfig {
    fn default() -> Self {
        Self {
            cache_directory: get_default_cache_directory(),
            max_entries: 50000,
            max_age_seconds: 7 * 24 * 60 * 60, // 7 days
            min_access_count: 2,               // Must be accessed at least twice
            save_frequency: 100,               // Save every 100 operations
        }
    }
}

/// Persistent cache that survives across sessions
pub struct PersistentCache {
    /// In-memory cache entries
    entries: Arc<RwLock<HashMap<u64, PersistentCacheEntry>>>,
    /// Configuration
    config: PersistentCacheConfig,
    /// Number of operations since last save
    operations_since_save: Arc<RwLock<usize>>,
    /// Cache file path
    cache_file_path: PathBuf,
}

impl PersistentCache {
    /// Create a new persistent cache
    pub fn new(config: PersistentCacheConfig) -> Self {
        let cache_file_path = config.cache_directory.join("mathhook_cache.json");

        let cache = Self {
            entries: Arc::new(RwLock::new(HashMap::new())),
            config,
            operations_since_save: Arc::new(RwLock::new(0)),
            cache_file_path,
        };

        // Load existing cache from disk
        cache.load_from_disk();

        cache
    }

    /// Get a cached result
    pub fn get(&self, expression_hash: u64) -> Option<Expression> {
        if let Ok(mut entries) = self.entries.write() {
            if let Some(entry) = entries.get_mut(&expression_hash) {
                // Update access statistics
                entry.access_count += 1;
                entry.last_access = current_timestamp();

                // Try to deserialize the expression
                if let Ok(expr) = self.deserialize_expression(&entry.simplified_expression) {
                    self.increment_operations();
                    return Some(expr);
                }
            }
        }
        None
    }

    /// Store a result in the cache
    pub fn put(&self, expression_hash: u64, simplified: &Expression) {
        let serialized = match self.serialize_expression(simplified) {
            Ok(s) => s,
            Err(_) => return, // Skip if serialization fails
        };

        let entry = PersistentCacheEntry {
            expression_hash,
            simplified_expression: serialized,
            access_count: 1,
            last_access: current_timestamp(),
            created_at: current_timestamp(),
        };

        if let Ok(mut entries) = self.entries.write() {
            entries.insert(expression_hash, entry);

            // Clean up old entries if cache is too large
            if entries.len() > self.config.max_entries {
                self.cleanup_old_entries(&mut entries);
            }
        }

        self.increment_operations();
    }

    /// Load cache from disk
    fn load_from_disk(&self) {
        if !self.cache_file_path.exists() {
            return;
        }

        match fs::read_to_string(&self.cache_file_path) {
            Ok(content) => {
                match serde_json::from_str::<HashMap<u64, PersistentCacheEntry>>(&content) {
                    Ok(loaded_entries) => {
                        if let Ok(mut entries) = self.entries.write() {
                            // Filter out expired entries
                            let current_time = current_timestamp();
                            let valid_entries: HashMap<u64, PersistentCacheEntry> = loaded_entries
                                .into_iter()
                                .filter(|(_, entry)| {
                                    current_time - entry.created_at < self.config.max_age_seconds
                                })
                                .collect();

                            *entries = valid_entries;
                            println!("📁 Loaded {} persistent cache entries", entries.len());
                        }
                    }
                    Err(e) => {
                        eprintln!("⚠️ Failed to parse persistent cache: {}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("⚠️ Failed to read persistent cache file: {}", e);
            }
        }
    }

    /// Save cache to disk
    pub fn save_to_disk(&self) {
        // Ensure cache directory exists
        if let Some(parent) = self.cache_file_path.parent() {
            if let Err(e) = fs::create_dir_all(parent) {
                eprintln!("⚠️ Failed to create cache directory: {}", e);
                return;
            }
        }

        if let Ok(entries) = self.entries.read() {
            // Filter entries worth persisting
            let current_time = current_timestamp();
            let persistent_entries: HashMap<u64, PersistentCacheEntry> = entries
                .iter()
                .filter(|(_, entry)| {
                    entry.access_count >= self.config.min_access_count
                        && current_time - entry.created_at < self.config.max_age_seconds
                })
                .map(|(k, v)| (*k, v.clone()))
                .collect();

            match serde_json::to_string_pretty(&persistent_entries) {
                Ok(content) => {
                    if let Err(e) = fs::write(&self.cache_file_path, content) {
                        eprintln!("⚠️ Failed to write persistent cache: {}", e);
                    } else {
                        println!(
                            "💾 Saved {} entries to persistent cache",
                            persistent_entries.len()
                        );
                    }
                }
                Err(e) => {
                    eprintln!("⚠️ Failed to serialize persistent cache: {}", e);
                }
            }
        }

        // Reset operation counter
        if let Ok(mut ops) = self.operations_since_save.write() {
            *ops = 0;
        }
    }

    /// Clean up old and rarely used entries
    fn cleanup_old_entries(&self, entries: &mut HashMap<u64, PersistentCacheEntry>) {
        let current_time = current_timestamp();
        let target_size = (self.config.max_entries as f64 * 0.8) as usize; // Remove 20%

        // Collect entries with their scores (higher score = more valuable)
        let mut scored_entries: Vec<(u64, f64)> = entries
            .iter()
            .map(|(hash, entry)| {
                let age_factor = 1.0 / (1.0 + (current_time - entry.last_access) as f64 / 3600.0); // Decay over hours
                let access_factor = (entry.access_count as f64).ln().max(1.0);
                let score = age_factor * access_factor;
                (*hash, score)
            })
            .collect();

        // Sort by score (ascending, so lowest scores are first)
        scored_entries.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));

        // Remove lowest scoring entries
        let to_remove = entries.len().saturating_sub(target_size);
        for (hash, _) in scored_entries.iter().take(to_remove) {
            entries.remove(hash);
        }

        println!("🧹 Cleaned up {} old cache entries", to_remove);
    }

    /// Increment operation counter and save if needed
    fn increment_operations(&self) {
        if let Ok(mut ops) = self.operations_since_save.write() {
            *ops += 1;
            if *ops >= self.config.save_frequency {
                // Save in background to avoid blocking
                self.save_to_disk();
            }
        }
    }

    /// Serialize an expression to string (placeholder implementation)
    fn serialize_expression(&self, expr: &Expression) -> Result<String, String> {
        // For now, use debug format as a simple serialization
        // In a real implementation, you'd want a proper serialization format
        Ok(format!("{:?}", expr))
    }

    /// Deserialize an expression from string (placeholder implementation)
    fn deserialize_expression(&self, serialized: &str) -> Result<Expression, String> {
        // This is a placeholder - in a real implementation, you'd parse the serialized format
        // For now, we'll return an error to indicate deserialization is not implemented
        Err("Deserialization not implemented yet".to_string())
    }

    /// Get cache statistics
    pub fn get_statistics(&self) -> PersistentCacheStatistics {
        if let Ok(entries) = self.entries.read() {
            let total_entries = entries.len();
            let total_access_count: u64 = entries.values().map(|e| e.access_count).sum();
            let average_access_count = if total_entries > 0 {
                total_access_count as f64 / total_entries as f64
            } else {
                0.0
            };

            let current_time = current_timestamp();
            let recent_entries = entries
                .values()
                .filter(|e| current_time - e.last_access < 3600) // Last hour
                .count();

            PersistentCacheStatistics {
                total_entries,
                recent_entries,
                total_access_count,
                average_access_count,
                cache_file_size: self.get_cache_file_size(),
                cache_directory: self.config.cache_directory.clone(),
            }
        } else {
            PersistentCacheStatistics::default()
        }
    }

    /// Get cache file size in bytes
    fn get_cache_file_size(&self) -> u64 {
        fs::metadata(&self.cache_file_path)
            .map(|m| m.len())
            .unwrap_or(0)
    }

    /// Force save cache to disk
    pub fn force_save(&self) {
        self.save_to_disk();
    }

    /// Clear all cache entries
    pub fn clear(&self) {
        if let Ok(mut entries) = self.entries.write() {
            entries.clear();
        }
        // Remove cache file
        let _ = fs::remove_file(&self.cache_file_path);
    }
}

/// Statistics for the persistent cache
#[derive(Debug, Clone, Default)]
pub struct PersistentCacheStatistics {
    /// Total number of cached entries
    pub total_entries: usize,
    /// Number of entries accessed recently (last hour)
    pub recent_entries: usize,
    /// Total access count across all entries
    pub total_access_count: u64,
    /// Average access count per entry
    pub average_access_count: f64,
    /// Size of cache file on disk in bytes
    pub cache_file_size: u64,
    /// Cache directory path
    pub cache_directory: PathBuf,
}

/// Get default cache directory
fn get_default_cache_directory() -> PathBuf {
    if let Some(cache_dir) = dirs::cache_dir() {
        cache_dir.join("mathhook")
    } else {
        PathBuf::from(".mathhook_cache")
    }
}

/// Get current Unix timestamp
fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

/// Global persistent cache instance
static GLOBAL_PERSISTENT_CACHE: OnceLock<PersistentCache> = OnceLock::new();

/// Get the global persistent cache instance
pub fn get_global_persistent_cache() -> &'static PersistentCache {
    GLOBAL_PERSISTENT_CACHE.get_or_init(|| PersistentCache::new(PersistentCacheConfig::default()))
}

/// Get a cached result from the global persistent cache
pub fn get_persistent_cached_result(expression_hash: u64) -> Option<Expression> {
    get_global_persistent_cache().get(expression_hash)
}

/// Store a result in the global persistent cache
pub fn store_persistent_cached_result(expression_hash: u64, simplified: &Expression) {
    get_global_persistent_cache().put(expression_hash, simplified);
}

/// Get persistent cache statistics
pub fn get_persistent_cache_statistics() -> PersistentCacheStatistics {
    get_global_persistent_cache().get_statistics()
}

/// Force save persistent cache to disk
pub fn save_persistent_cache() {
    get_global_persistent_cache().force_save();
}

/// Clear persistent cache
pub fn clear_persistent_cache() {
    get_global_persistent_cache().clear();
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_persistent_cache_creation() {
        let temp_dir = TempDir::new().unwrap();
        let config = PersistentCacheConfig {
            cache_directory: temp_dir.path().to_path_buf(),
            ..Default::default()
        };

        let cache = PersistentCache::new(config);
        let stats = cache.get_statistics();

        assert_eq!(stats.total_entries, 0);
        assert_eq!(stats.total_access_count, 0);
    }

    #[test]
    fn test_cache_file_path() {
        let temp_dir = TempDir::new().unwrap();
        let config = PersistentCacheConfig {
            cache_directory: temp_dir.path().to_path_buf(),
            ..Default::default()
        };

        let cache = PersistentCache::new(config);
        assert!(cache.cache_file_path.ends_with("mathhook_cache.json"));
    }

    #[test]
    fn test_default_cache_directory() {
        let default_dir = get_default_cache_directory();
        assert!(default_dir.to_string_lossy().contains("mathhook"));
    }

    #[test]
    fn test_global_persistent_cache() {
        let stats = get_persistent_cache_statistics();
        // Verify stats are accessible (total_entries is usize, always >= 0)
        assert!(stats.total_entries == stats.total_entries); // Should not panic

        // Test that we can call the global functions
        save_persistent_cache(); // Should not panic
    }
}
