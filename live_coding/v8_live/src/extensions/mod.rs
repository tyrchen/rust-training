use std::future::Future;

use crate::utils::execute_script;
use lazy_static::lazy_static;
use v8::{
    ExternalReference, ExternalReferences, FunctionCallbackArguments, HandleScope, MapFnTo,
    ReturnValue,
};

const GLUE: &str = include_str!("glue.js");

lazy_static! {
    pub static ref EXTERNAL_REFERENCES: ExternalReferences = ExternalReferences::new(&[
        ExternalReference {
            function: MapFnTo::map_fn_to(print),
        },
        ExternalReference {
            function: MapFnTo::map_fn_to(fetch),
        }
    ]);
}

pub struct Extensions;

impl Extensions {
    pub fn install(scope: &mut HandleScope) {
        let bindings = v8::Object::new(scope);

        let name = v8::String::new(scope, "print").unwrap();
        let func = v8::Function::new(scope, print).unwrap();
        bindings.set(scope, name.into(), func.into()).unwrap();

        let name = v8::String::new(scope, "fetch").unwrap();
        let func = v8::Function::new(scope, fetch).unwrap();
        bindings.set(scope, name.into(), func.into()).unwrap();

        if let Ok(result) = execute_script(scope, GLUE, false) {
            let func = v8::Local::<v8::Function>::try_from(result).unwrap();
            let v = v8::undefined(scope).into();
            let args = [bindings.into()];
            func.call(scope, v, &args).unwrap();
        }
    }
}

fn print(scope: &mut HandleScope, args: FunctionCallbackArguments, mut rv: ReturnValue) {
    let result: serde_json::Value = serde_v8::from_v8(scope, args.get(0)).unwrap();
    println!("Rust says: {result:#?}");
    rv.set(serde_v8::to_v8(scope, result).unwrap());
}

fn fetch(scope: &mut HandleScope, args: FunctionCallbackArguments, mut rv: ReturnValue) {
    let url: String = serde_v8::from_v8(scope, args.get(0)).unwrap();
    let fut = async move {
        let result = reqwest::get(&url).await.unwrap().text().await.unwrap();
        rv.set(serde_v8::to_v8(scope, result).unwrap());
    };

    run_local_future(fut);
}

fn run_local_future<R>(fut: impl Future<Output = R>) {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    let local = tokio::task::LocalSet::new();
    local.block_on(&rt, fut);
}
