# music-chat-tracker

install rust
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

also need php installed, for now

extensions
```
rust-analyzer
```


need these env variables in a .env file
```
RSPOTIFY_CLIENT_ID
RSPOTIFY_CLIENT_SECRET
RSPOTIFY_REDIRECT_URI
```

need to give full disk access to VSCode and the terminal

`RUST_LOG=info cargo run` to run with logging


`messages-exporter-copy.php` is copied from here, https://github.com/cfinke/OSX-Messages-Exporter


TODO:
- get added date of most recent song to use as the filter start date
- rewrite that giant php thing in Rust, and make it smaller for only what I need
- setup contiuous running, check for updates ever day or few hours
