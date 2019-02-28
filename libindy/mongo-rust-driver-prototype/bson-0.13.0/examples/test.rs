extern crate bson;
use std::collections::HashMap;
fn main() {
    let mut map: HashMap<i64, i64> = HashMap::new();
    map.insert(1, 0);
    map.insert(2, 1);
    let tmp = bson::to_bson(&map).unwrap();
}
