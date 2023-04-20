use super::youtube::scrape_youtube;
use super::youtube::video::Video;
use crate::conf::Configuration;
use crate::controls::messages::Message;
use crate::player_state::PlayerState;
use crate::song::Song;
use reqwest::Client;

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

pub async fn get_youtube_videos(
    conf: &Configuration,
    search: String,
    client: &Client,
) -> Vec<Video> {
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

/// Send a controlling request
pub async fn control(conf: &Configuration, client: &Client, msg: Message) {
    let hurl = conf.host_url();
    let (url, body) = match msg {
        Message::Play => (hurl + "/play", None),
        Message::Pause => (hurl + "/pause", None),
        Message::Skip(n) => (
            hurl + "/skip",
            Some(
                n.iter()
                    .fold("".to_string(), |acc, x| acc + &format!("{x}\r\n")),
            ),
        ),
    };

    let mut c = client.post(url).header("Key", conf.host_key.to_owned());
    c = match body {
        Some(b) => c.body(b),
        None => c,
    };
    c.send().await.unwrap();
}
