use playlist_handler::add_tracks_to_playlist;
use messages_scraper::get_tracks_from_messages;

fn main() {
    let track_ids_to_add: Vec<String> = get_tracks_from_messages();
    add_tracks_to_playlist(track_ids_to_add);
}