use async_trait::async_trait;
use aws_sdk_sns::{Region, Client};
use serde::Deserialize;
use serde_json::Value;
use tracing::info;

use crate::core::action_builder::{Action, ActionError};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SnsV1 {
    topic_arn: String,
    #[serde(default = "default_region")]
    region: String,
    #[serde(default)]
    message: String,
}

#[async_trait]
impl Action for SnsV1 {
    fn init(attributes: Value) -> Result<Self, ActionError> {
        Ok(serde_json::from_value(attributes)?)
    }

    async fn exec(&self) -> Result<(), ActionError> {
        let region = Region::new(self.region.clone());
        let cfg = aws_config::from_env().region(region).load().await;
        let client = Client::new(&cfg);

        let pub_res = client
            .publish()
            .topic_arn(&self.topic_arn)
            .message(&self.message)
            .send()
            .await?;

        info!(sns_message_id = pub_res.message_id().unwrap_or("not_found"), "SNS message publish success");
        Ok(())
    }
}

fn default_region() -> String {
    "us-west-2".to_string()
}
