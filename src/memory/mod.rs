use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

pub use super::Cashier;
use epoch_timestamp::Epoch;

#[derive(Debug, Clone)]
struct Value {
    data: String,
    expired_at: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct MemoryCashier {
    cache: Arc<RwLock<HashMap<String, Value>>>,
}

impl MemoryCashier {
    pub fn new() -> Self {
        MemoryCashier {
            cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl Cashier for MemoryCashier {
    fn set(&self, key: &str, value: &str) -> anyhow::Result<()> {
        let mut cache = if let Ok(write_guard) = self.cache.write() {
            write_guard
        } else {
            return Err(anyhow::anyhow!("Failed to acquire write lock on cache"));
        };

        cache.insert(
            key.to_string(),
            Value {
                data: value.to_string(),
                expired_at: None,
            },
        );
        Ok(())
    }

    fn set_with_ttl(&self, key: &str, value: &str, ttl: u64) -> anyhow::Result<()> {
        let mut cache = if let Ok(write_guard) = self.cache.write() {
            write_guard
        } else {
            return Err(anyhow::anyhow!("Failed to acquire write lock on cache"));
        };

        cache.insert(
            key.to_string(),
            Value {
                data: value.to_string(),
                expired_at: Some(Epoch::now() + Epoch::second(ttl)),
            },
        );
        Ok(())
    }

    fn get(&self, key: &str) -> anyhow::Result<Option<String>> {
        if let Ok(cache) = self.cache.read() {
            match cache.get(key) {
                Some(value) => {
                    if let Some(expired_at) = value.expired_at {
                        if expired_at <= Epoch::now() {
                            return Ok(None);
                        }
                    }
                    Ok(Some(value.data.clone()))
                }
                None => Ok(None),
            }
        } else {
            Err(anyhow::anyhow!("Failed to acquire read lock on cache"))
        }
    }

    fn delete(&self, key: &str) -> anyhow::Result<()> {
        let mut cache = self.cache.write().unwrap();
        cache.remove(key);
        Ok(())
    }

    fn clear(&self) -> anyhow::Result<()> {
        let mut cache = self.cache.write().unwrap();
        cache.clear();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set() {
        let cashier = MemoryCashier::new();
        cashier.set("key", "value").unwrap();
        assert_eq!(cashier.get("key").unwrap().unwrap(), "value");
    }

    #[test]
    fn test_set_with_ttl() {
        let cashier = MemoryCashier::new();
        cashier.set_with_ttl("key", "value", 1).unwrap();
        assert_eq!(cashier.get("key").unwrap().unwrap(), "value");

        std::thread::sleep(std::time::Duration::from_secs(2));
        assert_eq!(cashier.get("key").unwrap(), None);
    }

    #[test]
    fn test_get() {
        let cashier = MemoryCashier::new();
        cashier.set("key", "value").unwrap();
        assert_eq!(cashier.get("key").unwrap().unwrap(), "value");
    }

    #[test]
    fn test_delete() {
        let cashier = MemoryCashier::new();
        cashier.set("key", "value").unwrap();
        cashier.delete("key").unwrap();
        assert_eq!(cashier.get("key").unwrap(), None);
    }

    #[test]
    fn test_clear() {
        let cashier = MemoryCashier::new();
        cashier.set("key", "value").unwrap();
        cashier.clear().unwrap();
        assert_eq!(cashier.get("key").unwrap(), None);
    }
}
