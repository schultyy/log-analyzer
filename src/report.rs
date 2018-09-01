use serde_json;
use serde;

pub fn print_json<T: serde::Serialize>(payload: T) {
    let json = serde_json::to_string_pretty(&payload).expect("Failed to serialize to JSON");
    println!("{}", json);
}