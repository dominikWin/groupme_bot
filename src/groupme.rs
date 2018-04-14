use bot::Bot;
use error::GroupmeError;
use client::GroupmeClient;
use bot_builder::BotBuilder;

use std::rc::Rc;

pub struct Groupme {
    token: Option<String>,
    client: Rc<GroupmeClient>,
}

impl Groupme {
    pub fn new(token: Option<&str>) -> Groupme {
        Groupme {
            token: token.and_then(|s| Some(s.to_string())),
            client: Rc::new(GroupmeClient::new()),
        }
    }

    pub fn bot(&self, bot_id: &str) -> Bot {
        Bot {
            bot_id: bot_id.to_string(),
            client: self.client.clone(),
        }
    }

    pub fn create_bot(&self, name: &str, group_id: &str) -> Result<BotBuilder, GroupmeError> {
        if let Some(ref token) = self.token {
            let builder = BotBuilder::new(name, group_id, self.client.clone(), &token);
            return Ok(builder);
        }
        Err(GroupmeError::NoTokenError)
    }

    pub fn destroy(&self, bot: Bot) -> Result<(), GroupmeError> {
        let bot_id = bot.bot_id();
        if let Some(ref token) = self.token {
            return self.client.destroy(bot_id, &token);
        }
        Err(GroupmeError::NoTokenError)
    }
}
