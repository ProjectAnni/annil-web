use yew::prelude::*;

use crate::{
    router::{Auth, Route},
    utils::{alert, local_storage},
};

#[function_component(Login)]
pub fn login() -> Html {
    let auth = use_context::<Auth>().expect("No auth found.");
    let on_input_server = {
        let auth = auth.clone();
        Callback::from(move |msg: InputData| *auth.server.borrow_mut() = msg.value)
    };
    let on_input_jwt = {
        let auth = auth.clone();
        Callback::from(move |msg: InputData| *auth.jwt.borrow_mut() = msg.value)
    };
    let onclick = {
        let auth = auth.clone();
        Callback::from(move |_| {
            {
                let local_storage = local_storage();

                let mut server = auth.server.borrow_mut();
                if server.is_empty() {
                    alert("Empty server address");
                    return;
                }
                if !server.ends_with('/') {
                    *server = format!("{}/", server);
                }
                local_storage
                    .set_item("server", &server)
                    .expect("failed to set item to localStorage");

                let jwt = auth.jwt.borrow();
                let splitted: Vec<_> = jwt.split(".").collect();
                if splitted.len() != 3 {
                    // invalid jwt
                    alert("Invalid JWT");
                    return;
                }
                local_storage
                    .set_item("auth", &jwt)
                    .expect("failed to set item to localStorage");
            }

            yew_router::push_route(Route::Index);
        })
    };

    if !auth.jwt.borrow().is_empty() {
        yew_router::push_route(Route::Index);
    }

    html! {
        <div class="h-full flex flex-warp items-center justify-center flex-col gap-y-6">
            <input type="text" class="form-input px-4 py-3" placeholder="Annil Server Address" oninput={on_input_server} />
            <input type="text" class="form-input px-4 py-3" placeholder="JSON Web Token" oninput={on_input_jwt} />
            <svg xmlns="http://www.w3.org/2000/svg" class="h-8 w-8 ml-2 border-2 rounded-full border-green-600 hover:text-white hover:bg-green-600" fill="none" viewBox="0 0 24 24" stroke="currentColor" {onclick}>
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
            </svg>
        </div>
    }
}
