use yew::prelude::*;
use yew::Properties;
use crate::song::Song;

#[derive(Properties, PartialEq)]
pub struct PropsSongList{
    pub song_list: Vec<Song>
}

#[derive(Properties, PartialEq)]
struct PropsSong{
    song: Song
}

#[function_component]
pub fn SongList(PropsSongList {song_list}: &PropsSongList)-> Html{
    song_list.iter().map(|s| html!{
        <SongComponent song={s.clone()}/>
    }).collect()
}

#[function_component]
fn SongComponent(PropsSong {song}: &PropsSong) -> Html{
    html!{    
        <div>
            <h2>{song.name.clone()}</h2>
            <p>{song.artist.clone().unwrap_or("Unknown artist".to_string())}</p>
        </div>
    }
}
