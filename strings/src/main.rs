extern crate json;
use std::io;

fn main() {
    // Explicit type declaration
    let s_ref: &str = "ba";

    // Type inference
    let mut s = String::from("hi there");

    // Print to stdout
    println!("Hello, world! {} {}", s_ref, s);

    // Format to string
    let x: &str = format!("This is me {}", "really");
    println!("{}", x);

    // Read from stdin
    io::stdin().read_line(&mut s).expect("Failed to read");
    println!("{}", s);

    // Let's parse some json
    let js: json::JsonValue = json::parse(r#"{"key": "value"}"#).expect("Failed to parse json");
    println!("{}", js["key"]);
}
