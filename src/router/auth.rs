use std::{cell::RefCell, rc::Rc};

use yew::prelude::*;

use super::Route;

pub type Auth = Rc<AuthInner>;

#[derive(Debug, PartialEq)]
pub struct AuthInner {
    pub jwt: RefCell<String>,
}

#[derive(Properties, Clone, PartialEq)]
pub struct AuthProviderProps {
    /// Children to use context
    pub children: Children,
}

#[function_component(AuthProvider)]
pub fn auth_provider(props: &AuthProviderProps) -> Html {
    let window: web_sys::Window = web_sys::window().expect("window not available");
    let local_storage: web_sys::Storage = window
        .local_storage()
        .expect("localStorage not available")
        .expect("None localStorage object got");
    let jwt = local_storage
        .get_item("auth")
        .expect("failed to get item from localStorage")
        .unwrap_or(String::new());
    let ctx = use_state(|| {
        Rc::new(AuthInner {
            jwt: RefCell::new(jwt),
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
