use aws_sdk_dynamodb::types::AttributeValue;
use cashier::{dynamo, Cashier};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let config = aws_config::load_from_env().await;
    // let client = aws_sdk_dynamodb::Client::new(&config);

    let mut cashier = dynamo::DynamoCashier::new()
        .table_name("cashier2")
        .config(aws_config::load_from_env().await);

    cashier.connect().unwrap();

    cashier.set("key12", "value133").unwrap();
    cashier.set_with_ttl("asdasd", "@!3123123", 100).unwrap();

    Ok(())
}
