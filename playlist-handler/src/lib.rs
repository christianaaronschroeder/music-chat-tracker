use rspotify::{
    model::{PlaylistId, TrackId, PlaylistItem, Page},
    prelude::*,
    scopes,
    AuthCodeSpotify,
    Credentials,
    OAuth,
    Config,
};
use std::collections::HashSet;
use log::{error, info};

async fn build_rspotify_client() -> AuthCodeSpotify {
    info!("Building Spotify client...");

    let creds = Credentials::from_env().unwrap();
    let oauth = OAuth::from_env(scopes!("user-read-currently-playing playlist-modify-public")).unwrap();
    let config = Config::default();
    let spotify = AuthCodeSpotify::with_config(creds, oauth, config);

    // Obtaining the access token
    let url: String = spotify.get_authorize_url(false).unwrap();
    spotify.prompt_for_token(&url).await.unwrap();

    spotify
}

async fn get_playlist_track_ids(spotify: &AuthCodeSpotify, playlist_id: &PlaylistId<'_>) -> HashSet<String> {
    let mut track_ids: HashSet<String> = HashSet::new();

    let mut offset: u32 = 0;
    let offset_step: u32 = 100;
    loop {
        info!("Loading playlist track ids: {}", track_ids.len());
        let playlist: Page<PlaylistItem> = spotify
            .playlist_items_manual(playlist_id.clone(), None, None, Some(offset_step), Some(offset))
            .await
            .unwrap_or_else(|e: rspotify::ClientError| {
                error!("Error loading playlist items: {}", e);
                panic!("Failed to load playlist items");
            });

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
) {
    let tracks_not_in_playlist: Vec<_> = new_track_ids
        .into_iter()
        .filter(|track_id: &String| !existing_track_set.contains(track_id))
        .collect();

    if !tracks_not_in_playlist.is_empty() {
        let tracks_to_add: Vec<PlayableId> = tracks_not_in_playlist
            .iter()
            .map(|track_id: &String| track_id_str_to_playable_id(track_id))
            .collect::<Vec<PlayableId>>();

        let playlist_id = PlaylistId::from_id(playlist_id).unwrap();
        spotify
            .playlist_add_items(playlist_id, tracks_to_add, None)
            .await
            .unwrap_or_else(|e: rspotify::ClientError| {
                error!("Error adding tracks to playlist: {}", e);
                panic!("Failed to add tracks to playlist");
            });
    }
}

#[tokio::main]
pub async fn add_tracks_to_playlist(
    playlist_id_str: &str,
    track_ids_to_add: Vec<String>,
) {

    let spotify: AuthCodeSpotify = build_rspotify_client().await;

    let playlist_id: PlaylistId = PlaylistId::from_id(playlist_id_str).unwrap();
    let existing_track_ids: HashSet<String> = get_playlist_track_ids(&spotify, &playlist_id).await;

    info!("Number of existing tracks: {}", existing_track_ids.len());

    add_tracks_to_playlist_if_not_exists(&spotify, playlist_id_str, existing_track_ids, track_ids_to_add).await;

    info!("View the playlist at: https://open.spotify.com/playlist/{}", playlist_id_str);
}
