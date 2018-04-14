use client::GroupmeClient;
use error::GroupmeError;

use std::rc::Rc;

pub struct Bot {
    pub(crate) bot_id: String,
    pub(crate) client: Rc<GroupmeClient>,
}

impl Bot {
    pub fn post(&self, text: &str) -> Result<(), GroupmeError> {
        let gm_client = &self.client;
        gm_client.post(&self.bot_id, text, None)?;
        Ok(())
    }

    pub fn post_image(&self, text: &str, picture_url: &str) -> Result<(), GroupmeError> {
        let gm_client = &self.client;
        gm_client.post(&self.bot_id, text, Some(picture_url))?;
        Ok(())
    }

    pub fn bot_id(&self) -> &str {
        &self.bot_id
    }
}
