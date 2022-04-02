use core::panic;

use v8::{script_compiler, HandleScope, Script, ScriptOrigin, TryCatch};

use crate::LocalValue;

pub fn execute_script<'s>(
    scope: &mut HandleScope<'s>,
    code: impl AsRef<str>,
    is_module: bool,
) -> Result<LocalValue<'s>, LocalValue<'s>> {
    let scope = &mut TryCatch::new(scope);
    let source = v8::String::new(scope, code.as_ref()).unwrap();
    let origin = create_origin(scope, "dummy.js", is_module);
    if is_module {
        let source = script_compiler::Source::new(source, Some(&origin));
        let module = script_compiler::compile_module(scope, source).unwrap();
        module.instantiate_module(scope, module_callback).unwrap();
        let result = module.evaluate(scope).unwrap();
        let promise = v8::Local::<v8::Promise>::try_from(result).unwrap();
        match promise.state() {
            v8::PromiseState::Pending => panic!("We don't know how to process pending promise"),
            v8::PromiseState::Fulfilled => Ok(promise.result(scope)),
            v8::PromiseState::Rejected => Err(promise.result(scope)),
        }
    } else {
        Script::compile(scope, source, Some(&origin))
            .and_then(|script| script.run(scope))
            .map_or_else(|| Err(scope.stack_trace().unwrap()), Ok)
    }
}

fn module_callback<'s>(
    context: v8::Local<'s, v8::Context>,
    name: v8::Local<'s, v8::String>,
    arr: v8::Local<'s, v8::FixedArray>,
    module: v8::Local<'s, v8::Module>,
) -> Option<v8::Local<'s, v8::Module>> {
    println!("context: {:?}, name: {:?}, arr: {:?}", context, name, arr);
    Some(module)
}

fn create_origin<'s>(
    scope: &mut HandleScope<'s>,
    filename: impl AsRef<str>,
    is_module: bool,
) -> ScriptOrigin<'s> {
    let name: LocalValue = v8::String::new(scope, filename.as_ref()).unwrap().into();
    ScriptOrigin::new(
        scope,
        name.clone(),
        0,
        0,
        false,
        0,
        name,
        false,
        false,
        is_module,
    )
}
