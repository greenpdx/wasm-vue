#![feature(result_map_or_else)]
use wasm_bindgen::prelude::*;
use serde::{ Deserialize, Serialize};
use serde_json::{Value};

pub mod rdcsv;
pub mod nodedata;

pub use rdcsv::{rtn_budget};
use nodedata::*; //{Acct, Budget, BEACat, Node, rtn_tree, Year, Filter, print_tree, add_tree, chg_node, BKey, LKV };

use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::future_to_promise;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response}; // TextDecoder};
use web_sys::console;
use js_sys::{Uint8Array, Array};

//#[wasm_bindgen]
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct JsLKV {
    pub idx: f64,
    pub name: String,
}

//#[wasm_bindgen]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JsAcct {
    idx:i32,
    pub key: BKey,
    name: String,
    pub tac: i16,
    pub scode: i16,
    pub bea: BEACat,
    pub onoff: bool,
    pub astr: i32,
    pub bstr: i32,
    pub sub: i32,
    value: Vec<i64>
}

impl JsAcct {
    fn new_from(acct: &Acct) -> JsAcct {
        JsAcct {
            idx: acct.idx,
            key: acct.key,
            name: acct.name.clone(),
            tac: acct.tac,
            scode: acct.scode,
            bea: acct.bea,
            onoff: acct.onoff,
            astr: acct.astr,
            bstr: acct.bstr,
            sub: acct.sub,
            value: acct.value.clone(),
        }
    }

    fn to_acct(acct: &JsAcct) -> Acct {
        Acct {
            idx: acct.idx,
            key: acct.key,
            name: acct.name.clone(),
            tac: acct.tac,
            scode: acct.scode,
            bea: acct.bea,
            onoff: acct.onoff,
            astr: acct.astr,
            bstr: acct.bstr,
            sub: acct.sub,
            value: acct.value.clone(), 
        }
    }
}

//#[wasm_bindgen]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JsBudget {
    pub anames: Vec<JsLKV>,
    pub bnames: Vec<JsLKV>,
    pub sname: Vec<JsLKV>,
    pub accts: Vec<JsAcct>,
}

impl JsBudget {
    fn new_from(bdgt: &Budget) -> Self {
        let an = bdgt.anames.iter().map(|a| { JsLKV {idx: a.idx as f64, name: a.name() }}).collect();
        let bn = bdgt.bnames.iter().map(|a| { JsLKV {idx: a.idx as f64, name: a.name() }}).collect();
        let sn = bdgt.sname.iter().map(|a| { JsLKV {idx: a.idx as f64, name: a.name() }}).collect();
        let at = bdgt.accts.iter().map(|a| { JsAcct::new_from(a)}).collect();
        JsBudget {
            anames: an,
            bnames: bn,
            sname: sn,
            accts: at,
        }
    }

    pub fn to_budget(&self) -> Budget {
        let an = self.anames.iter().map(|a| { LKV {idx: a.idx as i32, name: a.name.clone() }}).collect();
        let bn = self.bnames.iter().map(|a| { LKV {idx: a.idx as i32, name: a.name.clone() }}).collect();
        let sn = self.sname.iter().map(|a| { LKV {idx: a.idx as i32, name: a.name.clone() }}).collect();
        let at = self.accts.iter().map(|a| { JsAcct::to_acct(a)}).collect();
        Budget {
            anames: an,
            bnames: bn,
            sname: sn,
            accts: at
        }
    }
}
//#[wasm_bindgen]
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct JsNode {
    pub idx: i32,
    pub key: BKey,
    pub val: BVal,
    pub name:  String,
    pub chld: Vec<i32>,
    pub parnt: NParent,
    pub lock: bool,
    pub state: i16,
    pub leaf: i16       // -1 not leaf
}

