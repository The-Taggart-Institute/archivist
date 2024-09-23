# The Archivist

![archivist magic card](https://static.starcitygames.com/sales/cardscans/MTG/ULG/en/nonfoil/Archivist.jpg)

Discord Bot for archiving links to a Wallabag RSS feed.

This Discord bot is designed to work in conjunction with [Wallabag](https://wallabag.it/), either the service or a self-hosted instance. Using the Wallabag API, it will store URLs from Discord messages for better archiving of resources.

## Installation

1. Create a Discord App with default permissions. Save the app token.
2. Clone this repository to a server of your choosing.
3. Create a Wallabag account for use with this app. Save the client ID, secret, URL, username, and password.
4. Copy `config.example.json` to `config.json`.
5. Fill in the `config.json` details with the saved information.
6. `cargo install --path .` and then `archivist` in this directory for the Rustaceans, or use `docker compose up -d` with the associated Dockerfile.
7. Use the Discord app install link on a server of your choosing.
8. DM `@The Archivist register`, then regiser the commands.
9. Get Archivin'!

## Usage

### Archivin'

`/archive <url> <tags,tags,tags>`

### Status

`@The Archivist status` 
