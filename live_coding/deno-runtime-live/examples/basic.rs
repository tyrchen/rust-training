use deno_core::{anyhow::Result, error::AnyError};
use deno_runtime::{
    deno_core::{self, resolve_url_or_path, Extension},
    permissions::Permissions,
    worker::MainWorker,
};
use deno_runtime_live::MainWorkerOptions;
use tokio::runtime::Builder;

fn main() -> Result<()> {
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

    // let js_file = format!("{}/examples/rest.ts", env!("CARGO_MANIFEST_DIR"));

    let url = resolve_url_or_path("/tmp/rest.js")?;
    let permissions = Permissions {
        net: Permissions::new_net(&Some(vec![]), false),
        ..Default::default()
    };

    let rt = Builder::new_current_thread()
        .enable_all()
        .max_blocking_threads(32)
        .build()?;

    let fut = async move {
        let mut worker =
            MainWorker::bootstrap_from_options(url.clone(), permissions, options.into_inner());

        worker.execute_main_module(&url).await?;
        worker.run_event_loop(false).await?;
        Ok::<_, AnyError>(())
    };
    let local = tokio::task::LocalSet::new();
    local.block_on(&rt, fut)?;
    Ok(())
}
