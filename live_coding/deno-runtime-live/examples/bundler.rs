use deno_bundler::BundleOptions;
use deno_core::resolve_url_or_path;
use deno_runtime::deno_core;
use tokio::fs;

#[tokio::main]
async fn main() {
    let options = BundleOptions::default();
    let f = format!("{}/examples/rest.ts", env!("CARGO_MANIFEST_DIR"));
    let m = resolve_url_or_path(&f).unwrap();
    let (code, _) = deno_bundler::bundle(m, options).await.unwrap();
    fs::write("/tmp/rest.js", code).await.unwrap();
}
