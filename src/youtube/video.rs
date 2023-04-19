use std::time::Duration;

#[derive(Debug, PartialEq, Clone)]
pub struct Video{
    pub title: String,
    pub artist: String,
    pub lenght: Duration,
    pub id: String,
    pub thumbnail: String,
}
