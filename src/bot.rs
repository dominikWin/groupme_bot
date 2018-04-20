use client::GroupmeClient;
use error::GroupmeError;

use std::rc::Rc;

/// Groupme Bot interface for posting messages.
#[derive(Debug)]
pub struct Bot {
    pub(crate) bot_id: String,
    pub(crate) client: Rc<GroupmeClient>,
}

impl Bot {
    /// Causes the bot to post a new message.
    ///
    /// # Errors
    ///
    /// Returns an `Err(GroupmeError::BadHeaderError(reqwest::StatusCode::NotFound))`
    /// if the bot_id is not found.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use groupme_bot::{Groupme, Bot};
    ///
    /// let groupme = Groupme::new(None);
    /// let bot: Bot = groupme.bot("Some bot_id");
    ///
    /// bot.post("Hello from Rust!").unwrap();
    /// ```
    pub fn post(&self, text: &str) -> Result<(), GroupmeError> {
        let gm_client = &self.client;
        gm_client.post(&self.bot_id, text, None)?;
        Ok(())
    }

    /// Causes the bot to post an image and message.
    ///
    /// Groupme will only accept image urls from their [Image Service](https://dev.groupme.com/docs/image_service).
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use groupme_bot::{Groupme, Bot};
    ///
    /// let groupme = Groupme::new(None);
    /// let bot: Bot = groupme.bot("Some bot_id");
    ///
    /// bot.post_image("Hello from Rust!", "https://i.groupme.com/something.large").unwrap();
    /// ```

    pub fn post_image(&self, text: &str, picture_url: &str) -> Result<(), GroupmeError> {
        let gm_client = &self.client;
        gm_client.post(&self.bot_id, text, Some(picture_url))?;
        Ok(())
    }

    /// Returns the bot_id for any created bot.
    ///
    /// Is not garanteed to match Groupme API if the `Bot` was created from its bot_id.
    pub fn bot_id(&self) -> &str {
        &self.bot_id
    }
}
