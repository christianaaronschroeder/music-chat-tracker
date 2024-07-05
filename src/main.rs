mod messages_scraper;
mod playlist_handler;

use messages_scraper::get_tracks_from_messages;
use playlist_handler::add_tracks_to_playlist;

use env_logger;
use log::info;
use std::thread::sleep;
use std::time::Duration;

use clap::{Arg, Command};
use dotenv::dotenv;
use std::env;
use tempfile::tempdir;

const DEFAULT_UPDATE_INTERVAL_S: u64 = 60 * 60 * 6; // 6 hours

fn update_playlist(
    playlist_id_str: &str,
    filter_start_date: &str,
    chat_db_path: &str,
    chat_display_name: &str,
    latest_message_id_file_path: &std::path::Path,
) {
    // get the tracks from the chat
    let track_ids_to_add: Vec<String> = get_tracks_from_messages(
        chat_db_path,
        chat_display_name,
        filter_start_date,
        None,
        latest_message_id_file_path,
    )
    .expect("Failed to get tracks from messages");
    info!("{:?} tracks found in the chat", track_ids_to_add.len());

    // add tracks to the playlist
    if !track_ids_to_add.is_empty() {
        add_tracks_to_playlist(playlist_id_str, track_ids_to_add, chat_display_name);
    } else {
        info!("No new tracks to add to the playlist");
    }
}

fn main() {
    env_logger::init();

    let matches = Command::new("Env CLI Example")
        .arg(Arg::new("playlist_id").short('p').long("playlist-id"))
        .arg(
            Arg::new("chat_display_name")
                .short('c')
                .long("chat-display-name"),
        )
        .arg(
            Arg::new("filter_start_date")
                .short('f')
                .long("filter-start-date"),
        )
        .arg(
            Arg::new("update_interval_s")
                .short('i')
                .long("update-interval-s"),
        )
        .arg(Arg::new("chat_db_path").short('d').long("chat-db-path"))
        .get_matches();

    // Read the defaults from the environment variables
    dotenv().ok();
    let chat_db_path_env = env::var("CHAT_DB_PATH").unwrap();
    let chat_display_name_env = env::var("CHAT_DISPLAY_NAME").unwrap();
    let playlist_id_str_env = env::var("PLAYLIST_ID").unwrap();
    let filter_start_date_env = env::var("DEFAULT_FILTER_START_DATE").unwrap();

    // Overwrite environment variables with command line arguments if provided
    let playlist_id_str = matches
        .get_one::<String>("playlist_id")
        .unwrap_or(&playlist_id_str_env);
    let chat_db_path = matches
        .get_one::<String>("chat_db_path")
        .unwrap_or(&chat_db_path_env);
    let chat_display_name = matches
        .get_one::<String>("chat_display_name")
        .unwrap_or(&chat_display_name_env);
    let filter_start_date = matches
        .get_one("filter_start_date")
        .unwrap_or(&filter_start_date_env);
    let update_interval_s = matches
        .get_one("update_interval_s")
        .unwrap_or(&DEFAULT_UPDATE_INTERVAL_S.to_string())
        .parse::<u64>()
        .expect("Failed to parse update interval");

    let interval = Duration::from_secs(update_interval_s);

    // create a temp file to hold the latest message id
    let tempdir = tempdir().unwrap();
    let latest_message_id_file_path = tempdir.path().join("last-message-id.txt");
    let _ = std::fs::File::create(&latest_message_id_file_path).unwrap();
    loop {
        update_playlist(
            playlist_id_str,
            filter_start_date,
            chat_db_path,
            chat_display_name,
            latest_message_id_file_path.as_path(),
        );
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
