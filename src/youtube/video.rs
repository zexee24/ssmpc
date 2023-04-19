use std::time::Duration;

#[derive(Debug, PartialEq, Clone)]
pub struct Video {
    pub title: String,
    pub artist: String,
    pub length: Duration,
    pub id: String,
    pub thumbnail: String,
}
