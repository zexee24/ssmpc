use::yew::prelude::*;
use super::Psp;
#[function_component]
pub fn StatusBar(Psp{player_state}: &Psp)->Html{
    if player_state.is_none() { 
        return html!{"No State"}
    } 
    let ps = player_state.as_ref().unwrap();
    match &ps.now_playing{
        Some(song) => {
            html!{<p>{format!("Playing {:?} by {:?} at {:?}",song.name ,song.artist, ps.current_duration )}</p>
    }
        },
        None => html!{<p>{"currently playing nothing"}</p>}
    }
    
}

