mod messages_scraper;
mod playlist_handler;

use messages_scraper::get_tracks_from_messages;
use playlist_handler::add_tracks_to_playlist;

use log::info;
use env_logger;
use std::time::Duration;
use std::thread::sleep;


fn update_playlist(playlist_id_str: &str, filter_start_date: &str) {
    let track_ids_to_add: Vec<String> = get_tracks_from_messages(filter_start_date).expect("Failed to get tracks from messages");
    info!("{:?} tracks found in the chat", track_ids_to_add.len());
    add_tracks_to_playlist(playlist_id_str, track_ids_to_add);
}

fn main() {
    env_logger::init();
    let interval = Duration::from_secs(60*60); // run the update every hour
    let playlist_id_str: &str = "7hVMUyFFi6bNtjO4hubtJm";
    let filter_start_date: &str = "2024-07-01";

    loop {
        update_playlist(playlist_id_str, filter_start_date);
        sleep(interval);
    }
}
