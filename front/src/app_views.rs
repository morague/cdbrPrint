use yew::prelude::*;
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::spawn_local;
use web_sys::{EventTarget, HtmlInputElement, HtmlElement};

use crate::generator::{Parser, Brcdgenerator};
use crate::job::{Job};


#[wasm_bindgen(module = "/static/index.js")]
extern "C" {
    #[wasm_bindgen(js_name = invokePrint, catch)]
    pub async fn printer(buf: String) -> Result<JsValue, JsValue>;
}




pub fn main_view() -> Html {
    let generate = {
        Callback::from(move |e: MouseEvent| {
            spawn_local(async move {
                if let Some(node) = e.target().and_then(|target: EventTarget| {
                    target.dyn_into::<web_sys::Element>().ok()
                }) {
                    let qty_node: HtmlInputElement = node.previous_sibling()
                                            .unwrap()
                                            .dyn_into::<web_sys::HtmlInputElement>()
                                            .unwrap();
    
                    let barcode_node: HtmlInputElement = node.previous_sibling()
                                            .unwrap()
                                            .previous_sibling()
                                            .unwrap()
                                            .dyn_into::<web_sys::HtmlInputElement>()
                                            .unwrap();
                    
                    let status_placeholder: HtmlElement = node.parent_element()
                                                        .unwrap()
                                                        .next_element_sibling()
                                                        .unwrap()
                                                        .first_child()
                                                        .unwrap()
                                                        .dyn_into::<web_sys::HtmlElement>()
                                                        .unwrap();
    
                    let qty: String = qty_node.value();
                    let barcode: String = barcode_node.value();
                    let inputs: Result<(String, String, u8), String> = Parser::new(barcode, qty).parse_inputs();
                    web_sys::console::log_1(&format!("{:?}", inputs).into());

                    match inputs {
                        Ok(s) => {
                            let buf: Vec<u8> = Brcdgenerator::new(s.0, s.1, s.2).generate(80u32);
                            let job = serde_json::to_string(&Job::new(buf, s.2));
                            printer(job.unwrap()).await;
                        },
                        Err(s) => status_placeholder.set_inner_html(&s)
                    };


                    // let printer: Printer = Printer::new(String::from("QL-720NW"));
                    // printer.config_dimension(62u32, 29u32);
                    // let jog: Job =  Job::new(buffer, qty);

                    barcode_node.set_value("");
                    qty_node.set_value("1");
                }       
            });
        })
    };


    html! {
        <section id={"content"}>
            <div id={"barcode-container"}>
                <label id="barcode-label" for="barcode">{"Entrez Votre code barre: "}</label>
                <div id={"inp-container"}>
                    <input type="text" id="barcode" name="barcode" placeholder="Entrez un code-barre" />
                    <input type="number" name="qty" id="qty" value=1 min=0 max=32 placeholder="Combien" />
                    <button id={"generator"} onclick= {generate}>{"Générer"}</button>
                </div>
                <div id="process_status">
                    <p id="status"></p> 
                </div>
            </div>
        </section>
    }
}

