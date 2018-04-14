extern crate groupme_bot;

use groupme_bot::{Bot, Groupme};
use std::env;

fn main() {
    let group_id = &env::var("GROUPME_BOT_ID").unwrap();

    let groupme = Groupme::new(None);
    let bot: Bot = groupme.bot(group_id);

    bot.post("Hello from Rust!").unwrap();
}
