use std::rc::Rc;
use reqwest::{self, Client};
use bot::Bot;
use std::collections::HashMap;

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
}

#[derive(Debug)]
pub enum GroupmeError {
    BotNotFound,
    ReqwestError(reqwest::Error),
}

impl From<reqwest::Error> for GroupmeError {
    fn from(error: reqwest::Error) -> Self {
        GroupmeError::ReqwestError(error)
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

        assert_eq!(response.status(), reqwest::StatusCode::Accepted);

        Ok(())
    }
}
