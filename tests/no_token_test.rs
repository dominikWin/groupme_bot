extern crate groupme_bot;

use groupme_bot::{Groupme, GroupmeError};

#[test]
fn test_create_no_token() {
    let groupme = Groupme::new(None);

    let bot = groupme.create_bot("name", "some wrong group id");

    let error = bot.unwrap_err();

    match error {
        GroupmeError::NoTokenError => {}
        _ => panic!("Wrong error type returned"),
    }
}

#[test]
fn test_destroy_no_token() {
    let groupme = Groupme::new(None);

    let bot = groupme.bot("some wrong id");

    let response = groupme.destroy(bot);

    let error = response.unwrap_err();

    match error {
        GroupmeError::NoTokenError => {}
        _ => panic!("Wrong error type returned"),
    }
}
