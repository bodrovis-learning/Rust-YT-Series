use std::collections::HashMap;

fn main() {
    // hash maps
    let mut a = HashMap::new();

    // a.insert(String::from("demo"), 42);
    a.insert(String::from("key"), 100);
    a.insert(String::from("key"), 200);

    let value = a.get(&String::from("demo")).unwrap_or(&1);

    a.entry(String::from("demo")).or_insert(50);

    for(k, v) in &a {
        println!("Key: {k}, value: {v}");
    }
}
