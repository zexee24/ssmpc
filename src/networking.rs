use crate::conf::Configuration;
use crate::player_state::PlayerState;
pub async fn get_state(conf: Configuration) -> Result<PlayerState, String>{
    let raw =reqwest::Client::new().get(conf.host_url() + "/")
        .header("Key", conf.host_key)
        .send()
        .await.map_err(|e| e.to_string()).unwrap().text().await.map_err(|e| e.to_string())?;
    serde_json::from_str(&raw).map_err(|e| e.to_string())
}
