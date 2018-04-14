extern crate groupme_bot;

use groupme_bot::{Bot, Groupme};

fn main() {
    let groupme = Groupme::new("Your Groupme Token");
    let bot: Bot = groupme.bot("A bot_id");
    bot.post("Hi").unwrap();
}
