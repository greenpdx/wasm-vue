extern crate csv; 
extern crate serde_json;
mod utils;
pub mod netget;
pub mod budget;
pub mod canvas3d;

use serde::{ Deserialize, Serialize};

use serde_json::{Value};
use wasm_bindgen::prelude::*;
use web_sys::{console};
use netget::net1;
use csv::{ReaderBuilder};
use js_sys::{Array, Uint8Array};
use budget::{rtn_budget, JsBudget, load_csv, fetch_csv, get_tree, JsNode, T1};
use budget::nodedata::*;

use std::mem::drop;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, t3!");
}

#[wasm_bindgen]
pub struct NodesPtr {
    #[wasm_bindgen(skip)]
    pub nodes: Vec<Node>,
    #[wasm_bindgen(skip)]
    pub bdgt: Budget,
}

#[wasm_bindgen]
pub fn init_app(bdgt: JsValue) -> Result<NodesPtr, JsValue> {
    let s: JsBudget = bdgt.into_serde().unwrap();
    let b =  JsBudget::to_budget(&s);
    let t = get_tree(s);
    let mut tre: Vec::<Node> = t; 
    Ok(NodesPtr{
        nodes: tre,
        bdgt: b
    })
}

#[wasm_bindgen]
pub fn jsnodes(nodes_ctx: &mut NodesPtr) -> Result<JsValue, JsValue> {
    let jstre: Vec<JsNode> = nodes_ctx.nodes.iter().map(|n| {JsNode::from_node(n)}).collect();
    let mut tre: Vec::<JsNode> = jstre;  
    Ok(JsValue::from_serde(&tre).unwrap())
}

#[wasm_bindgen]
pub fn getvals(nodes_ctx: &mut NodesPtr) -> Box<[f64]> {
    //let n = nodes_ctx.nodes.;
    nodes_ctx.nodes.iter().map(|n| {n.val as f64}).collect::<Vec<f64>>().into_boxed_slice()
}

#[wasm_bindgen]
pub fn chgval(nodes_ctx: &mut NodesPtr, idx:i32, dif: f64) -> Box<[f64]> {
    //let n = nodes_ctx.nodes.;
    let mut nodes = nodes_ctx.nodes.as_mut();

    let out = chg_node(nodes,idx, dif);
    nodes_ctx.nodes.iter().map(|n| {n.val as f64}).collect::<Vec<f64>>().into_boxed_slice()
}


#[wasm_bindgen]
pub fn free_nodes(nodes_ctx: &mut NodesPtr) {
    drop(nodes_ctx);
}



#[wasm_bindgen]
pub async fn net2(val: JsValue) -> Result<JsValue, JsValue> {
//    let rtn = netget::net1(val).await?;

//    let ary = Uint8Array::from(rtn);
//    let the: &[u8] = &ary.to_vec();
//    println!("{:?}",the);
//    let bdgt = rtn_budget(the).unwrap();
    let s = val.as_string().unwrap();
    let bdgt = load_csv(s).await?;

    Ok(bdgt)
}

#[wasm_bindgen]
pub async fn net3(bdgt: JsValue, filter: JsValue) -> Result<JsValue, JsValue> {
    let s: JsBudget = bdgt.into_serde().unwrap();
    console::log_1(&filter);
    let x: T1 = filter.into_serde().unwrap();
    //let f: String = filter.as_string().unwrap();
    let tre = get_tree(s);
    //console::log_1(&"Test tree".into());
    let jstre: Vec<JsNode> = tre.iter().map(|n| {JsNode::from_node(n)}).collect();

    let mut tre: Vec::<JsNode> = jstre;  
    Ok(JsValue::from_serde(&tre).unwrap())
}

#[wasm_bindgen]
pub fn tryit(s: JsValue) -> Result<JsValue, JsValue> {
    console::log_1(&s);  // &"Test".into());
    Ok(s)
}

#[wasm_bindgen]
pub fn wlog() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    //let tst = global();
    console::log_1(&"Test".into());
    // Your code goes here!
    console::log_1(&JsValue::from_str("Hello world!"));

    Ok(())
}
