use yew::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <header class="bg-gray-800 p-4">
            <h1 class="text-xl font-bold text-white">{ "Annil-Web" }</h1>
        </header>
    }
}
