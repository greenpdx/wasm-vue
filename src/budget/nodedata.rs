use serde::{ Deserialize, Serialize};
use std::error::Error;
use wasm_bindgen::prelude::*;
use js_sys::JsString;
use std::f64;

//use ndarray_csv::{ArrayReader, ArrayWriter};
#[wasm_bindgen]
#[allow(dead_code)]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq,Deserialize, Serialize)]
pub enum Year {
Y1976, TQ, Y1977, Y1978, Y1979, Y1980, Y1981,
Y1982, Y1983, Y1984, Y1985, Y1986, Y1987,
Y1988, Y1989, Y1990, Y1991, Y1992, Y1993,
Y1994, Y1995, Y1996, Y1997, Y1998, Y1999,
Y2000, Y2001, Y2002, Y2003, Y2004, Y2005,
Y2006, Y2007, Y2008, Y2009, Y2010, Y2011,
Y2012, Y2013, Y2014, Y2015, Y2016, Y2017,
Y2018, Y2019, Y2020, Y2021, Y2022, Y2023,
Y2024
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy,Deserialize, Serialize)]
pub struct BKey {
    pub acode: i16,
    pub bcode: i16,
    pub ccode: i32,
}
#[wasm_bindgen]
impl BKey {
    pub fn new(a: i16, b: i16, c: i32) -> Self {
        BKey {
            acode: a,
            bcode: b,
            ccode: c
        }
    }
}

impl PartialEq for BKey {
    fn eq(&self, other: &BKey) -> bool {
        self.acode == other.acode && self.bcode == other.bcode
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
pub enum BEACat {
    I,  //NetInterest,
    M,  //Mandatory,
    D,  //Discretionary
    X, 
}

//#[wasm_bindgen]
#[derive(Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct LKV {
    pub idx: i32,
    pub name: String,
}


//#[wasm_bindgen]
impl LKV {
    //#[wasm_bindgen(constructor)]
    pub fn new(idx: i32,name: String) -> LKV {
        LKV { 
            idx: idx,
            name: name
         }
    }

    //#[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        let s = self.name.clone();
        String::from(s)
    }

    //#[wasm_bindgen(setter)]
    pub fn set_name(&mut self, name: String) {
        self.name = name.clone();
    }
}

impl Clone for LKV {
    fn clone(&self) -> Self {
        LKV {
            idx: self.idx,
            name: self.name.clone(),
        }
    }
}

/*
    pub fn get_name() -> Result<Budget, JsValue> {
        let an: Vec<LKV> = Vec::new();
        let bn: Vec<LKV> = Vec::new();
        let sn: Vec<LKV> = Vec::new();
        let ac: Vec<Acct> = Vec::new();
        let b = Budget {
            anames: an,
            bnames: bn,
            sname: sn,
            accts: ac,
        };
        Ok(b)
    }
*/


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Acct {
    pub idx:i32,
    pub key: BKey,
    pub name: String,
    pub tac: i16,
    pub scode: i16,
    pub bea: BEACat,
    pub onoff: bool,
    pub astr: i32,
    pub bstr: i32,
    pub sub: i32,
    pub value: Vec<i64>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Budget {
    pub anames: Vec<LKV>,
    pub bnames: Vec<LKV>,
    pub sname: Vec<LKV>,
    pub accts: Vec<Acct>,
}

pub type NParent = i32;
pub type BVal = f64;


#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Node {
    pub idx: i32,
    pub key: BKey,
    pub val: BVal,
    pub name: String,
    pub chld: Vec<i32>,
    pub parnt: NParent,
    pub lock: bool,
    pub state: i16,
    pub leaf: i16       // -1 not leaf
}

impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        self.key == other.key
        || self.idx == other.idx
    }
}

impl Node {
    //const nvec: Vec<Acct> = Vec::<Acct>::new();
    pub fn new(idx: i32, name: String, key: BKey) -> Node {
        Node {
            idx: idx,
            val: 0.0,
            name: name,
            chld: Vec::new(),
            parnt: 0,
            lock: false,
            leaf: -1,
            key: key,
            state: 0,
        }
    }
}

#[wasm_bindgen]
pub struct Filter {
    pub acode: i32,     // -1 not used
    pub bcode: i32,     // -1 not used
    pub ccode: i32,     // -1 not used
    // name: String,
    pub tac: Option<i16>,
    pub scode: Option<i16>,
    pub bea: Option<BEACat>,
    pub onoff: Option<bool>,
    // aprnt: i32,
    // bprnt: i32,
    // value: Vec<i64>,
    pub year: Year,
}

impl Filter {
    pub fn new() -> Self {
        Filter {
            acode: -1,
            bcode: -1,
            ccode: -1,
            tac: None,
            scode: None,
            bea: Some(BEACat::D),
            onoff: None,
            year: Year::Y2020
        } 
    }
}

