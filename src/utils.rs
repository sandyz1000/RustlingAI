use gloo_utils::format::JsValueSerdeExt;
use serde::Serialize;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{Blob, BlobPropertyBag, Url};

pub fn get_today() -> String {
    chrono::Local::now().format("%Y-%m-%d").to_string()
}

pub fn download_file(data: &impl Serialize, filename: String) {
    // Serialize data to JSON
    let js_value: JsValue = JsValue::from_serde(&data).unwrap();

    // Create Blob
    let option = BlobPropertyBag::new();
    option.set_type("application/json");
    let blob =
        Blob::new_with_str_sequence_and_options(&js_value, &option).expect("Failed to create blob");

    // Create object URL
    let url = Url::create_object_url_with_blob(&blob).unwrap();

    // Create and click temporary anchor
    let document = gloo_utils::window().document().unwrap();
    let anchor = document
        .create_element("a")
        .unwrap()
        .dyn_into::<web_sys::HtmlAnchorElement>()
        .unwrap();

    anchor.set_href(&url);
    anchor.set_download(&filename);
    anchor.click();

    // Cleanup
    Url::revoke_object_url(&url).unwrap();
}
