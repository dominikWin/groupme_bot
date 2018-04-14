use std::rc::Rc;
use reqwest::{self, Client};
use bot::Bot;
use std::collections::HashMap;
use serde_json;
use error::GroupmeError;

pub struct Groupme {
    token: Option<String>,
    client: Rc<GroupmeClient>,
}

impl Groupme {
    pub fn new(token: Option<&str>) -> Groupme {
        let gm_client = GroupmeClient {
            path: "https://api.groupme.com/v3".to_string(),
            client: Client::new(),
        };

        Groupme {
            token: token.and_then(|s| Some(s.to_string())),
            client: Rc::new(gm_client),
        }
    }

    pub fn bot(&self, bot_id: &str) -> Bot {
        Bot {
            bot_id: bot_id.to_string(),
            client: self.client.clone(),
        }
    }

    pub fn create_bot(&self, name: &str, group_id: &str) -> Result<BotBuilder, GroupmeError> {
        if self.token.is_none() {
            return Err(GroupmeError::NoTokenError);
        }
        Ok(BotBuilder {
            name: name.to_string(),
            group_id: group_id.to_string(),
            avatar_url: None,
            callback_url: None,
            dm_notification: None,
            client: self.client.clone(),
            token: self.token.clone().unwrap(),
        })
    }
}

pub struct BotBuilder {
    name: String,
    group_id: String,
    avatar_url: Option<String>,
    callback_url: Option<String>,
    dm_notification: Option<bool>,

    client: Rc<GroupmeClient>,
    token: String,
}

impl BotBuilder {
    pub fn with_avatar_url(mut self, avatar_url: &str) -> Self {
        self.avatar_url = Some(avatar_url.to_string());
        self
    }

    pub fn with_callback_url(mut self, callback_url: &str) -> Self {
        self.callback_url = Some(callback_url.to_string());
        self
    }

    pub fn with_dm_notification(mut self, dm_notification: bool) -> Self {
        self.dm_notification = Some(dm_notification);
        self
    }

    pub fn create(self) -> Result<Bot, GroupmeError> {
        let gm_client: &GroupmeClient = &self.client;
        let bot_id = gm_client.create(
            &self.token,
            &self.name,
            &self.group_id,
            self.avatar_url.as_ref().map(String::as_str),
            self.callback_url.as_ref().map(String::as_str),
            self.dm_notification,
        )?;
        Ok(Bot {
            bot_id,
            client: self.client.clone(),
        })
    }
}

pub(super) struct GroupmeClient {
    path: String,
    client: Client,
}

impl GroupmeClient {
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
        if response.status() == reqwest::StatusCode::NotFound {
            return Err(GroupmeError::BotNotFound);
        }

        if response.status() != reqwest::StatusCode::Accepted {
            return Err(GroupmeError::BadHeaderError(format!(
                "Wrong header response: {:?}",
                response.status()
            )));
        }

        Ok(())
    }

    pub fn create(
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
            return Err(GroupmeError::BadHeaderError(format!(
                "Wrong header response: {:?}",
                response.status()
            )));
        }

        let response_text = response.text()?;
        let response_json: Value = serde_json::from_str(&response_text)?;

        let bot_id = if let Value::String(ref bot_id) = response_json["response"]["bot"]["bot_id"] {
            bot_id.clone()
        } else {
            return Err(GroupmeError::BotNotFound);
        };
        Ok(bot_id)
    }
}
