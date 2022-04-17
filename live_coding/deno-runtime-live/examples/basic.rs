use deno_core::anyhow::Result;
use deno_runtime::{
    deno_core::{self, resolve_url_or_path, Extension},
    permissions::Permissions,
    worker::MainWorker,
};
use deno_runtime_live::MainWorkerOptions;

#[tokio::main]
async fn main() -> Result<()> {
    let disable_ops_ext = Extension::builder()
        .middleware(|op| match op.name {
            // "op_print" => op.disable(),
            // "op_fetch" => op.disable(),
            // "op_fetch_send" => op.disable(),
            // "op_fetch_custom_client" => op.disable(),
            _ => op,
        })
        .build();
    let mut options = MainWorkerOptions::default();
    options.extensions.push(disable_ops_ext);

    let js_file = format!("{}/examples/fetch.js", env!("CARGO_MANIFEST_DIR"));
    let url = resolve_url_or_path(&js_file)?;
    let permissions = Permissions {
        net: Permissions::new_net(&Some(vec![]), false),
        ..Default::default()
    };
    let mut worker =
        MainWorker::bootstrap_from_options(url.clone(), permissions, options.into_inner());

    worker.execute_main_module(&url).await?;
    Ok(())
}
