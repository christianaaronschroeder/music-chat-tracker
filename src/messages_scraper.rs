use log::info;
use regex::Regex;
use rusqlite::{Connection, Result};
use std::fs::{read_to_string, OpenOptions};
use std::io::Write;
use std::str;

#[derive(Debug)]
struct Chat {
    id: i32,
}

#[derive(Debug)]
struct Message {
    id: i32,
    text: Option<String>,
    attributed_body: String,
}

const SPOTIFY_TRACK_URL_HEXCODE: &str =
    "68747470733a2f2f6f70656e2e73706f746966792e636f6d2f747261636b";

fn get_chat_id(conn: &Connection, display_name: &str) -> Result<i32> {
    let mut stmt = conn
        .prepare("select * from chat where display_name=? and service_name='iMessage' limit 1")?;
    let chat_iter = stmt.query_map([display_name], |row| Ok(Chat { id: row.get(0)? }))?;

    for chat in chat_iter {
        return Ok(chat.unwrap().id);
    }
    Ok(-1)
}

fn get_messages(
    conn: &Connection,
    chat_id: &i32,
    filter_start_date: &str,
    filter_stop_date: Option<&str>,
) -> Result<Vec<Message>, rusqlite::Error> {
    let filter_stop_date = match filter_stop_date {
        Some(date) => date,
        None => "9999-12-31 23:59:59",
    };

    let query = format!(
        r#"
        SELECT
            message.ROWID,
            message.text,
            hex(message.attributedBody),
            datetime(message.date/1000000000 + strftime('%s', '2001-01-01 00:00:00'), 'unixepoch', 'localtime') AS date_from_nanoseconds,
            datetime(message.date + strftime('%s', '2001-01-01 00:00:00'), 'unixepoch', 'localtime') AS date_from_seconds
        FROM message LEFT JOIN handle ON message.handle_id=handle.ROWID
        WHERE message.ROWID IN (SELECT message_id FROM chat_message_join WHERE chat_id={})
        AND hex(message.attributedBody) LIKE '%{}%'
        AND datetime(message.date / 1000000000 + strftime('%s', '2001-01-01 00:00:00'), 'unixepoch', 'localtime') >= '{}'
        AND datetime(message.date / 1000000000 + strftime('%s', '2001-01-01 00:00:00'), 'unixepoch', 'localtime') <= '{}'
        ORDER BY date_from_nanoseconds ASC;
        "#,
        chat_id, SPOTIFY_TRACK_URL_HEXCODE, filter_start_date, filter_stop_date
    );
    let mut stmt = conn.prepare(&query)?;
    let message_iter = stmt.query_map([], |row| {
        Ok(Message {
            id: row.get(0)?,
            text: row.get(1)?,
            attributed_body: row.get(2)?,
        })
    })?;

    let messages: Result<Vec<Message>, rusqlite::Error> = message_iter.collect();
    messages
}

fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, String> {
    let hex = hex.trim();
    if hex.len() % 2 != 0 {
        return Err(String::from("Hex string has an odd length"));
    }

    let bytes: Result<Vec<u8>, _> = (0..hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).map_err(|e| e.to_string()))
        .collect();

    bytes
}

fn hex_str_to_str(hex_string: &str) -> Result<String, String> {
    match hex_to_bytes(&hex_string) {
        Ok(bytes) => Ok(String::from_utf8_lossy(&bytes).to_string()),
        Err(e) => Err(e),
    }
}

fn extract_track_ids_from_message(input: &str) -> Vec<String> {
    let re = Regex::new(r"https://open\.spotify\.com/track/([a-zA-Z0-9]+)").unwrap();
    let mut track_ids = Vec::new();
    for cap in re.captures_iter(input) {
        if let Some(track_id) = cap.get(1) {
            // We only want the valid track IDs that are 22 characters long,
            // because the attributeBody we are pulling from has a bunch of noise
            if track_id.len() != 22 {
                continue;
            }
            track_ids.push(track_id.as_str().to_string());
        }
    }

    track_ids
}

fn extract_track_ids(messages: Vec<Message>) -> Vec<String> {
    let mut track_ids: Vec<String> = Vec::new();

    for message in messages {
        let converted_string = hex_str_to_str(&message.attributed_body).unwrap();
        let ids = extract_track_ids_from_message(&converted_string);
        track_ids.extend(ids);
    }

    // we do this instead of using a HashSet to maintain chronological order
    let mut unique_ids: Vec<String> = Vec::new();
    for track_id in track_ids {
        if !unique_ids.contains(&track_id) {
            unique_ids.push(track_id);
        }
    }

    unique_ids
}

pub fn get_tracks_from_messages(
    chat_db_path: &str,
    chat_display_name: &str,
    filter_start_date: &str,
    filter_stop_date: Option<&str>,
    latest_message_id_file_path: &std::path::Path,
) -> Result<Vec<String>> {
    let conn = Connection::open(chat_db_path)?;

    info!("path: {:?}", latest_message_id_file_path);

    let chat_id = get_chat_id(&conn, chat_display_name)?;
    info!("Chat ID: {}", chat_id);

    let messages = get_messages(&conn, &chat_id, &filter_start_date, filter_stop_date)?;
    info!("Messages: {:?}", messages.len());

    // Check if the latest message is the same as last time the script was run
    let latest_message_id_str = messages.last().unwrap().id.to_string();
    let last_message_id_str = read_to_string(latest_message_id_file_path).unwrap();
    info!("Latest message ID: {}", latest_message_id_str);
    info!("Last message ID: {}", last_message_id_str);

    // if the latest message is the same as the last message, we know we don't need to check for new messages
    let track_ids = if latest_message_id_str != last_message_id_str {
        extract_track_ids(messages)
    } else {
        Vec::new()
    };
    info!("Track IDs: {:?}", track_ids.len());

    // write the latest message id to the file
    let mut last_message_file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(latest_message_id_file_path)
        .unwrap();
    let _ = last_message_file.write_all(latest_message_id_str.as_bytes());

    Ok(track_ids)
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
