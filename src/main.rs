use aws_sdk_dynamodb::types::AttributeValue;
use epoch_timestamp::Epoch;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = aws_config::load_from_env().await;
    let client = aws_sdk_dynamodb::Client::new(&config);

    // 10초 뒤
    let expired_at = Epoch::now() + Epoch::second(10);

    client
        .put_item()
        .table_name("cache")
        .item("key", AttributeValue::S("testkey".into()))
        .item("value", AttributeValue::S("foo".into()))
        .item("expiredAt", AttributeValue::N(expired_at.to_string()))
        .send()
        .await
        .unwrap();

    Ok(())
}
