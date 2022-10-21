use async_trait::async_trait;
use serde::Deserialize;
use serde_json::Value;
use tracing::info;

use crate::core::action_builder::{Action, ActionError};

#[derive(Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
enum AuthMethod {
    Basic {
        username: String,
        password: Option<String>,
    },
    Bearer {
        token: String,
    },
    None,
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
enum HttpMethod {
    Get,
    Post,
    Patch,
    Put,
    Delete,
}

#[derive(Deserialize)]
pub struct HttpV1 {
    auth: AuthMethod,
    url: String,
    method: HttpMethod,
    #[serde(default)]
    body: String,
}

#[async_trait]
impl Action for HttpV1 {
    fn init(attributes: Value) -> Result<Self, ActionError> {
        Ok(serde_json::from_value(attributes)?)
    }

    async fn exec(&self) -> Result<(), ActionError> {
        let client = reqwest::Client::new();
        let url = self.url.clone();

        let mut builder = match self.method {
            HttpMethod::Get => client.get(url),
            HttpMethod::Post => client.post(url),
            HttpMethod::Patch => client.patch(url),
            HttpMethod::Put => client.put(url),
            HttpMethod::Delete => client.delete(url),
        };

        builder = match (&self.method, self.body.to_string().len()) {
            (HttpMethod::Get, _) => builder,
            (_, 0) => builder,
            (_, _) => builder
                .header("Content-Type", "application/json")
                .body(self.body.clone()),
        };

        builder = match &self.auth {
            AuthMethod::Basic { username, password } => {
                builder.basic_auth(username, password.clone())
            }
            AuthMethod::Bearer { token } => builder.bearer_auth(token),
            AuthMethod::None => builder,
        };

        let res = builder.send().await?;
        info!(status = res.status().to_string());

        Ok(())
    }
}
