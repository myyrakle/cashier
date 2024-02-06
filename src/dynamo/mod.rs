use aws_sdk_dynamodb::{
    error::SdkError,
    operation::create_table,
    types::{
        builders::{AttributeDefinitionBuilder, KeySchemaElementBuilder},
        AttributeDefinition, AttributeValue, BillingMode, KeySchemaElement, KeyType,
        ScalarAttributeType,
    },
};
use tokio::runtime::Handle;

pub use super::Cashier;

#[derive(Debug, Clone)]
pub struct DynamoCashier {
    config: Option<aws_config::SdkConfig>,
    client: Option<aws_sdk_dynamodb::Client>,
    table_name: Option<String>,
}

impl DynamoCashier {
    /// Create a new DynamoCashier
    pub fn new() -> Self {
        DynamoCashier {
            client: None,
            table_name: None,
            config: None,
        }
    }

    /// set the table name
    pub fn table_name(mut self, table_name: &str) -> Self {
        self.table_name = Some(table_name.to_string());
        self
    }

    /// set the AWS config
    pub fn config(mut self, config: aws_config::SdkConfig) -> Self {
        self.config = Some(config);
        self
    }

    /// Connect to a DynamoDB server
    pub fn connect(&mut self) -> anyhow::Result<()> {
        tokio::task::block_in_place(|| {
            Handle::current().block_on(async {
                let config = match &self.config {
                    Some(config) => config.to_owned(),
                    None => aws_config::load_from_env().await,
                };

                self.client = Some(aws_sdk_dynamodb::Client::new(&config));
            });
        });

        Ok(())
    }

    /// Check if the table exists
    pub fn check_exists(&self) -> anyhow::Result<bool> {
        tokio::task::block_in_place(|| {
            Handle::current().block_on(async {
                let client = match &self.client {
                    Some(client) => client,
                    None => return Err(anyhow::anyhow!("client is not set")),
                };

                let table_name = match &self.table_name {
                    Some(table_name) => table_name,
                    None => return Err(anyhow::anyhow!("table_name is not set")),
                };

                let describe_output = client.describe_table().table_name(table_name).send().await;

                match describe_output {
                    Err(SdkError::ServiceError(err))
                        if err.err().is_resource_not_found_exception() =>
                    {
                        Ok(false)
                    }
                    Err(err) => Err(err.into()),
                    Ok(_) => Ok(true),
                }
            })
        })
    }

    /// Create table
    pub fn create_table(&self) -> anyhow::Result<()> {
        let result = tokio::task::block_in_place(|| {
            Handle::current().block_on(async {
                let client = match &self.client {
                    Some(client) => client,
                    None => return Err(anyhow::anyhow!("client is not set")),
                };

                let table_name = match &self.table_name {
                    Some(table_name) => table_name,
                    None => return Err(anyhow::anyhow!("table_name is not set")),
                };

                let _ = client
                    .create_table()
                    .table_name(table_name)
                    .attribute_definitions(
                        AttributeDefinitionBuilder::default()
                            .attribute_name("key".to_string())
                            .attribute_type(ScalarAttributeType::S)
                            .build()
                            .unwrap(),
                    )
                    .key_schema(
                        KeySchemaElementBuilder::default()
                            .attribute_name("key".to_string())
                            .key_type(KeyType::Hash)
                            .build()
                            .unwrap(),
                    )
                    .billing_mode(BillingMode::PayPerRequest)
                    .send()
                    .await?;

                Ok(())
            })
        });

        result
    }

    /// Create table if not exists
    pub fn create_table_if_not_exists(&self) -> anyhow::Result<()> {
        let exists = self.check_exists()?;

        if exists {
            return Ok(());
        }

        self.create_table()
    }
}

impl Cashier for DynamoCashier {
    fn set(&self, key: &str, value: &str) -> anyhow::Result<()> {
        let result = tokio::task::block_in_place(|| {
            Handle::current().block_on(async {
                let client = match &self.client {
                    Some(client) => client,
                    None => return Err(anyhow::anyhow!("client is not set")),
                };

                let table_name = match &self.table_name {
                    Some(table_name) => table_name,
                    None => return Err(anyhow::anyhow!("table_name is not set")),
                };

                client
                    .put_item()
                    .table_name(table_name)
                    .item("key", AttributeValue::S(key.into()))
                    .item("value", AttributeValue::S(value.into()))
                    .send()
                    .await?;

                Ok(())
            })
        });

        result
    }

    fn set_with_ttl(&self, key: &str, value: &str, ttl: u64) -> anyhow::Result<()> {
        todo!()
        // let result = tokio::task::block_in_place(|| {
        //     Handle::current().block_on(async {
        //         let client = match &self.client {
        //             Some(client) => client,
        //             None => return Err(anyhow::anyhow!("client is not set")),
        //         };

        //         let table_name = match &self.table_name {
        //             Some(table_name) => table_name,
        //             None => return Err(anyhow::anyhow!("table_name is not set")),
        //         };

        //         let expired_at = (chrono::Utc::now() + chrono::Duration::seconds(ttl as i64))
        //             .timestamp()
        //             .to_string();

        //         client
        //             .put_item()
        //             .table_name(table_name)
        //             .item("key", value.into())
        //             .item("value", value.into())
        //             .item("expiredAt", expired_at.into())
        //             .send()
        //             .await?;

        //         Ok(())
        //     })
        // });

        // result
    }

    fn get(&self, key: &str) -> anyhow::Result<Option<String>> {
        todo!()
    }

    fn delete(&self, key: &str) -> anyhow::Result<()> {
        todo!()
    }

    fn clear(&self) -> anyhow::Result<()> {
        todo!()
    }
}
