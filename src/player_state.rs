use std::{collections::VecDeque, time::Duration};
use serde::Deserialize;
use crate::song::Song;
use crate::conf::Configuration;

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct PlayerState {
    pub now_playing: Option<Song>,
    pub queue: VecDeque<Song>,
    pub volume: f32,
    pub speed: f32,
    pub paused: bool,
    pub total_duration: Option<Duration>,
    pub current_duration: Option<Duration>,
}

impl PlayerState{
    pub async fn get(conf: Configuration)-> Result<Self, String>
    {
        let s = reqwest::get(conf.host_url()+"/").await.map_err(|e| e.to_string())?;
        let state: PlayerState = serde_json::from_str(&s.text().await.map_err(|e| e.to_string())?).map_err(|e| e.to_string())?;
        Ok(state)
    }
}
