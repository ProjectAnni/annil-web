use yew::prelude::*;

// https://www.zondicons.com/

#[function_component(IconPlay)]
pub fn icon_play() -> Html {
    html! {
      <svg
        class="w-8 h-8"
        fill="currentColor"
        xmlns="http://www.w3.org/2000/svg"
        viewBox="0 0 20 20"
      >
        <path d="M4 4l12 6-12 6z" />
      </svg>
    }
}

#[function_component(IconPause)]
pub fn icon_pause() -> Html {
    html! {
      <svg
         class="w-8 h-8"
         fill="currentColor"
         xmlns="http://www.w3.org/2000/svg"
         viewBox="0 0 20 20"
      >
        <path d="M5 4h3v12H5V4zm7 0h3v12h-3V4z" />
      </svg>
    }
}

#[function_component(IconPrev)]
pub fn icon_prev() -> Html {
    html! {
      <svg
        class="w-8 h-8"
        fill="currentColor"
        xmlns="http://www.w3.org/2000/svg"
        viewBox="0 0 20 20"
      >
        <path d="M4 5h3v10H4V5zm12 0v10l-9-5 9-5z" />
      </svg>
    }
}

#[function_component(IconNext)]
pub fn icon_next() -> Html {
    html! {
      <svg
        class="w-8 h-8"
        fill="currentColor"
        xmlns="http://www.w3.org/2000/svg"
        viewBox="0 0 20 20"
      >
        <path d="M13 5h3v10h-3V5zM4 5l9 5-9 5V5z" />
      </svg>
    }
}

#[function_component(IconLoop)]
pub fn icon_loop() -> Html {
    html! {
      <svg
        class="w-8 h-8"
        fill="currentColor"
        xmlns="http://www.w3.org/2000/svg"
        viewBox="0 0 20 20"
      >
        <path
          d="M5 4a2 2 0 0 0-2 2v6H0l4 4 4-4H5V6h7l2-2H5zm10 4h-3l4-4 4 4h-3v6a2 2 0 0 1-2 2H6l2-2h7V8z"
        />
      </svg>
    }
}

#[function_component(IconRandom)]
pub fn icon_random() -> Html {
    html! {
      <svg
        class="w-8 h-8"
        fill="currentColor"
        xmlns="http://www.w3.org/2000/svg"
        viewBox="0 0 20 20"
      >
        <path
          d="M6.59 12.83L4.4 15c-.58.58-1.59 1-2.4 1H0v-2h2c.29 0 .8-.2 1-.41l2.17-2.18 1.42 1.42zM16 4V1l4 4-4 4V6h-2c-.29 0-.8.2-1 .41l-2.17 2.18L9.4 7.17 11.6 5c.58-.58 1.59-1 2.41-1h2zm0 10v-3l4 4-4 4v-3h-2c-.82 0-1.83-.42-2.41-1l-8.6-8.59C2.8 6.21 2.3 6 2 6H0V4h2c.82 0 1.83.42 2.41 1l8.6 8.59c.2.2.7.41.99.41h2z"
        />
      </svg>
    }
}

#[function_component(IconHeart)]
pub fn icon_heart() -> Html {
    html! {
      <svg
        class="w-6 h-6"
        fill="currentColor"
        xmlns="http://www.w3.org/2000/svg"
        viewBox="0 0 20 20"
      >
        <path
          d="M10 3.22l-.61-.6a5.5 5.5 0 0 0-7.78 7.77L10 18.78l8.39-8.4a5.5 5.5 0 0 0-7.78-7.77l-.61.61z"
        />
    </svg>
    }
}
