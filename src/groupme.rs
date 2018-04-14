use bot::Bot;
use error::GroupmeError;
use client::GroupmeClient;

use std::rc::Rc;

pub struct Groupme {
    token: Option<String>,
    client: Rc<GroupmeClient>,
}

impl Groupme {
    pub fn new(token: Option<&str>) -> Groupme {
        let gm_client = GroupmeClient::new();

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
