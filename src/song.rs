use serde::Deserialize;
use yew::Properties;
#[derive(Clone, Debug, Deserialize, PartialEq, Properties)]
pub struct Song {
    pub name: String,
    pub artist: Option<String>,
    pub url: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SongWithImage {
    pub song: Song,
    //Encoded image with base64
    pub image: Option<String>,
}
