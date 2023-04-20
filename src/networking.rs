use crate::conf::Configuration;
use crate::player_state::PlayerState;
use crate::song::Song;
use reqwest::Client;
use super::youtube::video::Video;
use super::youtube::scrape_youtube;

pub async fn get_state(conf: &Configuration, client: &Client) -> Result<PlayerState, String> {
    let raw = client
        .get(conf.host_url() + "/")
        .header("Key", conf.host_key.to_owned())
        .send()
        .await
        .map_err(|e| e.to_string())?
        .text()
        .await
        .map_err(|e| e.to_string())?;
    serde_json::from_str(&raw).map_err(|e| e.to_string())
}

pub async fn get_song_list(conf: &Configuration, client: &Client) -> Result<Vec<Song>, String> {
    let raw = client
        .get(conf.host_url() + "/list")
        .header("Key", conf.host_key.to_owned())
        .send()
        .await
        .map_err(|e| e.to_string())?
        .text()
        .await
        .map_err(|e| e.to_string())?;
    serde_json::from_str(&raw).map_err(|e| e.to_string())
}

pub async fn add_song_to_queue(conf: Configuration, client: &Client, songs: Vec<Song>) {
    client
        .post(conf.host_url() + "/add")
        .body(
            songs
                .iter()
                .fold(String::new(), |acc, s| acc + &s.name + "\r\n"),
        )
        .header("Key", conf.host_key.to_owned())
        .send()
        .await
        .unwrap();
}

pub async fn get_youtube_videos(conf: &Configuration, search: String, client: &Client) -> Vec<Video>{
    scrape_youtube(&search, client, conf).await.unwrap()
}

/// Send a request to the backend to download a certain url
pub async fn download_video(conf: &Configuration, client: &Client, url: String) {
    client
        .post(conf.host_url() + "/download")
        .header("Key", conf.host_key.to_owned())
        .body(url)
        .send()
        .await
        .unwrap();
}
