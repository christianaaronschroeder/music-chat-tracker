use rspotify::{model::{PlaylistId, TrackId}, prelude::*, scopes, AuthCodeSpotify, Credentials, OAuth};
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

async fn get_playlist_track_ids(spotify: &AuthCodeSpotify, playlist_id: &PlaylistId<'_>) -> HashSet<String> {

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


pub fn track_id_str_to_playable_id(track_id: &str) -> PlayableId {
    PlayableId::Track(TrackId::from_id(track_id).unwrap())
}

async fn add_tracks_to_playlist_if_not_exists(
    spotify: &AuthCodeSpotify,
    playlist_id: &str,
    existing_track_set: HashSet<String>,
    new_track_ids: Vec<String>,
) -> () {
    let tracks_not_in_playlist: Vec<_> = new_track_ids.into_iter().filter(|track_id| !existing_track_set.contains(track_id)).collect();
    if !tracks_not_in_playlist.is_empty() {

        let tracks_to_add = tracks_not_in_playlist.iter().map(|track_id| track_id_str_to_playable_id(track_id)).collect::<Vec<PlayableId>>();

        let playlist_id = PlaylistId::from_id(playlist_id).unwrap();
        spotify.playlist_add_items(playlist_id, tracks_to_add, None).await.unwrap();
    }
}

#[tokio::main]
async fn main() {
    let spotify: AuthCodeSpotify = build_rspotify_client().await;

    let playlist_id_str: &str = "4wN2XkppT6xRzQI5js4nd3";
    let playlist_id: PlaylistId = PlaylistId::from_id(playlist_id_str).unwrap();
    let track_ids: HashSet<String> = get_playlist_track_ids(&spotify, &playlist_id).await;

    println!("Number of tracks: {}", track_ids.len());

    add_tracks_to_playlist_if_not_exists(&spotify, playlist_id_str, track_ids, vec!["5iKndSu1XI74U2OZePzP8L".to_string()]).await;
}
