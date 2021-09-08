use super::Route;
use crate::utils::local_storage;
use std::{cell::RefCell, rc::Rc};
use yew::prelude::*;

pub type Auth = Rc<AuthInner>;

#[derive(Debug, PartialEq)]
pub struct AuthInner {
    pub jwt: RefCell<String>,
    pub server: RefCell<String>,
}

impl AuthInner {
    pub async fn get_cover_url(&self, catalog: &str) -> anyhow::Result<String> {
        crate::utils::request_create_object_url(
            &format!("{}{}/cover", self.server.borrow(), catalog),
            self.jwt.borrow().as_str(),
        )
        .await
    }

    pub async fn get_music_url(&self, catalog: &str, track_number: u8) -> anyhow::Result<String> {
        crate::utils::request_create_object_url(
            &format!("{}{}/{}", self.server.borrow(), catalog, track_number),
            self.jwt.borrow().as_str(),
        )
        .await
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct AuthProviderProps {
    /// Children to use context
    pub children: Children,
}

#[function_component(AuthProvider)]
pub fn auth_provider(props: &AuthProviderProps) -> Html {
    let local_storage = local_storage();
    let jwt = local_storage
        .get_item("auth")
        .expect("failed to get auth from localStorage")
        .unwrap_or(String::new());
    let server = local_storage
        .get_item("server")
        .expect("failed to get server from localStorage")
        .unwrap_or(String::new());
    let ctx = use_state(|| {
        Rc::new(AuthInner {
            jwt: RefCell::new(jwt),
            server: RefCell::new(server),
        })
    });

    html! {
        <ContextProvider<Auth> context={(*ctx).clone()}>
            { props.children.clone() }
        </ContextProvider<Auth>>
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct AuthValidatorProps {
    /// Route navigated to when validation not passed
    pub to: Route,
    /// Children rendered when validation passed
    pub children: Children,
}

#[function_component(AuthValidator)]
pub fn auth_validator(props: &AuthValidatorProps) -> Html {
    let auth = use_context::<Auth>().expect("No auth found.");
    log::trace!("JWT: {}", auth.jwt.borrow());

    // redirect when jwt is empty
    if auth.jwt.borrow().is_empty() {
        yew_router::push_route(props.to.clone());
        html! {}
    } else {
        html! {
            <>
                { props.children.clone() }
            </>
        }
    }
}
