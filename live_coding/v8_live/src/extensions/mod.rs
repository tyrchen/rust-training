use v8::{FunctionCallbackArguments, HandleScope, ReturnValue};

use crate::utils::execute_script;

const GLUE: &str = include_str!("glue.js");

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

        if let Ok(result) = execute_script(scope, GLUE) {
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
    let result = reqwest::blocking::get(&url).unwrap().text().unwrap();
    rv.set(serde_v8::to_v8(scope, result).unwrap());
}
