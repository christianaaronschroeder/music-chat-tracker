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
