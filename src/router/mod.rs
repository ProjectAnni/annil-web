use yew::prelude::*;
pub use yew_router::prelude::*;

mod auth;
pub use auth::Auth;
use auth::*;

use crate::views::*;

#[derive(Debug, Clone, PartialEq, Routable)]
pub enum Route {
    /// Index route
    #[at("/")]
    Index,

    /// Login route to input JSON-Web-Token for annil server
    #[at("/login")]
    Login,

    /// Play route for audio playback of selected music
    #[at("/play/:catalog/:track_number")]
    Play { catalog: String, track_number: u8 },
}

pub fn switch(routes: &Route) -> Html {
    let inner = match routes {
        Route::Index => html! {
            <AuthValidator to={Route::Login}>
                <Index />
            </AuthValidator>
        },
        Route::Login => html! { <Login /> },
        Route::Play {
            catalog,
            track_number,
        } => html! {
            <AuthValidator to={Route::Login}>
                <Play catalog={catalog.clone()} track_number={*track_number}/>
            </AuthValidator>
        },
    };

    html! {
        <AuthProvider>
            { inner }
        </AuthProvider>
    }
}
