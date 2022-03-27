use std::{cell::RefCell, rc::Rc};

use v8::{HandleScope, Isolate};

type GlobalContext = v8::Global<v8::Context>;
type JsRuntimeStateRef = Rc<RefCell<JsRuntimeState>>;

pub struct JsRuntimeState {
    context: Option<GlobalContext>,
}

impl JsRuntimeState {
    pub fn new(isolate: &mut Isolate) -> JsRuntimeStateRef {
        let context = {
            let handle_scope = &mut HandleScope::new(isolate);
            let context = v8::Context::new(handle_scope);
            v8::Global::new(handle_scope, context)
        };

        Rc::new(RefCell::new(JsRuntimeState {
            context: Some(context),
        }))
    }

    pub fn get_context(isolate: &mut Isolate) -> GlobalContext {
        let state = isolate.get_slot::<JsRuntimeStateRef>().unwrap().clone();
        let ctx = &state.borrow().context;
        ctx.as_ref().unwrap().clone()
    }
}
