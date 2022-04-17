use deno_runtime::{
    deno_broadcast_channel::InMemoryBroadcastChannel,
    deno_core::{error::AnyError, FsModuleLoader},
    deno_web::BlobStore,
    worker::WorkerOptions,
    BootstrapOptions,
};
use std::{
    ops::{Deref, DerefMut},
    rc::Rc,
    sync::Arc,
};

pub struct MainWorkerOptions(WorkerOptions);

impl Deref for MainWorkerOptions {
    type Target = WorkerOptions;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for MainWorkerOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl MainWorkerOptions {
    pub fn into_inner(self) -> WorkerOptions {
        self.0
    }
}

impl Default for MainWorkerOptions {
    fn default() -> Self {
        let create_web_worker_cb = Arc::new(|_| {
            panic!("Web workers are not supported");
        });
        let web_worker_preload_module_cb = Arc::new(|_| {
            panic!("Web workers are not supported");
        });
        let bootstrap = BootstrapOptions {
            args: vec![],
            cpu_count: 1,
            debug_flag: false,
            enable_testing_features: false,
            location: None,
            no_color: false,
            is_tty: false,
            runtime_version: "x".to_string(),
            ts_version: "x".to_string(),
            unstable: false,
            apply_source_maps: true,
        };
        Self(WorkerOptions {
            bootstrap,
            extensions: vec![],
            unsafely_ignore_certificate_errors: None,
            root_cert_store: None,
            user_agent: "my-runtime 0.1".to_string(),
            seed: None,
            module_loader: Rc::new(FsModuleLoader),
            create_web_worker_cb,
            web_worker_preload_module_cb,
            js_error_create_fn: None,
            maybe_inspector_server: None,
            should_break_on_first_statement: false,
            get_error_class_fn: Some(&get_error_class_name),
            origin_storage_dir: None,
            blob_store: BlobStore::default(),
            broadcast_channel: InMemoryBroadcastChannel::default(),
            shared_array_buffer_store: None,
            compiled_wasm_module_store: None,
        })
    }
}

fn get_error_class_name(e: &AnyError) -> &'static str {
    deno_runtime::errors::get_error_class_name(e).unwrap_or("Error")
}
