# YouTube Playlist Search

A Rust implementation of a YouTube playlist search tool.

Details available in the project [documentation](https://github.com/pwschaedler/ytps-rust/wiki).

## Usage

```sh
cargo run
```

### Logging In and Authenticating

To login and authenticate with OAuth2 in order to read private playlists, a JSON client secret retrieved via the Google Cloud Console is required. See [Google's documentation](https://developers.google.com/identity/protocols/oauth2) for more details on obtaining a client secret.

For desktop applications, the client secret is not considered "secret" and is allowed to be embedded within an application, but this package does not provide a built-in secret. Users must generate their own and provide the secret to authenticate.

```sh
cargo run login client_secret.json
```

Once authenticated, tokens will be cached in a [standard directory location](https://codeberg.org/dirs/directories-rs). Login should only be required once.
