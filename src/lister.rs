use yew::prelude::*;
use yew::Properties;
use crate::song::Song;
use crate::conf::Configuration;
use reqwest::Client;

#[derive(Properties, PartialEq)]
pub struct PropsSongList{
    pub song_list: Vec<Song>,
    pub on_click: Callback<Song>,
}

#[derive(Properties, PartialEq)]
struct PropsSong{
    song: Song,
    on_click: Callback<Song>,
}



#[function_component]
pub fn SongList(PropsSongList {song_list, on_click}: &PropsSongList)-> Html{
    song_list.iter().map(|s| html!{
        <SongComponent song={s.clone()} on_click={on_click}/>
    }).collect()
}

#[function_component]
fn SongComponent(PropsSong {song, on_click}: &PropsSong) -> Html{
    let song = song.clone();
    let on_click = on_click.clone();
    html!{    
        <div>
            <h2>{song.name.clone()}</h2>
            <p>{song.artist.clone().unwrap_or("Unknown artist".to_string())}</p>
            <button onclick={move |_|{
                on_click.emit(song.clone());
            }}>{"Add"}</button>
        </div>
    }
}
