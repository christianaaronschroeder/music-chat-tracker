mod messages_scraper;
mod playlist_handler;

use messages_scraper::get_tracks_from_messages;
use playlist_handler::add_tracks_to_playlist;

use env_logger;
use log::info;
use std::thread::sleep;
use std::time::Duration;

fn update_playlist(playlist_id_str: &str, filter_start_date: &str) {
    let chat_db_path = "../Library/Messages/chat.db";
    let chat_display_name = "Music (A Little Spam)";
    let track_ids_to_add: Vec<String> =
        get_tracks_from_messages(chat_db_path, chat_display_name, filter_start_date, None)
            .expect("Failed to get tracks from messages");
    info!("{:?} tracks found in the chat", track_ids_to_add.len());
    add_tracks_to_playlist(playlist_id_str, track_ids_to_add);
}

fn main() {
    env_logger::init();
    let interval = Duration::from_secs(60 * 60); // run the update every hour
    let playlist_id_str: &str = "7hVMUyFFi6bNtjO4hubtJm";
    let filter_start_date: &str = "2024-07-01";

    loop {
        update_playlist(playlist_id_str, filter_start_date);
        sleep(interval);
    }
}

// temp for real tests
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }
}
