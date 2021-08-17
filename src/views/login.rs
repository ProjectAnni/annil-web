use yew::prelude::*;

use crate::{
    router::{Auth, Route},
    utils::local_storage,
};

#[function_component(Login)]
pub fn login() -> Html {
    let auth = use_context::<Auth>().expect("No auth found.");
    let oninput = {
        let auth = auth.clone();
        Callback::from(move |msg: InputData| *auth.clone().jwt.borrow_mut() = msg.value)
    };
    let onclick = {
        let auth = auth.clone();
        Callback::from(move |_| {
            let data = auth.jwt.borrow();
            let splitted: Vec<_> = data.split(".").collect();
            if splitted.len() != 3 {
                // invalid jwt
                return;
            }
            let local_storage = local_storage();
            local_storage
                .set_item("auth", &data)
                .expect("failed to set item to localStorage");

            yew_router::push_route(Route::Index);
        })
    };

    if !auth.jwt.borrow().is_empty() {
        yew_router::push_route(Route::Index);
    }

    html! {
        <div class="flex h-40 items-center justify-center flex-row">
            <input type="text" class="form-input px-4 py-3" placeholder="JWT" {oninput} />
            <svg xmlns="http://www.w3.org/2000/svg" class="h-8 w-8 hover:text-green-600 ml-2" fill="none" viewBox="0 0 24 24" stroke="currentColor" {onclick}>
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
            </svg>
        </div>
    }
}
