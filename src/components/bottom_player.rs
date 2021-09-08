use crate::{components::icons::*, router::Auth};
use yew::prelude::*;

#[derive(Debug, Default, Clone)]
struct MusicIndex {
    catalog: String,
    track_number: u8,
}

#[derive(Debug, Clone)]
struct Playlist(Vec<MusicIndex>);

impl Playlist {
    fn get(&self, index: usize) -> Option<MusicIndex> {
        if index >= self.0.len() || self.0.is_empty() {
            return None;
        }

        Some(self.0[index].clone())
    }

    fn previous_music(&self, index: usize) -> Option<MusicIndex> {
        // at the beginning
        if self.0.is_empty() || index == 0 {
            return None;
        }

        Some(self.0[index - 1].clone())
    }

    fn next_music(&self, index: usize) -> Option<MusicIndex> {
        // at the end
        if index + 1 >= self.0.len() {
            return None;
        }

        Some(self.0[index + 1].clone())
    }
}

impl Default for Playlist {
    fn default() -> Self {
        Self(vec![
            MusicIndex {
                catalog: "KICM-2065".to_string(),
                track_number: 1,
            },
            MusicIndex {
                catalog: "KICM-2065".to_string(),
                track_number: 2,
            },
            MusicIndex {
                catalog: "KICM-2065".to_string(),
                track_number: 3,
            },
        ])
    }
}

#[function_component(BottomPlayer)]
pub fn bottom_player() -> Html {
    let is_playing = use_state(|| false);
    let playing_index = use_ref(|| 0usize);
    let playlist = use_state(|| Playlist::default());
    let auth: Auth = use_context().expect("Failed to get auth in context");

    // Audio player
    let audio_player = NodeRef::default();

    // Progress bars
    let play_bar = NodeRef::default();
    let cache_bar = NodeRef::default();

    let play_icon = {
        let is_playing = is_playing.clone();
        move || -> Html {
            if *is_playing {
                html! { <IconPause /> }
            } else {
                html! { <IconPlay /> }
            }
        }
    };

    let audio_url = use_state(|| String::new());
    {
        let playing_index = playing_index.clone();
        let playlist = playlist.clone();
        let auth = auth.clone();
        let audio_url = audio_url.clone();
        use_effect(move || {
            let current_playing_index = { *playing_index.borrow() };
            if audio_url.is_empty() {
                wasm_bindgen_futures::spawn_local(async move {
                    if let Some(index) = playlist.get(current_playing_index) {
                        match auth.get_music_url(index.catalog, index.track_number).await {
                            Ok(url) => audio_url.set(url),
                            Err(err) => log::error!("{:?}", err),
                        };
                    }
                });
            }
            || ()
        });
    }

    let toggle_playing = {
        let audio_player = audio_player.clone();
        let is_playing = is_playing.clone();
        Callback::from(move |_| {
            if let Some(audio) = audio_player.cast::<web_sys::HtmlAudioElement>() {
                let is_playing = is_playing.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    // toggle music
                    if audio.paused() {
                        let toggle_play = audio.play().expect("Failed to play audio");
                        if let Err(err) = wasm_bindgen_futures::JsFuture::from(toggle_play).await {
                            log::error!("{:?}", err);
                        } else {
                            is_playing.set(true);
                        }
                    } else {
                        audio.pause().expect("Failed to pause audio");
                        is_playing.set(false);
                    }
                });
            } else {
                unreachable!()
            }
        })
    };

    let toggle_music = {
        |offset: i32| {
            let is_playing = is_playing.clone();
            let playlist = playlist.clone();
            let playing_index = playing_index.clone();
            let auth = auth.clone();
            let audio_url = audio_url.clone();
            let audio_player = audio_player.clone();

            let current_playing_index = { *playing_index.borrow() };
            Callback::from(move |_| {
                log::info!("offset = {}", offset);
                let index = if offset > 0 {
                    playlist.next_music(current_playing_index)
                } else {
                    playlist.previous_music(current_playing_index)
                };

                if let Some(index) = index {
                    // change music with offset
                    if offset > 0 {
                        *playing_index.borrow_mut() += 1;
                    } else {
                        *playing_index.borrow_mut() -= 1;
                    }

                    // set play url
                    let is_playing = is_playing.clone();
                    let auth = auth.clone();
                    let audio_url = audio_url.clone();
                    let audio_player = audio_player.clone();
                    let fut = async move {
                        match auth.get_music_url(index.catalog, index.track_number).await {
                            Ok(url) => {
                                audio_url.set(url);

                                // toggle play state if necessary
                                if let Some(audio) =
                                    audio_player.cast::<web_sys::HtmlAudioElement>()
                                {
                                    log::info!("audio paused: {}", audio.paused());
                                    if audio.paused() {
                                        wasm_bindgen_futures::spawn_local(async move {
                                            // toggle music
                                            let toggle_play =
                                                audio.play().expect("Failed to play audio");
                                            if let Err(err) =
                                                wasm_bindgen_futures::JsFuture::from(toggle_play)
                                                    .await
                                            {
                                                log::error!("{:?}", err);
                                            } else {
                                                is_playing.set(true);
                                            }
                                        });
                                    }
                                }
                            }
                            Err(err) => log::error!("{:?}", err),
                        }
                    };
                    wasm_bindgen_futures::spawn_local(fut);
                }
            })
        }
    };

    html! {
        <div class="flex flex-col">
            <div class="flex-grow w-full cursor-pointer -mt-1 -mb-1">
                <div class="h-1 relative origin-left top-1 bg-gray-300"></div>
                <div class="h-1 relative origin-left bg-gray-600" ref={cache_bar}></div>
                <div class="h-1 relative origin-left bottom-1 bg-red-500" ref={play_bar}></div>
            </div>
            <div class="w-full max-w-full p-4 flex items-center">
                <span onclick={toggle_music(-1)}><IconPrev /></span>
                <span onclick={toggle_playing}>{ play_icon() }</span>
                <span onclick={toggle_music(1)}><IconNext /></span>
                <span class="flex-grow"></span>
                <span>{ "End" }</span>
            </div>
            <audio class="hidden" src={audio_url.to_string()} ref={audio_player} />
        </div>
    }
}