fn filtr_acct( acct: &Acct, filter: &Filter) -> BVal {
    let bea = match filter.bea  {
        Some(b) => {
            b == acct.bea
        }
        None => true
    };
   let scode = match filter.scode {
        Some(s) => {
            s == acct.scode
        }
        None => true
    };
   let onoff = match filter.onoff {
        Some(o) => {
            o == acct.onoff
        }
        None => true
    };
   let tac = match filter.tac {
        Some(t) => {
            t == acct.tac
        }
        None => true
    };
    let y = filter.year as usize;
    let yv = acct.value[y];
    let year = yv != 0;
    //println!("{:?} {:?} {:?} {:?}", bea, scode, year, yv);
    if year && bea && scode && onoff && tac {
        yv as BVal
    } else {
        0.0
    }

}

fn mkleaf(idx: i32, act: &Acct, year: usize) -> Node {
    let mut leaf = Node::new(idx, act.name.clone(), act.key);
    leaf.leaf = act.idx as i16;
    //let idx = year as usize;
    leaf.val = act.value[year] as BVal;
    leaf
}

pub fn rtn_tree(bdgt: &Budget,filter: &Filter) -> Result<Vec<Node>, Box<dyn Error>> {
//    pub fn rtn_tree(bdgt: &Budget,filter: &Filter) -> Result<Vec<Node>, Box<dyn Error>> {
    let accts = &bdgt.accts;
    let anames = &bdgt.anames;
    let bnames = &bdgt.bnames;

    //let mut bset: HashSet<&mut Node> = HashSet::new();
    let mut vecnode: Vec<Node> = Vec::new();
    let mut root = Node::new(0, "Total".to_string(), BKey::new(-1, -1, -1));
    root.parnt = -1;
    vecnode.push(root);
    let mut root: Vec<i32> = Vec::new();
    let mut cnt = 0 as i32;
    for (_idx, act) in accts.iter().enumerate() {
        let ans  =  filtr_acct(act, &filter);
        if ans == 0.0 { continue; }
        //if cnt > 12 {
        //    break
        //}
        cnt = cnt + 1;

        let mut aidx = -1;
        let mut bidx = -1;
        /*
        */
        for i in  0..vecnode.len() {
            if vecnode[i].leaf == -1 { 
                if vecnode[i].key == act.key {
                    bidx = i as i32;
                    break;
                } else if vecnode[i].key == BKey::new(act.key.acode, -1, -1) {
                    aidx = i as i32;
                }
            }
        }
        let mut anode: Node;
        let mut bnode: Node;
        let mut leaf: Node;
        let mut lidx = vecnode.len() as i32;
         if aidx == -1 {
  //          println!(" AIDX {:?}", lidx);
            // add agency. bureau, and acct
            aidx = vecnode.len() as i32;
            bidx = aidx + 1;
            lidx = bidx + 1;
            
            anode = Node::new(aidx, anames[act.astr as usize].name.clone(), BKey::new(act.key.acode, -1, -1 ));
            bnode = Node::new(bidx, bnames[act.bstr as usize].name.clone(), act.key);
            leaf = mkleaf(lidx, &act, Year::Y2020 as usize);

            anode.parnt = 0;
            anode.chld.push(bidx);
            bnode.parnt = aidx;
            //bnode.chld.push(lidx);
            leaf.parnt = bidx;

            vecnode.push(anode);
            vecnode.push(bnode);
            //vecnode.push(leaf);

            root.push(aidx);
        }
       if bidx == -1 {
 //           println!(" BIDX {:?} {:?}", aidx, lidx);
            bidx = vecnode.len() as i32;
            lidx = bidx+1;

            vecnode[aidx as usize].chld.push(bidx);
            bnode = Node::new(bidx, bnames[act.bstr as usize].name.clone(), act.key);
            //leaf = mkleaf(lidx, &act);
 
            bnode.parnt = aidx;
            //bnode.chld.push(lidx);
            //leaf.parnt = bidx;

            vecnode.push(bnode);
            //vecnode.push(leaf);
       }
        
       vecnode[bidx as usize].chld.push(lidx);
       leaf = mkleaf(lidx, &act, Year::Y2020 as usize);
       leaf.parnt = bidx;
    
       vecnode.push(leaf);     
    };
    vecnode[0].chld = root;
    Ok(vecnode)
}

pub fn print_tree(vecnode: &Vec<Node>, bdgt: &Budget ) {
       struct PrtNode<'n> {
           f: &'n  dyn Fn(&PrtNode, &Node, i32) }
        let prt_node = PrtNode {
            f: &|prt_node, node, level| {
                //println!("\n");
                print!("{:?} {:?} {:?} {:?} {:?} {:?} ", node.idx, level, node.key, node.val, node.name, node.lock );
                if node.leaf != -1 {
                    let leaf = &bdgt.accts[node.leaf as usize];
                    print!("\n{:?} {:?} {:?} {:?}", leaf.key.ccode, node.name, leaf.bea, bdgt.sname[leaf.sub as usize]);
                    print!("{:?} {:?} ",node.leaf, node.val);
                } else {
                    print!("{:?} ", node.chld);
                };
                println!(".");
                for n in node.chld.iter() {
                    (prt_node.f)(prt_node, &vecnode[*n as usize], level + 1)
                }
            }
        };
    
 
    (prt_node.f)(&prt_node, &vecnode[0], 0);
    //print_children(&vecnode[0].chld);     
}

