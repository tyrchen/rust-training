use serde::{Deserialize, Serialize};
use v8::{CreateParams, HandleScope, Isolate, Script, V8};

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    pub status: usize,
    pub message: String,
}

fn main() {
    // initialize v8 engine
    init();
    // create an isolate
    let params = CreateParams::default();
    let isolate = &mut Isolate::new(params);

    // create handle scope
    let handle_scope = &mut HandleScope::new(isolate);
    // create context
    let context = v8::Context::new(handle_scope);
    // create context scope
    let context_scope = &mut v8::ContextScope::new(handle_scope, context);

    // javascript code
    let source = r#"
        function hello() {
            return {
                status: 200,
                message: "Hello from Rust!"
            };
        }
        hello();
    "#;
    let source = v8::String::new(context_scope, source).unwrap();
    let script = Script::compile(context_scope, source, None).unwrap();
    let result = script.run(context_scope).unwrap();
    let value: serde_json::Value = serde_v8::from_v8(context_scope, result).unwrap();
    println!("Result is: {value:?}");
}

fn init() {
    let platform = v8::new_default_platform(0, false).make_shared();
    V8::initialize_platform(platform);
    V8::initialize();
}
