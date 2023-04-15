use crate::conf::Configuration;
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

/*pub async fn search_yt(conf: &Configuration, search: String) -> Result<Vec<String>, String>{
    let client = invidious::reqwest::asynchronous::Client::new(conf.invidious_server.clone());
    let res = client.search(Some(&format!("q={}",search))).await.map_err(|e| e.to_string())?;
    Ok(res.items.iter().filter_map(|i| {
        if let SearchItem::Video { title, id, author, author_id, author_url, length, thumbnails, description, description_html, views, published, published_text, live, paid, premium } = i{
           return Some(title.clone())
        }
        None
    }).collect())
}
*/
