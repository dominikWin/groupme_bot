extern crate groupme_bot;

use groupme_bot::{Groupme, GroupmeError};

// All of the strings provided here are wrong.
//
// This tests that the library properly returns
// the correct error.

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

#[test]
fn test_unauthorized_create() {
    let groupme = Groupme::new(Some("Bad API Token"));

    let bot = groupme.create_bot("name", "group id").unwrap().create();

    let error = bot.unwrap_err();

    match error {
        GroupmeError::AuthError => {}
        _ => panic!("Wrong error type returned"),
    }
}

#[test]
fn test_unauthorized_destroy() {
    let groupme = Groupme::new(Some("Bad API Token"));

    let bot = groupme.bot("bot_id");

    let response = groupme.destroy(bot);

    let error = response.unwrap_err();

    match error {
        GroupmeError::AuthError => {}
        _ => panic!("Wrong error type returned"),
    }
}
