use std::convert::TryInto;

use reqwasm::http::Request;
use wasm_bindgen_futures::JsFuture;
use yew::web_sys::{Blob, RequestMode, Storage, Url};

pub async fn request_create_object_url(url: &str, authorization: &str) -> anyhow::Result<String> {
    let response = Request::get(url)
        .mode(RequestMode::Cors)
        .header("Authorization", authorization)
        .send()
        .await?;
    let blob = response
        .as_raw()
        .blob()
        .map_err(|err| anyhow::anyhow!("failed to get blob from response: {:?}", err))?;
    let blob = JsFuture::from(blob)
        .await
        .map_err(|err| anyhow::anyhow!("Promise of response.blob() rejected: {:?}", err))?;
    let blob: Blob = blob.try_into()?;
    let url = Url::create_object_url_with_blob(&blob)
        .map_err(|err| anyhow::anyhow!("failed to createObjectUrl with blob: {:?}", err))?;
    Ok(url)
}

pub fn local_storage() -> Storage {
    yew::utils::window()
        .local_storage()
        .expect("localStorage not available")
        .expect("None localStorage object got")
}

pub fn alert(msg: &str) {
    yew::utils::window()
        .alert_with_message(msg)
        .expect("failed to alert");
}
