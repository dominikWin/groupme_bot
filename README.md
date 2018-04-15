# groupme_bot

[Rust](https://www.rust-lang.org/) library for the [Groupme](https://groupme.com) Bots API.

The library allows for Rust programms to create, destroy, and post messages
and images to Groupme.

## Example

To use bots you need to get an API token at [dev.groupme.com](https://dev.groupme.com/).

```
use groupme_bot::{Groupme, Bot};

let groupme = Groupme::new(None);
let bot = groupme.bot("Secret bot_id");

bot.post("Hello, world!");
```
