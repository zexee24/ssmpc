use std::time::Duration;

use super::Psp;
use yew::prelude::*;
#[function_component]
pub fn StatusBar(Psp { player_state }: &Psp) -> Html {
    if player_state.is_none() {
        return html! {"No State"};
    }
    let ps = player_state.as_ref().unwrap();
    match &ps.now_playing {
        Some(song) => {
            html! {
                <div>
                    <p>{format!("Playing {} by {} at {:?}",song.name ,song.artist.clone().unwrap_or("Unknown".to_string()), ps.current_duration.unwrap_or(Duration::from_secs(0)))}</p>
                    <h2>{"Next up:"}</h2>
                    <Queue player_state ={player_state.clone()}/>
                </div>
            }
        }
        None => html! {<p>{"currently playing nothing"}</p>},
    }
}

#[function_component]
pub fn Queue(Psp { player_state }: &Psp) -> Html{
    let name_list: Vec<_> = match player_state{
        Some(p) => p.queue.clone().into(),
        None => vec![],
    };

    html!{
        <div>
            {html!{name_list.iter().fold("".to_owned(), |acc, x| acc + &format!("{} | ", x.name))}}
        </div>
    }
}
