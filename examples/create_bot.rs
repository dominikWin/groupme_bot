extern crate groupme_bot;

use groupme_bot::Groupme;

use std::{thread, time};
use std::env;

fn main() {
    let token = &env::var("GROUPME_TOKEN").unwrap();
    let group_id = &env::var("GROUPME_GROUP").unwrap();

    let groupme = Groupme::new(Some(token));
    let bot = groupme
        .create_bot("My Bot", group_id)
        .unwrap()
        .create()
        .unwrap();

    // Wait 5 seconds
    thread::sleep(time::Duration::new(5, 0));

    bot.post("Hello, world!").unwrap();
}
