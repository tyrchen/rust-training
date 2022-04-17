use std::rc::Rc;

use deno_core::{anyhow::Result, FsModuleLoader, JsRuntime, RuntimeOptions, Snapshot};
use deno_core_live::execute_main_module;
use lazy_static::lazy_static;

lazy_static! {
    static ref SNAPSHOT: &'static [u8] = {
        let data = include_bytes!("../snapshots/main.bin");
        let decompressed = zstd::decode_all(&data[..]).unwrap().into_boxed_slice();
        Box::leak(decompressed)
    };
}

#[tokio::main]
async fn main() -> Result<()> {
    let options = RuntimeOptions {
        module_loader: Some(Rc::new(FsModuleLoader)),
        startup_snapshot: Some(Snapshot::Static(&*SNAPSHOT)),
        ..Default::default()
    };

    let mut rt = JsRuntime::new(options);

    let path = format!("{}/examples/basic_module.js", env!("CARGO_MANIFEST_DIR"));
    execute_main_module(&mut rt, &path).await?;
    // let code = include_str!("basic.js");
    // let ret: String = eval(&mut rt, code).await?;
    // print!("Rust: {:?}", ret);

    Ok(())
}
