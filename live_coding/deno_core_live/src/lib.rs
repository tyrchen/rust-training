use std::time::Duration;

use deno_core::{
    anyhow::{anyhow, Result},
    resolve_url_or_path, serde_v8, v8, JsRuntime,
};
use serde::de::DeserializeOwned;

pub mod ops;

pub async fn eval<T>(rt: &mut JsRuntime, code: &str) -> Result<T>
where
    T: DeserializeOwned,
{
    // let code = include_str!("basic.js");
    let ret = rt.execute_script("<anon>", code)?;
    let result = rt.resolve_value(ret).await?;
    let scope = &mut rt.handle_scope();
    let result = v8::Local::new(scope, result);
    Ok(serde_v8::from_v8(scope, result)?)
}

pub async fn execute_main_module(rt: &mut JsRuntime, path: impl AsRef<str>) -> Result<()> {
    let url = resolve_url_or_path(path.as_ref())?;
    let id = rt.load_main_module(&url, None).await?;
    let mut receiver = rt.mod_evaluate(id);
    let timer = tokio::time::sleep(Duration::from_millis(500));

    let fut = async move {
        loop {
            tokio::select! {
                resolved = &mut receiver => {
                    return resolved.expect("failed to evaluate module");
                }
                _ = rt.run_event_loop(false) => {}

            }
        }
    };

    tokio::select! {
        ret = fut => ret,
        _ = timer => Err(anyhow!("js script timeout"))
    }
}