impl JsNode {
    pub fn from_node(node: &Node) -> Self {
        let chld = node.chld.iter().map(|c| {*c as i32}).collect();
        JsNode {
            idx: node.idx,
            key: node.key,
            val: node.val,
            name:  node.name.clone(),
            chld: chld,
            parnt: node.parnt,
            lock: node.lock,
            state: node.state,
            leaf: node.leaf
        }
    }
}

#[wasm_bindgen]
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct T1 {
    pub acode: i32,     // -1 not used
    pub bcode: i32,     // -1 not used
    pub ccode: i32,     // -1 not used
    //pub tac: Option<i16>,
    //pub scode: Option<i16>,
    //pub bea: Option<BEACat>,
    //pub onoff: Option<bool>,
    pub year: u32,
}


pub fn get_tree(jsbdgt: JsBudget) -> Vec<Node> {
    let mut nodes: Vec<JsNode> = Vec::new();
    let an = jsbdgt.anames.iter().map(|a| { LKV::new(a.idx as i32, a.name.clone()) }).collect();
    let bn = jsbdgt.bnames.iter().map(|a| { LKV::new(a.idx as i32, a.name.clone()) }).collect();
    let sn = jsbdgt.sname.iter().map(|a| { LKV::new(a.idx as i32, a.name.clone()) }).collect();
    let act= jsbdgt.accts.iter().map(|a| { JsAcct::to_acct(a) }).collect();

    let bdgt: Budget = Budget {
        anames:  an,
        bnames: bn,
        sname: sn,
        accts: act
    };
    let f = Filter::new();
    let mut tre = rtn_tree(&bdgt, &f).unwrap();
    let ftree = add_tree(&mut tre);
    //let jstre: Vec<JsNode> = tre.iter().map(|n| {JsNode::from_node(n)}).collect();
    //let len = tre.len();  
    tre
}


pub async fn fetch_csv(url: String) -> Budget {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(
        &url,
        &opts,
    ).unwrap();  /////  Catch error

    request
        .headers()
        .set("Accept", "text/csv").unwrap();   ///// Catch error

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await.unwrap();  //// Catch eeror

    // `resp_value` is a `Response` object.
    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();
    let data = JsFuture::from(resp.text().unwrap()).await.unwrap();  /////Catch error

    let ary = JsValue::from(data);
    console::log_1(&ary);  // &"Test".into());
    let data: String = ary.as_string().unwrap();
    console::log_1(&JsValue::from_f64((data.len() as f64)));  // &"Test".into());

    let bdgt = rtn_budget(data);
    
    bdgt
}



#[wasm_bindgen]
pub async fn load_csv(url: String) -> Result<JsValue, JsValue> {
    //let path = Path::new(&pth);

    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(
        &url,
        &opts,
    )?;

    request
        .headers()
        .set("Accept", "text/csv")?;

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    // `resp_value` is a `Response` object.
    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();
    let data = JsFuture::from(resp.text()?).await?;

    let ary = JsValue::from(data);
    console::log_1(&ary);  // &"Test".into());
    let data: String = ary.as_string().unwrap();
    console::log_1(&JsValue::from_f64((data.len() as f64)));  // &"Test".into());

    let bdgt = rtn_budget(data);
    
    let jsbdgt = JsBudget::new_from(&bdgt);
    /*
        Ok(b) => {
            let jb = JsValue::from_serde(&b).unwrap();
            console::log_1(&jb);  // &"Test".into());
            let val = JsValue::from_serde(&b).unwrap();
            Ok(val)
        
        },
        Err(e) => {
            console::log_1(&JsValue::from(&format!("{}",e)));  // &"Test".into());
            Err(JsValue::from_str(&format!("{}",e)))
        },
    };*/
    Ok(JsValue::from_serde(&jsbdgt).unwrap())
    //let jb = JsValue::from_serde(&bdgt).unwrap();
    //console::log_1(&jb);  // &"Test".into());
    //let val = JsValue::from_serde(&bdgt).unwrap();
}
