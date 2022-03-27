use v8::{HandleScope, Script, TryCatch};

use crate::LocalValue;

pub fn execute_script<'s>(
    scope: &mut HandleScope<'s>,
    code: impl AsRef<str>,
) -> Result<LocalValue<'s>, LocalValue<'s>> {
    let scope = &mut TryCatch::new(scope);
    let source = v8::String::new(scope, code.as_ref()).unwrap();
    Script::compile(scope, source, None)
        .and_then(|script| script.run(scope))
        .map_or_else(|| Err(scope.stack_trace().unwrap()), Ok)
}
