#[derive(Debug, Clone)]
pub struct RedisCashier {
    client: redis::Client,
}
