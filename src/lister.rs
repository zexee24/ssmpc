use crate::song::Song;
use yew::prelude::*;
use yew::Properties;

#[derive(Properties, PartialEq)]
pub struct PropsSongList {
    pub song_list: Vec<Song>,
    pub on_click: Callback<Song>,
    pub search: AttrValue,
}

#[derive(Properties, PartialEq)]
struct PropsSong {
    song: Song,
    on_click: Callback<Song>,
}

#[function_component]
pub fn SongList(
    PropsSongList {
        song_list,
        on_click,
        search,
    }: &PropsSongList,
) -> Html {
    let search = search.to_string();
    song_list
        .iter()
        .filter(|s| {
            s.name.to_lowercase().contains(&search.to_lowercase())
                || s.artist
                    .clone()
                    .map(|a| a.to_lowercase().contains(&search.to_lowercase()))
                    .unwrap_or(false)
        })
        .map(|s| {
            html! {
                <SongComponent song={s.clone()} on_click={on_click}/>
            }
        })
        .collect()
}

#[function_component]
fn SongComponent(PropsSong { song, on_click }: &PropsSong) -> Html {
    let song = song.clone();
    let on_click = on_click.clone();
    html! {
        <div>
            <h3>{song.name.clone()}</h3>
            <h4>{song.artist.clone().unwrap_or("Unknown artist".to_string())}</h4>
            <button onclick={move |_|{
                on_click.emit(song.clone());
            }}>{"Add"}</button>
        </div>
    }
}
