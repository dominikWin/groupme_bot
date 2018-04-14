use error::GroupmeError;

use reqwest::{self, Client};
use serde_json;
use std::collections::HashMap;

pub(super) struct GroupmeClient {
    path: String,
    client: Client,
}

impl GroupmeClient {
    pub(super) fn new() -> GroupmeClient {
        GroupmeClient {
            path: "https://api.groupme.com/v3".to_string(),
            client: Client::new(),
        }
    }

    pub(super) fn post(
        &self,
        bot_id: &str,
        text: &str,
        picture_url: Option<&str>,
    ) -> Result<(), GroupmeError> {
        let mut body = HashMap::new();
        body.insert("bot_id", bot_id);
        body.insert("text", text);
        if let Some(picture_url) = picture_url {
            body.insert("picture_url", picture_url);
        }

        let response = self.client
            .post(&format!("{}/bots/post", self.path))
            .json(&body)
            .send()?;

        if response.status() != reqwest::StatusCode::Accepted {
            return Err(GroupmeError::BadHeaderError(response.status()));
        }

        Ok(())
    }

    pub(super) fn create(
        &self,
        token: &str,
        name: &str,
        group_id: &str,
        avatar_url: Option<&str>,
        callback_url: Option<&str>,
        dm_notification: Option<bool>,
    ) -> Result<String, GroupmeError> {
        use serde_json::{Map, Value};
        let mut bot = Map::new();
        bot.insert("name".to_string(), Value::String(name.to_string()));
        bot.insert("group_id".to_string(), Value::String(group_id.to_string()));
        if let Some(avatar_url) = avatar_url {
            bot.insert(
                "avatar_url".to_string(),
                Value::String(avatar_url.to_string()),
            );
        }
        if let Some(callback_url) = callback_url {
            bot.insert(
                "callback_url".to_string(),
                Value::String(callback_url.to_string()),
            );
        }
        if let Some(dm_notification) = dm_notification {
            bot.insert("dm_notification".to_string(), Value::Bool(dm_notification));
        }

        let mut body = Map::new();
        body.insert("bot".to_string(), Value::Object(bot));
        let body = Value::Object(body);
        let mut response = self.client
            .post(&format!("{}/bots?token={}", self.path, token))
            .json(&body)
            .send()?;
        if response.status() == reqwest::StatusCode::Unauthorized {
            return Err(GroupmeError::Unauthorized);
        }
        if response.status() != reqwest::StatusCode::Created {
            return Err(GroupmeError::BadHeaderError(response.status()));
        }

        let response_text = response.text()?;
        let response_json: Value = serde_json::from_str(&response_text)?;

        let bot_id = if let Value::String(ref bot_id) = response_json["response"]["bot"]["bot_id"] {
            bot_id.clone()
        } else {
            return Err(GroupmeError::GenericError);
        };
        Ok(bot_id)
    }

    pub(super) fn destroy(&self, bot_id: &str, token: &str) -> Result<(), GroupmeError> {
        let mut body = HashMap::new();
        body.insert("bot_id", bot_id.to_string());
        let response = self.client
            .post(&format!("{}/bots/destroy?token={}", self.path, token))
            .json(&body)
            .send()?;

        if response.status() == reqwest::StatusCode::Unauthorized {
            return Err(GroupmeError::Unauthorized);
        }
        if response.status() != reqwest::StatusCode::Ok {
            return Err(GroupmeError::BadHeaderError(response.status()));
        }

        Ok(())
    }
}
