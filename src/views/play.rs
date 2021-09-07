use yew::prelude::*;

use crate::components::icons::*;
use crate::{router::Auth, utils::request_create_object_url};

#[derive(Properties, Clone, PartialEq)]
pub struct PlayProps {
    pub catalog: String,
    pub track_number: u8,
}

#[function_component(Play)]
pub fn play(props: &PlayProps) -> Html {
    let auth = use_context::<Auth>().expect("Failed to get auth in context");
    let base = format!("{}{}", auth.server.borrow(), props.catalog);
    let jwt = auth.jwt.borrow().clone();

    let cover = use_state(|| String::new());
    {
        let cover = cover.clone();
        let base = base.clone();
        let jwt = jwt.clone();
        use_effect(move || {
            if cover.is_empty() {
                let cover_url = format!("{}/cover", base);
                wasm_bindgen_futures::spawn_local(async move {
                    match request_create_object_url(&cover_url, &jwt).await {
                        Ok(url) => cover.set(url),
                        Err(err) => log::error!("{:?}", err),
                    };
                });
            }
            || ()
        });
    }

    let audio = use_state(|| String::new());
    {
        let audio = audio.clone();
        let track_number = props.track_number;
        use_effect(move || {
            if audio.is_empty() {
                let audio_url = format!("{}/{}", base, track_number);
                wasm_bindgen_futures::spawn_local(async move {
                    match request_create_object_url(&audio_url, &jwt).await {
                        Ok(url) => audio.set(url),
                        Err(err) => log::error!("{:?}", err),
                    };
                });
            }
            || ()
        })
    }

    let audio_player = NodeRef::default();
    let on_play_click = {
        let audio_player = audio_player.clone();
        Callback::from(move |_| {
            if let Some(audio) = audio_player.cast::<web_sys::HtmlAudioElement>() {
                wasm_bindgen_futures::spawn_local(async move {
                    // toggle music
                    if audio.paused() {
                        let toggle_play = audio.play().expect("Failed to play audio");
                        if let Err(err) = wasm_bindgen_futures::JsFuture::from(toggle_play).await {
                            log::error!("{:?}", err);
                        }
                    } else {
                        audio.pause().expect("Failed to pause audio")
                    }

                    // change icon
                });
            } else {
                unreachable!()
            }
        })
    };

    html! {
        <div class="w-full flex flex-grow items-center justify-center">
            <div class="flex items-center justify-center">
              <audio class="hidden" src={audio.to_string()} ref={audio_player}></audio>
              <div class="bg-white shadow-lg rounded-lg w-180">
                <div class="flex">
                  <div class="p-8">
                    <img
                      class="w-full rounded hidden md:block min-h-96 max-h-96 min-w-96 max-w-96"
                      src={cover.to_string()}
                      alt="Album Pic"
                    />
                  </div>
                  <div class="flex flex-col justify-between w-full p-8">
                    <div class="flex justify-between">
                      <div>
                        <h3 class="text-2xl text-grey-darkest font-medium">
                          {props.catalog.clone()}
                        </h3>
                        <p class="text-sm text-grey mt-1">{props.track_number}</p>
                      </div>
                      <div>
                        <IconHeart />
                      </div>
                    </div>
                    <div class="flex justify-between items-center mt-8">
                      <div class="text-grey-darker">
                        <IconRandom />
                      </div>
                      <div class="text-grey-darker">
                        <IconPrev />
                      </div>
                      <div class="text-white p-8 rounded-full bg-red-500 shadow-lg" onclick={on_play_click}>
                        <IconPause />
                      </div>
                      <div class="text-grey-darker">
                        <IconNext />
                      </div>
                      <div class="text-grey-darker">
                        <IconLoop />
                      </div>
                    </div>
                  </div>
                </div>
              </div>
            </div>
        </div>
    }
}
