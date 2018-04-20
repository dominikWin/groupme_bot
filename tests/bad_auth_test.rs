extern crate groupme_bot;

use groupme_bot::{Groupme, GroupmeError};

#[test]
fn test_unauthorized_post() {
    let groupme = Groupme::new(None);
    let bot = groupme.bot("Wrong bot_id");

    let response = bot.post("Hello, world!");

    let error = response.unwrap_err();

    match error {
        GroupmeError::AuthError => {}
        _ => panic!("Wrong error type returned"),
    }
}

#[test]
fn test_unauthorized_post_image() {
    let groupme = Groupme::new(None);
    let bot = groupme.bot("Wrong bot_id");

    let response = bot.post_image("Hello, world!", "http://example.com/i.png");

    let error = response.unwrap_err();

    match error {
        GroupmeError::AuthError => {}
        _ => panic!("Wrong error type returned"),
    }
}
