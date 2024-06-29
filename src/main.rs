use rspotify::{model::PlaylistId, prelude::*, AuthCodeSpotify, Credentials, OAuth, scopes};
use std::collections::HashSet;

async fn build_rspotify_client() -> AuthCodeSpotify {
    let creds = Credentials::from_env().unwrap();
    let oauth = OAuth::from_env(scopes!("user-read-currently-playing playlist-modify-public")).unwrap();

    let spotify = AuthCodeSpotify::new(creds, oauth);

    // Obtaining the access token
    let url = spotify.get_authorize_url(false).unwrap();
    spotify.prompt_for_token(&url).await.unwrap();

    spotify
}

async fn get_playlist_track_ids(spotify: &AuthCodeSpotify, playlist_id: &str) -> HashSet<String> {
    let playlist_id = PlaylistId::from_id(playlist_id).unwrap();

    // we want to use a HashSet because want to store unique ids that we may access frequently
    let mut track_ids = HashSet::new();

    let mut offset = 0;
    let offset_step = 100;
    loop {
        println!("Loading playlist track ids: {}", track_ids.len());
        let playlist = spotify.playlist_items_manual(playlist_id.clone(), None, None, Some(offset_step), Some(offset)).await.unwrap();
        for item in playlist.items {
            if let Some(track) = item.track {
                if let rspotify::model::PlayableItem::Track(track) = track {
                    track_ids.insert(track.id.unwrap().id().to_string());
                }
            }
        }
        if playlist.next.is_none() {
            break;
        }
        offset += offset_step;
    }

    track_ids
}

#[tokio::main]
async fn main() {
    let spotify = build_rspotify_client().await;

    let playlist_id = "7hVMUyFFi6bNtjO4hubtJm"; // Replace with your playlist ID
    let track_ids = get_playlist_track_ids(&spotify, playlist_id).await;

    println!("Number of tracks: {}", track_ids.len());
}
