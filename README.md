# music-chat-tracker
This is a Rust program you can run locally that monitors your iMessages of a chosen chat looking for Spotify track URLs and adding any found tracks to a single Spotify playlist. This can run on any system that saves iMessages in a `chat.db` fashion, so likely only a Macbook.

## Background
#### Context
I am in a several-years-old group chat with a group of friends where we discuss music, and share songs through Spotify links.
#### Problem
Many of the chat members like to "claim" songs, i.e. make sure everyone knows they were the one who "found it." So, this causes a stir whenever a song that was previously shared is shared again. Due to the age of the group chat, and the lackluster search feature in Apple Messages, it can be difficult to determine if a particular song has been shared before.
#### Solution
Make an automatically updated Spotify playlist that acts as an archive of all songs that have been shared in the group chat. This still does not let us track who previously sent a song, but it instead can act as a log to check against before you accidentally re-share a song and get attacked by the music-mob.
#### Why Rust?
I went with Rust for two reasons. First, I am doing a lot more coding in Rust at work and thought it would be a good refresher project! But the main reason is that I keep this program constantly running on a 2017 Macbook that is on half of its last leg, so I wanted to make this as lightweight as I reasonably could. I prototyped this program in a Jupyter Notebook using `spotipy` and my Macbook would regularly shut off because it couldn't handle it. Also, someone had kindly built `rspotify` that already did everything I needed.

## Dev Environment Setup
### Requirements
This project should be run on MacOS with Messages signed in and synced. But it could be setup to work on a different OS as long there is a `chat.db` sqlite database of the messages available.
You will need Rust installed:
- [Install Rust](https://www.rust-lang.org/tools/install) `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

### IDE
We prefer to use VSCode:
- [Install VSCode](https://code.visualstudio.com/download)
- Add the [`rust-analyzer` extension](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

### Environment Variables
To update the playlist through the Spotify Web API you will need a client id, client secret, and redirect uri. These can be found by getting a spotify dev license
[https://developer.spotify.com/dashboard/create](https://developer.spotify.com/dashboard/create). If you don't care about the redirect uri then you can really set it to anything, like http://spotify.com or http://google.com.

You will need to save these values as environment variables in the .env file:
```
RSPOTIFY_CLIENT_ID="your-client-id"
RSPOTIFY_CLIENT_SECRET="your-client-secret"
RSPOTIFY_REDIRECT_URI="your-redirect-uri"
CHAT_DB_PATH = "~/Library/Messages/chat.db"
CHAT_DISPLAY_NAME = "Name of The Chat"
PLAYLIST_ID = "yourplaylistid1234"
DEFAULT_FILTER_START_DATE = "YYYY-MM-DD"
```

### Full Disk Access
On MacOS, for the message exporter script to have access to your messages in `~Library/Messages/chat.db` you will need to [give Full Disk Access](https://kb.synology.com/en-us/C2/tutorial/How_to_enable_Full_Disk_Access_on_a_Mac) to your IDE.
1. Click on the Apple icon on the top left corner of your screen.
2. Select System Preferences.
3. Go to Security & Privacy Preferences > Privacy and click Full Disk Access from the left panel.
![image](https://github.com/christianaaronschroeder/music-chat-tracker/assets/43764673/41a6f93d-39e2-4e0e-a40a-b6f601a2e370)

Error you might see if Full Disk Access is not given:
```
thread 'main' panicked at messages-scraper/src/lib.rs:93:102:
Failed to export chat messages: Custom { kind: NotFound, error: "No file was exported to /var/folders/s8/8xydrdd946l1t00fxgxbh27m0000gn/T/.tmpipDVWs/messages_export.html" }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

### Running it!
#### How I've Been Running It
To take advantage of Rust's optimizations I build a release whenever I make a change and then run it with a 5-minute update interval, `RUST_LOG=info ./target/release/music-chat-tracker --update-interval-s 600`
#### Basic Commands
- Build it: `cargo build`
- Build it with optimizations and without debug symbols: `cargo build --release`
- Test it: `cargo test`
- Run it: `cargo run`
- Run it with logs: `RUST_LOG=info cargo run`
#### Arguments
You can use the following command line arguments to override the values in your .env:

`--playlist-id`, `-p` - The ID of the playlist.

`--chat-display-name`, `-c` - The display name for the chat.

`--filter-start-date`, `-f` - The start date for filtering.

`--update-interval-s`, `-i` - The update interval in seconds. Defaults to `60*60`, 1 hour.

`--chat-db-path`, `-d` - The path to the chat database.
