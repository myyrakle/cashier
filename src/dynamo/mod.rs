use aws_sdk_dynamodb::{
    operation::create_table,
    types::{
        builders::KeySchemaElementBuilder, AttributeDefinition, BillingMode, KeySchemaElement,
        KeyType,
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

                let describe_output = client
                    .describe_table()
                    .table_name(table_name)
                    .send()
                    .await?;

                Ok(describe_output.table.is_some())
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
