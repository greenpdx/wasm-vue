pub mod rdcsv;
pub mod nodedata;

//  this exports as bdgtree::{rtn_budget}
//pub use rdcsv::{rtn_budget};
//pub use nodedata::{Budget, BEACat, Node, rtn_tree, Year, Filter, print_tree, add_tree, chg_node };

use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

// this is first line test
// 
// ```
// mod rdcsv;
//
// use rdcsv;
// ```

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn load_test_csv() {
        let path = Path::new("./test-small.csv");
        let bdgt = rdcsv::rtn_budget(path).unwrap();
        assert_eq!(bdgt.accts.len(), 5);
        println!("{:?}", bdgt);
    }
    #[test]
    fn get_tree() {

    }
}
