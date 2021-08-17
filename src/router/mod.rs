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
    #[at("/play/:catalog/:track")]
    Play { catalog: String, track: u8 },
}

pub fn switch(routes: &Route) -> Html {
    let inner = match routes {
        Route::Index => html! {
            <AuthValidator to={Route::Login}>
                <Index />
            </AuthValidator>
        },
        Route::Login => html! { <Login /> },
        Route::Play { catalog, track } => html! {
            <AuthValidator to={Route::Login}>
                // TODO: Play view
                <h1> { catalog } </h1>
                <span> { track } </span>
            </AuthValidator>
        },
    };

    html! {
        <AuthProvider>
            { inner }
        </AuthProvider>
    }
}
