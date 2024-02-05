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
        tokio::task::block_in_place(move || {
            Handle::current().block_on(async move {
                let config = match &self.config {
                    Some(config) => config.to_owned(),
                    None => aws_config::load_from_env().await,
                };

                self.client = Some(aws_sdk_dynamodb::Client::new(&config));
            });
        });

        Ok(())
    }
}
