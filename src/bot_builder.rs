use client::GroupmeClient;
use error::GroupmeError;
use bot::Bot;

use std::rc::Rc;

/// Builder to create Bots with configuration and add them to Groupme.
///
/// # Examples
///
/// ```
/// use groupme_bot::{Groupme, Bot};
///
/// let groupme: Groupme = Groupme::new(Some("Required API Token"));
/// let bot: Bot = groupme
///     .create_bot("My Bot", "a group id")
///     .unwrap()
///     .with_avatar_url("some url")
///     .with_callback_url("some other url")
///     .with_dm_notification(true)
///     .create()
///     .unwrap();
/// ```
#[derive(Debug)]
pub struct BotBuilder {
    name: String,
    group_id: String,
    client: Rc<GroupmeClient>,
    token: String,

    avatar_url: Option<String>,
    callback_url: Option<String>,
    dm_notification: Option<bool>,
}

impl BotBuilder {
    pub(super) fn new(
        name: &str,
        group_id: &str,
        client: Rc<GroupmeClient>,
        token: &str,
    ) -> BotBuilder {
        BotBuilder {
            name: name.to_string(),
            group_id: group_id.to_string(),
            client,
            token: token.to_string(),

            avatar_url: None,
            callback_url: None,
            dm_notification: None,
        }
    }

    /// Sets the avatar_url for a new Bot.
    ///
    /// Groupme will only accept image urls from their [Image Service](https://dev.groupme.com/docs/image_service).
    pub fn with_avatar_url(mut self, avatar_url: &str) -> Self {
        self.avatar_url = Some(avatar_url.to_string());
        self
    }

    /// Sets the callback_url for a new Bot.
    pub fn with_callback_url(mut self, callback_url: &str) -> Self {
        self.callback_url = Some(callback_url.to_string());
        self
    }

    /// Sets the dm_notificaion for a new Bot.
    pub fn with_dm_notification(mut self, dm_notification: bool) -> Self {
        self.dm_notification = Some(dm_notification);
        self
    }

    /// Builds a `Bot` with the set values and adds it to a group.
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
