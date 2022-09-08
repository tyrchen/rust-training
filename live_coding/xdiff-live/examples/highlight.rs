use std::fs;

use serde_json::json;
use xdiff_live::highlight_text;

fn main() {
    let v = json!({
        "foo": "bar",
        "baz": "qux"
    });
    let text = serde_json::to_string_pretty(&v).unwrap();

    let content = highlight_text(&text, "json", None).unwrap();
    fs::write("fixtures/highlight1.txt", content).unwrap();
}
