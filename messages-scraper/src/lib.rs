use std::fs;
use std::process::Command;
use log::info;
use chrono::Utc;
use scraper::{Html, Selector};
use regex::Regex;

fn export_chat_messages(
    filter_start_date: &str,
    filter_stop_date: &str,
    output_dir: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let script_path = "messages-exporter-copy.php";
    let music_chat_name = "+15404495562, +15405531247, +15405778447, Ideen Ashraf, Jeffrey Smith, Josh Sternfeld, Marshall Hurst, Rustin Ahmadian, Wiatt Bingley";

    let output_template = format!("music_chat_backup_{}_{}", filter_start_date, filter_stop_date);

    let args = [
        ("--output_directory", output_dir),
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

    let expected_output_path = format!("{}/{}.html", output_dir, output_template);
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
) -> Vec<String>{
    let current_date = Utc::now().format("%Y-%m-%d").to_string();
    let output_path = export_chat_messages(filter_start_date, &current_date, "music_chat_exports").expect("Failed to export chat messages");
    let soup = load_chat_export(&output_path);
    let track_ids = extract_track_ids(&soup);

    track_ids
}