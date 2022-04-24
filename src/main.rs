use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert("chal-1", true);
    map.insert("chal-2", false);

    for i in map.iter() {
        println!("{:?}", i);
    }

    println!("Hello, world!");
}
