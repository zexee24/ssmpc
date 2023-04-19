use std::time::Duration;

use percent_encoding::{AsciiSet, CONTROLS, percent_encode};
use regex::Regex;
use serde_json::Value;
use video::Video;

pub mod results;
mod video;

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');
// Copied the original version of this function from https://dipack.dev/you-tube-scraper-for-your-spotify-library-in-rust
async fn scrape_youtube(
    query: &str,
    client: &reqwest::Client,
) -> Result<Vec<Video>, Box<dyn std::error::Error>> {
    let res = client
        .get(format!(
            "https://www.youtube.com/results?search_query={}",
            percent_encode(query.as_bytes(), FRAGMENT)
        ))
        .send().await?;
    let search_results_html = res.text().await.unwrap();
    // Copied this regex from this brilliant library:
    // https://github.com/HermanFassett/youtube-scrape/blob/master/scraper.js#L44
    let re = Regex::new("ytInitialData = [^{]*(.*?); *</script>").unwrap();
    let cap = re.captures_iter(&search_results_html).next().ok_or("Failed to find regex")?;
    let json_str = cap[0].replace("ytInitialData = ", "").replace(";</script>", "");
    let json: serde_json::Value = serde_json::from_str(&json_str).unwrap();
    let sc = json.get("contents").ok_or("Failed to find field")?
            .get("twoColumnSearchResultsRenderer").ok_or("Failed to find field")?
            .get("primaryContents").ok_or("Failed to find field")?
            .get("sectionListRenderer").ok_or("Failed to find field")?
            .get("contents").ok_or("Failed to find field")?[0]
            .get("itemSectionRenderer").ok_or("Failed to find field")?
            .get("contents").ok_or("Failed to find field")?.as_array().ok_or("Failed to make array")?;
    Ok(sc.iter().filter_map(|i| {
        let base = i.get("videoRenderer")?;
        parse_video(base)
    }).collect())
}

fn parse_video(json: &Value) -> Option<Video>{

    let title: String = json.get("title")?.get("runs")?[0].get("text")?.as_str()?.to_string();
    let artist: String = json.get("ownerText")?.get("runs")?[0].get("text")?.as_str()?.to_string();
    let lenght_raw: String = json.get("lengthText")?.get("simpleText")?.as_str()?.to_string();
    let lenght = parse_duration(lenght_raw).unwrap();
    let id = json.get("videoId")?.as_str()?.to_string();
    let thumbnail = json.get("thumbnail")?.get("thumbnails")?[0].get("url")?.as_str()?.to_string();
    Some(Video{ 
        title, 
        artist, 
        lenght, 
        id, 
        thumbnail,
    })
}

///Returns the duration of a string with format of: [hh.mm.ss] with some possibly missing
fn parse_duration(s: String) -> Option<Duration>{
    let secs = s.split('.').fold(0, |acc,x| {
        acc + 60*x.parse::<u64>().unwrap()
    });
    Some(Duration::from_secs(secs))
}

#[cfg(test)]
mod tests{
    use reqwest::Client;
    use super::*;

    #[tokio::test]
    async fn test_scuffed_yt_responce(){
        let client = Client::new();
        let query = "Metallica Lux Aeterna";

        let responce = scrape_youtube(query, &client).await;
        assert_eq!(responce.unwrap().first().unwrap().title, "Metallica: Lux Æterna (Official Music Video)".to_string())
    }
}
