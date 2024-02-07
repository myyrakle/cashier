/// cache trait
pub trait Cashier {
    /// Set the value of a key
    fn set(&self, key: &str, value: &str) -> anyhow::Result<()>;

    /// Set the value of a key with a time-to-live (TTL) in seconds
    fn set_with_ttl(&self, key: &str, value: &str, ttl: u64) -> anyhow::Result<()>;

    /// Get the value of a key
    fn get(&self, key: &str) -> anyhow::Result<Option<String>>;

    /// Delete a key
    fn delete(&self, key: &str) -> anyhow::Result<()>;

    /// clear all keys
    fn clear(&self) -> anyhow::Result<()>;
}

pub mod dynamo;
pub mod memory;
pub mod postgres;
pub mod redis;
