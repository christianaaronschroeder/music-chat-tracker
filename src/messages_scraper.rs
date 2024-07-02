use std::fs;
use std::path::Path;
use std::process::Command;
use log::info;
use chrono::Utc;
use scraper::{Html, Selector};
use regex::Regex;
use tempfile::tempdir;

fn export_chat_messages(
    filter_start_date: &str,
    filter_stop_date: &str,
    output_dir: &Path,
) -> Result<String, Box<dyn std::error::Error>> {
    let script_path = "messages-exporter-copy.php";
    let music_chat_name = "+15404495562, +15405531247, +15405778447, Ideen Ashraf, Jeffrey Smith, Josh Sternfeld, Marshall Hurst, Rustin Ahmadian, Wiatt Bingley";

    let output_template = format!("music_chat_backup_{}_{}", filter_start_date, filter_stop_date);
    let output_dir_str = output_dir.to_str().unwrap();

    let args: [(&str, &str); 5] = [
        ("--output_directory", output_dir_str),
        ("--path-template", &output_template),
        ("--date-start", filter_start_date),
        ("--date-stop", filter_stop_date),
        ("--match", music_chat_name),
    ];

    let mut command = Command::new("php");
    command.arg(script_path);
    for (arg_name, value) in &args {
        command.arg(format!("{}={}", arg_name, value));
    }

    command.output()?;

    let expected_output_path = format!("{}/{}.html", output_dir_str, output_template);
    if !fs::metadata(&expected_output_path).is_ok() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("No file was exported to {}", expected_output_path),
        )));
    }

    info!("Transcript successfully exported to {}", expected_output_path);
    Ok(expected_output_path)
}

pub fn load_chat_export(file_path: &str) -> Html {
    let html_str = fs::read_to_string(file_path)
        .expect("Something went wrong reading the file");
    let soup = Html::parse_document(&html_str);
    info!("Conversation loaded");
    soup
}

pub fn extract_track_ids(soup: &Html) -> Vec<String> {
    let selector = Selector::parse("p.m").unwrap();
    let re = Regex::new(r"^https://open.spotify.com/track").unwrap();
    let mut track_ids = Vec::new();

    for element in soup.select(&selector) {
        if let Some(link) = element.text().next() {
            if re.is_match(link) {
                let parts: Vec<&str> = link.split('/').collect();
                if let Some(track_id) = parts.last() {
                    let id = track_id.split('?').next().unwrap();
                    track_ids.push(id.to_string());
                }
            }
        }
    }

    let mut unique_ids = Vec::new();
    for track_id in track_ids {
        if !unique_ids.contains(&track_id) {
            unique_ids.push(track_id);
        }
    }

    info!("{} Unique IDs found", unique_ids.len());

    unique_ids
}

pub fn get_tracks_from_messages(
    filter_start_date: &str,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let current_date: String = Utc::now().format("%Y-%m-%d").to_string();

    let tmp_dir: tempfile::TempDir = tempdir()?;

    let output_path: String = export_chat_messages(filter_start_date, &current_date, tmp_dir.path()).expect("Failed to export chat messages");
    let soup: Html = load_chat_export(&output_path);
    let track_ids: Vec<String> = extract_track_ids(&soup);

    tmp_dir.close()?;
    
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