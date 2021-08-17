use yew::prelude::*;

mod router;
use router::*;

mod components;
mod views;

use components::header::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <Header />
            <Router<Route> render={Router::render(switch)} />
        </>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
