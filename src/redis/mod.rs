pub use super::Cashier;

#[derive(Debug, Clone)]
pub struct RedisCashier {
    client: Option<redis::Client>,
}

impl RedisCashier {
    /// Create a new RedisCashier
    pub fn new() -> Self {
        RedisCashier { client: None }
    }

    /// Connect to a Redis server
    pub fn connect(&mut self, url: &str) -> anyhow::Result<()> {
        self.client = Some(redis::Client::open(url)?);
        Ok(())
    }
}

impl Cashier for RedisCashier {
    fn set(&self, key: &str, value: &str) -> anyhow::Result<()> {
        if let Some(client) = &self.client {
            let mut con = client.get_connection()?;
            redis::cmd("SET").arg(key).arg(value).query(&mut con)?;
            Ok(())
        } else {
            Err(anyhow::anyhow!("Redis client not connected"))
        }
    }

    fn set_with_ttl(&self, key: &str, value: &str, ttl: u64) -> anyhow::Result<()> {
        if let Some(client) = &self.client {
            let mut con = client.get_connection()?;
            redis::cmd("SET")
                .arg(key)
                .arg(value)
                .arg("EX")
                .arg(ttl)
                .query(&mut con)?;
            Ok(())
        } else {
            Err(anyhow::anyhow!("Redis client not connected"))
        }
    }

    fn get(&self, key: &str) -> anyhow::Result<Option<String>> {
        if let Some(client) = &self.client {
            let mut con = client.get_connection()?;
            let value: Option<String> = redis::cmd("GET").arg(key).query(&mut con)?;
            Ok(value)
        } else {
            Err(anyhow::anyhow!("Redis client not connected"))
        }
    }

    fn delete(&self, key: &str) -> anyhow::Result<()> {
        if let Some(client) = &self.client {
            let mut con = client.get_connection()?;
            redis::cmd("DEL").arg(key).query(&mut con)?;
            Ok(())
        } else {
            Err(anyhow::anyhow!("Redis client not connected"))
        }
    }

    fn clear(&self) -> anyhow::Result<()> {
        if let Some(client) = &self.client {
            let mut con = client.get_connection()?;
            redis::cmd("FLUSHDB").query(&mut con)?;
            Ok(())
        } else {
            Err(anyhow::anyhow!("Redis client not connected"))
        }
    }
}
