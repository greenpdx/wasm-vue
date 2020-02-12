use std::io;
use std::fs::File;
use std::io::prelude::*;

use std::path::Path;

mod nodedata;
mod rdcsv;
//use bdgtree;

use rdcsv::{rtn_budget};
use nodedata::*; //{Budget, BEACat, Node, rtn_tree, Year, Filter, print_tree, add_tree, chg_node };

fn main() -> io::Result<()> {
    let path = Path::new("../static/test-small.csv");

    let mut file = File::open("./static/test-small.csv")?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf);

    let budget = rtn_budget(String::from_utf8(buf).unwrap());

    let filter = Filter {
        acode: -1,     // -1 not used
        bcode: -1,     // -1 not used
        ccode: -1,     // -1 not used
        tac: None,  
        scode: None,
        bea: Some(BEACat::D),
        onoff: Some(true),
        year: Year::Y2020
    };
    |budget: Budget| {
        let accts = budget.accts;
        println!("{:?}", accts[0]);
    };
    let mut vecnode = rtn_tree(&budget, &filter).unwrap();
    //println!("{:?}", vecnode.len());
    add_tree(&mut vecnode);
    print_tree(&vecnode, &budget);
    let _node: Node = vecnode[0].clone();
    chg_node(&mut vecnode, 3, 5.0);
    print_tree(&vecnode, &budget);
    lock_node(&mut vecnode, 2);
    chg_node(&mut vecnode, 3, -5.0);
    print_tree(&vecnode, &budget);
    unlock_node(&mut vecnode, 2);
    chg_node(&mut vecnode, 3, -15.0);
    print_tree(&vecnode, &budget);
    Ok(())
}