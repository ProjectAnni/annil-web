use yew::prelude::*;

use super::Route;

#[derive(Clone, Debug, PartialEq)]
pub struct Auth {
    pub jwt: String,
}

#[derive(Properties, Clone, PartialEq)]
pub struct AuthProviderProps {
    /// Children to use context
    pub children: Children,
}

#[function_component(AuthProvider)]
pub fn auth_provider(props: &AuthProviderProps) -> Html {
    let ctx = use_state(|| Auth { jwt: String::new() });

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

    // redirect when jwt is empty
    if auth.jwt.is_empty() {
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
