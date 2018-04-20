use bot::Bot;
use error::GroupmeError;
use client::GroupmeClient;
use bot_builder::BotBuilder;

use std::rc::Rc;

/// Local representation of Groupme API account for managing bots.
#[derive(Debug)]
pub struct Groupme {
    token: Option<String>,
    client: Rc<GroupmeClient>,
}

impl Groupme {
    /// Coustructs a new `Groupme`.
    ///
    /// Takes an optional API access token.
    /// The token is not needed to use post messages.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use groupme_bot::Groupme;
    ///
    /// let groupme: Groupme = Groupme::new(None);
    ///
    /// let groupme: Groupme = Groupme::new(Some("Secret API Token"));
    /// ```
    pub fn new(token: Option<&str>) -> Groupme {
        Groupme {
            token: token.and_then(|s| Some(s.to_string())),
            client: Rc::new(GroupmeClient::new()),
        }
    }

    /// Creates a `Bot` from its bot id.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use groupme_bot::{Groupme, Bot};
    ///
    /// let groupme: Groupme = Groupme::new(None);
    ///
    /// let bot: Bot = groupme.bot("Secret bot_id");
    /// ```
    pub fn bot(&self, bot_id: &str) -> Bot {
        Bot {
            bot_id: bot_id.to_string(),
            client: self.client.clone(),
        }
    }

    /// Creates a new Groupme for a group.
    ///
    /// Takes a group_id.
    /// Group ids can be found with the groups API:
    ///
    /// ```shell
    /// curl https://api.groupme.com/v3/groups?token=<token>
    /// ```
    ///
    /// # Errors
    ///
    /// Will return a `GroupmeError::NoTokenError` if there is no provided token.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use groupme_bot::{Groupme, Bot};
    ///
    /// let groupme: Groupme = Groupme::new(Some("Required API Token"));
    /// let bot = groupme
    ///     .create_bot("My bot", "Required group ID")
    ///     .unwrap()
    ///     .create()
    ///     .unwrap();
    /// ```
    ///
    pub fn create_bot(&self, name: &str, group_id: &str) -> Result<BotBuilder, GroupmeError> {
        if let Some(ref token) = self.token {
            let builder = BotBuilder::new(name, group_id, self.client.clone(), &token);
            return Ok(builder);
        }
        Err(GroupmeError::NoTokenError)
    }

    /// Destroys a bot and removes it from its group.
    ///
    /// # Errors
    ///
    /// Will return a `GroupmeError::NoTokenError` if there is no provided token.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use groupme_bot::{Groupme, Bot};
    ///
    /// let groupme: Groupme = Groupme::new(Some("Required API Token"));
    ///
    /// let bot: Bot = groupme.bot("Secret bot_id");
    ///
    /// groupme.destroy(bot);
    /// ```
    pub fn destroy(&self, bot: Bot) -> Result<(), GroupmeError> {
        let bot_id = bot.bot_id();
        if let Some(ref token) = self.token {
            return self.client.destroy(bot_id, &token);
        }
        Err(GroupmeError::NoTokenError)
    }
}