pub fn add_tree(vecnode: &mut Vec<Node>) {
    fn add_node(vecnode: &mut Vec<Node>, idx: i32, level: i32) -> BVal {
        let node = vecnode[idx as usize].clone();
        let mut val = 0.0;
        if node.leaf != -1 {
            return node.val;
        } else {
            for i in node.chld.iter() {
                val = val + add_node(vecnode, i.clone(), level+1);
            }
            vecnode[idx as usize].val = val;
            return val
        }
    }
    let total = add_node(vecnode, 0, 0);
    vecnode[0].val = total;

} 

// parents mu be uptodate before this function
// 
fn chg_childs(vecnode: &mut Vec<Node>, idx: i32, delta: BVal) -> BVal {
    let node = vecnode[idx as usize].clone();
    let mut lockval = 0.0;
    let mut useval = 0.0;
    let mut val = 0.0;
    let mut diff = delta;
    if node.val + delta < 0.0 {
        diff = delta + node.val;
        vecnode[idx as usize].val = 0.0;
    };
    println!("START CHILD {:?} {:?} {:?} {:?}", idx, diff, node.chld.len(), node.val);
    if node.leaf == -1 {
        for i in node.chld.iter() {
            if vecnode[*i as usize].lock ||  vecnode[*i as usize].state > 0 {
                lockval += node.val;
                continue;
            }
            useval += vecnode[*i as usize].val;
        };
        if diff > useval {
            // error
            return 0.0
        };
        //println!("### {:?} {:?}", useval, val);
        let oldval = node.val;
        for i in node.chld.iter() {
            let chld = vecnode[*i as usize].clone();
            if ! chld.lock  && chld.state == 0 {
                let dif = vecnode[*i as usize].val * diff / useval;
                val += chg_childs(vecnode, *i, dif);
            } else {
                val += chld.val;
            }
        }
        //println!("{:?} {:?} {:?}", node.leaf, oldval, val) ;
        vecnode[idx as usize].val = val;
    } else {
        vecnode[idx as usize].val += diff;
        //println!("### {:?} {:?}", node.val, vecnode[idx as usize].val);
    };
    println!("END CHLD {:?} {:?} {:?}\n", idx, vecnode[idx as usize].val, lockval);
    vecnode[idx as usize].val
}

fn chg_parent(vecnode: &mut Vec<Node>, idx: i32, delta: BVal) -> BVal {
    let node = vecnode[idx as usize].clone();
    let  pidx = node.parnt;
    let val = node.val;
    println!("PCHG {:?} {:?} {:?}", idx, val, delta);
    if idx == 0 { //at top
        let num = chg_childs(vecnode, 0, -delta);
        //println!("ROOT EQ {:?} {:?}", node.val, num);
    } else {
        //let num = chg_childs(vecnode, 0, -delta);
        if node.lock {
            let num = chg_childs(vecnode, idx, delta);
        } else {
            vecnode[idx as usize].state = 1;
            let mut diff = delta;
            if node.val + delta < 0.0 {
                diff = delta.abs() + node.val;
                vecnode[idx as usize].val = 0.0;
            } else {
                vecnode[idx as usize].val += diff;
            };
            chg_parent(vecnode, pidx, delta);
            vecnode[idx as usize].state = 0;
        }
    }
    0.0
}

fn nodmin(vecnode: &Vec<Node>) -> BVal {
    let mut min: BVal = f64::MAX;
    let itr = vecnode.iter();
    for itm in itr {
        if itm.val < min { min = itm.val; }
    }
    min
}

pub fn chg_node(vecnode: &mut Vec<Node>, idx: i32, delta: BVal) -> BVal {
    let node = vecnode[idx as usize].clone();
    println!("\n\nCHG NODE {:?} {:?} {:?}", idx, delta, node.leaf);
    // split for children and traverse add up to root
    if node.leaf != -1 {
        //let leaf = vecnode[node.leaf as usize].clone();
        vecnode[idx as usize].state = 1;
        chg_parent(vecnode, idx, delta);
        vecnode[idx as usize].state = 0;

    } else {
        vecnode[idx as usize].state = 1;
        if delta.abs() > nodmin(vecnode) {
            println!("CHKOver {:?}", delta);
        }
        let num = chg_childs(vecnode, idx, delta);
        chg_parent(vecnode, idx, delta);
        vecnode[idx as usize].state = 0;
        //println!("FIN {:?}", num);
    }
    0.0
}

pub fn lock_node(vecnode: &mut Vec<Node>, idx: i32) {
    //let node = vecnode[idx as usize].clone();
    vecnode[idx as usize].lock = true;
}

pub fn unlock_node(vecnode: &mut Vec<Node>, idx: i32) {
    //let node = vecnode[idx as usize].clone();
    vecnode[idx as usize].lock = false;
}

pub fn node_locked(vecnode: &mut Vec<Node>, idx: i32) -> bool {
    //let node = vecnode[idx as usize].clone();
    vecnode[idx as usize].lock
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
