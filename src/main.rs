use yew::prelude::*;

mod router;
use router::*;

mod views;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[function_component(App)]
fn app() -> Html {
    html! {
        <Router<Route> render={Router::render(switch)} />
    }
}

fn main() {
    yew::start_app::<App>();
}
