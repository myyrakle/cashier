use cashier::{dynamo, Cashier};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let config = aws_config::load_from_env().await;
    // let client = aws_sdk_dynamodb::Client::new(&config);

    let mut cashier = dynamo::DynamoCashier::new()
        .table_name("cashier2")
        .config(aws_config::load_from_env().await);

    cashier.connect().unwrap();

    let foo = cashier.get("asda2sd").unwrap();
    println!("foo: {:?}", foo);

    cashier.delete("asdasd").unwrap();

    Ok(())
}
