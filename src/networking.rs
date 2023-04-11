use crate::conf::Configuration;
use crate::player_state::PlayerState;
use crate::song::Song;
use reqwest::Client;

pub async fn get_state(conf: &Configuration, client: &Client) -> Result<PlayerState, String>{
    let raw = client.get(conf.host_url() + "/")
        .header("Key", conf.host_key.to_owned())
        .send()
        .await.map_err(|e| e.to_string()).unwrap().text().await.map_err(|e| e.to_string())?;
    serde_json::from_str(&raw).map_err(|e| e.to_string())
}

pub async fn get_song_list(conf : &Configuration, client: &Client) -> Result<Vec<Song>, String> {
    let raw = client.get(conf.host_url() + "/list")
        .header("Key", conf.host_key.to_owned())
        .send()
        .await
        .map_err(|e| e.to_string()).unwrap().text().await.map_err(|e| e.to_string())?;
    serde_json::from_str(&raw).map_err(|e| e.to_string())
}
