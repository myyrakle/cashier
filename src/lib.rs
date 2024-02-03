/// cache trait
pub trait Cashier {
    /// Set the value of a key
    fn set(key: &str, value: &str) -> anyhow::Result<()>;

    /// Set the value of a key with a time-to-live (TTL) in seconds
    fn set_with_ttl(key: &str, value: &str, ttl: u64) -> anyhow::Result<()>;

    /// Get the value of a key
    fn get(key: &str) -> anyhow::Result<&str>;

    /// Delete a key
    fn delete(key: &str) -> anyhow::Result<()>;

    /// clear all keys
    fn clear() -> anyhow::Result<()>;
}
