use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use web_sys::{console};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::{JsFuture, spawn_local};
use web_sys::{Request, RequestInit, RequestMode, Response, Headers};
use csv::{Reader};
use js_sys::Uint8Array;

#[derive(Debug, Serialize, Deserialize)]
pub struct Branch {
    pub name: String,
    pub commit: i32,
}

//pub fn topnet() {
//    spawn_local(net1);
//} 

pub async fn net1(val: JsValue) -> Result<JsValue, JsValue> {
    console::log_1(&val);

    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(
        "/static/test-small.csv",
        &opts,
    )?;

    request
        .headers()
        .set("Accept", "test/json")?;

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    // `resp_value` is a `Response` object.
    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();
    console::log_1(&resp);
    // Convert this other `Promise` into a rust `Future`.
    let data = JsFuture::from(resp.text()?).await?;
    //let ary = Uint8Array::from(data);
    //let len = ary.length();
    //let the: &[u8] = &Vec::with_capacity(len as usize);
    //let mut rdr = csv::ReaderBuilder::new().from_reader(the);
    

    // Send the `Branch` struct back to JS as an `Object`.
    Ok(data)

}