use playlist_handler::add_tracks_to_playlist;
use messages_scraper::get_tracks_from_messages;
use log::info;
use env_logger;

fn main() {
    env_logger::init();
    let track_ids_to_add = get_tracks_from_messages("2024-06-20").expect("Failed to get tracks from messages");
    info!("{:?} tracks found in the chat", track_ids_to_add.len());
    let playlist_id_str: &str = "3op9QLxlW6byL2uJEMLyIC";
    add_tracks_to_playlist(playlist_id_str, track_ids_to_add);
}