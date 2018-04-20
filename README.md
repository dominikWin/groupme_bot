# groupme_bot

[![Crates.io](https://img.shields.io/crates/v/groupme_bot.svg)](https://crates.io/crates/groupme_bot)
[![Build Status](https://travis-ci.org/dominikWin/groupme_bot.svg?branch=master)](https://travis-ci.org/dominikWin/groupme_bot)


[Rust](https://www.rust-lang.org/) library for the [Groupme](https://groupme.com) Bots API.

The library allows for Rust programms to create, destroy, and post messages
and images to Groupme.

## Example

To use bots you need to get an API token at [dev.groupme.com](https://dev.groupme.com/).

Bots can be created either online or through the library.

```
use groupme_bot::{Groupme, Bot};

// Posting doesn't require a token
let groupme = Groupme::new(None);

let bot = groupme.bot("Secret bot_id");

bot.post("Hello, world!").unwrap();
```
