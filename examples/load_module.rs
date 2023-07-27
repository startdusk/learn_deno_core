use std::rc::Rc;

use deno_core::{anyhow::Result, FsModuleLoader, JsRuntime, RuntimeOptions};
use learn_deno_core::execute_main_module;

#[tokio::main]
async fn main() -> Result<()> {
    let options = RuntimeOptions {
        module_loader: Some(Rc::new(FsModuleLoader)),
        ..Default::default()
    };
    let mut rt = JsRuntime::new(options);

    let path = format!("{}/examples/load_module.js", env!("CARGO_MANIFEST_DIR"));
    execute_main_module(&mut rt, path).await?;
    Ok(())
}
