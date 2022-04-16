use std::{fs, path::Path};

use deno_core::{JsRuntime, RuntimeOptions};
use zstd::encode_all;

const SNAPSHOT_FILE: &str = "snapshots/main.bin";

fn main() {
    let options = RuntimeOptions {
        will_snapshot: true,
        ..Default::default()
    };
    let mut rt = JsRuntime::new(options);
    let data = rt.snapshot();
    let filename = Path::new(env!("CARGO_MANIFEST_DIR")).join(SNAPSHOT_FILE);
    let compressed = encode_all(&*data, 7).unwrap();
    fs::write(filename, compressed).unwrap();
}
