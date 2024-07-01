from bs4 import BeautifulSoup
import re
import os
import spotipy
from spotipy.oauth2 import SpotifyOAuth
from datetime import datetime
import logging
import subprocess
from dotenv import load_dotenv

logger = logging.getLogger()
logger.setLevel(logging.INFO)

SCRIPT_PATH = "messages-exporter-copy.php"
# this is the name of the music chat in MY local chat database. Passing in the given chat name, "Music (A Little Spam)" will not work. Idk why some are contacts and some aren't
MUSIC_CHAT_NAME = "+15404495562, +15405531247, +15405778447, Ideen Ashraf, Jeffrey Smith, Josh Sternfeld, Marshall Hurst, Rustin Ahmadian, Wiatt Bingley"


def export_chat_messages(
    filter_start_date: str,
    filter_stop_date: str,
    output_dir: str = "music_chat_exports",
    music_chat_name: str = MUSIC_CHAT_NAME,
    script_path: str = SCRIPT_PATH,
) -> str:  
    # Define the arguments in a dictionary
    args = {
        "--output_directory": output_dir,
        "--path-template": f"music_chat_backup_RTEST_{filter_start_date}_{filter_stop_date if filter_stop_date else datetime.today().strftime('%Y-%m-%d')}",
        "--date-start": filter_start_date,
        "--date-stop": filter_stop_date,
        "--match": music_chat_name,
    }

    # Construct the command string
    command = f"php {script_path}"
    for arg_name,value in args.items():
        if value is not None:
            command += f' {arg_name}="{value}"'
            
    # call the command
    subprocess.run(command, shell=True)

    # double check the file was created
    expected_output_path = os.path.join(args["--output_directory"], args["--path-template"] + ".html")
    assert os.path.isfile(expected_output_path), FileNotFoundError(f"No file was exported to {expected_output_path}")
    logger.info(f"Transcript successfully exported to {expected_output_path}")

    return expected_output_path

def load_chat_export(file_path: str) -> BeautifulSoup:
    with open(file_path, 'r') as file:
        html_str = file.read()
    soup = BeautifulSoup(html_str, 'html.parser')
    logging.info(f"Conversation loaded: {soup.title.text}")
    return soup

def extract_track_ids(soup: BeautifulSoup) -> list:
    # Find all <p> tags with class 'm' that contain links starting with 'https://open.spotify/track'
    html_links = soup.find_all('p', class_='m', string=re.compile(r'^https://open.spotify.com/track'))
    spotify_track_urls = [link.text.strip() for link in html_links]
    logging.info(f"{len(spotify_track_urls)} Spotify track urls found")

    track_ids = [url.split('/')[-1].split('?')[0] for url in spotify_track_urls]


    # Filter to only unique track ids
    # We don't use a set here so that we can maintain the chronological order that the tracks were sent in
    unique_ids = []
    for track_id in track_ids:
        if track_id not in unique_ids:
            unique_ids.append(track_id)

    logging.info(f"{len(unique_ids)} Unique IDs found")

    return unique_ids

def build_spotify_client() -> spotipy.Spotify:
    load_dotenv()
    # Get the client id and secret from the environment
    client_id = os.environ.get('RSPOTIFY_CLIENT_ID')
    client_secret = os.environ.get('RSPOTIFY_CLIENT_SECRET')
    redirect_uri = os.environ.get('RSPOTIFY_REDIRECT_URI')

    # Authentication flow with Spotify
    sp = spotipy.Spotify(
        auth_manager=SpotifyOAuth(
            client_id=client_id,
            client_secret=client_secret,
            redirect_uri=redirect_uri, # This should match the redirect URI in your Spotify Developer Dashboard
            scope='playlist-modify-public'
        )
    )

    return sp

def load_tracks_in_playlist(playlist_id: str, sp: spotipy.Spotify) -> list:
    existing_track_ids = []
    limit = 100 # can't go higher
    offset_mult = 0 # offset multiplier
    while True:
        offset = offset_mult*100
        logging.info(f"loading tracks {offset} to {offset+limit} from playlist {playlist_id}")
        existing_tracks_chunk = sp.playlist_items(playlist_id, fields=["items"], limit=limit, offset=offset)["items"]
        logging.info(f"{len(existing_tracks_chunk)} tracks loaded")

        # if no tracks left, stop loading them
        if len(existing_tracks_chunk) == 0:
            break

        for track in existing_tracks_chunk:
            try:
                id = track["track"]["id"]
                existing_track_ids.append(id)
            except Exception as e:
                logging.error(f"Failed to get track ID from: {track}")

        # if the there were less than 100 tracks loaded, don't bother trying again
        if len(existing_tracks_chunk) < limit:
            break

        offset_mult+=1

    logging.info(f"{len(existing_track_ids)} tracks currently in playlist {playlist_id}")
    return existing_track_ids

def chunk_list(_list, size):
    for i in range(0, len(_list), size):  
        yield _list[i:i + size] 

def add_tracks_to_playlist(playlist_id: str, track_uris_to_add: list, sp: spotipy.Spotify, last_updated_date: str):
    # Need to chunk the uris because we can only add 100 at a time
    chunked = list(chunk_list(track_uris_to_add, 100))
    logging.info(f"{len(chunked)} chunks made")
    # Add tracks to the playlist
    for chunk in chunked:
        sp.playlist_add_items(playlist_id, chunk)
        logging.info(f"Tried to add {len(chunk)} tracks to playlist {playlist_id}")

    # update the playlist details
    new_description = f'All songs sent in the "Music (A Little Spam)" group chat since I was added. Last updated on {last_updated_date}.'
    sp.playlist_change_details(playlist_id, description=new_description)

    print(f'You can view your playlist here: https://open.spotify.com/playlist/{playlist_id}')

if __name__ == "__main__":
    playlist_id = "3op9QLxlW6byL2uJEMLyIC"
    current_date = datetime.today().strftime('%Y-%m-%d')
    output_path = export_chat_messages(
        filter_start_date="2024-06-20",
        filter_stop_date=current_date,
    )
    soup = load_chat_export(output_path)
    track_ids = extract_track_ids(soup)
    sp = build_spotify_client()
    existing_track_ids = load_tracks_in_playlist(playlist_id, sp)

    # Get list of new tracks to add
    track_ids_not_in_playlist = [track_id for track_id in track_ids if track_id not in existing_track_ids]
    logging.info(f"{len(track_ids_not_in_playlist)} tracks found that are not in the playlist")

    # Create URIs because that is what the API expects
    track_uris_to_add = [f'spotify:track:{track_id}' for track_id in track_ids_not_in_playlist]

    add_tracks_to_playlist(playlist_id, track_uris_to_add, sp, current_date)

    logging.info(f'You can view your playlist here: https://open.spotify.com/playlist/{playlist_id}')