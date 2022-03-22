use std::fs;

fn main() {
    println!("Hello, world!");

    let file = fs::read_to_string("./fixture/package.json").unwrap();
    let json: serde_json::Value = serde_json::from_str(&file).unwrap();

    println!("{}", json);
}
