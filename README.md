# cashier

![](https://img.shields.io/badge/language-Rust-red) ![](https://img.shields.io/badge/version-0.1.1-brightgreen) [![GitHub license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/myyrakle/rupring/blob/master/LICENSE)

This is a module for Key Value caching. Provides cache operation through the same interface for various data sources.
Features currently provided include in-memory, redis, and AWS dynamo.

## features

- `dynamo`: AWS DynamoDB
- `redis`: Redis

## install

If you want to use only the dynamo feature, install it as follows.

```
[dependencies]
cashier = { version = "0.1.0", features = ["dynamo"] }
```

If you want to use all features, use "full".

```
[dependencies]
cashier = { version = "0.1.0", features = ["full"] }
```

## Basic

```rust
use cashier::{dynamo, Cashier};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut cashier = dynamo::DynamoCashier::new()
        .table_name("cashier2")
        .config(aws_config::load_from_env().await);

    cashier.connect().unwrap();

    // If the table does not exist, it is automatically created. (Generated in on-demand mode.)
    cashier.create_table_if_not_exists().unwrap();

    // Set new data
    cashier.set("asda2sd", "value").unwrap();

    // Set a cache that expires in 10 seconds
    cashier.set_with_ttl("foo", "bar", 10).unwrap();

    // Get saved cache data.
    let foo = cashier.get("asda2sd").unwrap();
    println!("foo: {:?}", foo);

    // Reset all data.
    cashier.clear().unwrap();

    Ok(())
}

```
