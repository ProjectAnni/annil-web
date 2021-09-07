use crate::router::Route;
use yew::prelude::*;
use yew_router::prelude::Link;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <header class="bg-gray-800 p-4">
            <Link<Route> route={Route::Index}>
                <h1 class="text-xl font-bold text-white">{ "Annil-Web" }</h1>
            </Link<Route>>
        </header>
    }
}
