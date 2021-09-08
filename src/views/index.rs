use crate::components::bottom_player::*;
use yew::prelude::*;

#[function_component(Index)]
pub fn index() -> Html {
    html! {
        <div class="flex-grow flex flex-col justify-between">
            <div>{ "Content" }</div>
            <BottomPlayer />
        </div>
    }
}
