

use wasm_bindgen::prelude::*;
use yew::prelude::*;
use crate::app_views::{main_view};


#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}


#[function_component(App)]
pub fn app() -> Html {
    main_view()
}
