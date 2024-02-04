#[derive(Debug, Clone)]
pub struct RedisCashier {
    client: Option<redis::Client>,
}

impl RedisCashier {
    pub fn new() -> Self {
        RedisCashier { client: None }
    }

    pub fn connect(&mut self, url: &str) -> anyhow::Result<()> {
        self.client = Some(redis::Client::open(url)?);
        Ok(())
    }
}
