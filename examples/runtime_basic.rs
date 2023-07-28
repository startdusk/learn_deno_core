use std::path::Path;

use deno_core::{anyhow::Result, resolve_url_or_path, Extension};
use deno_runtime::{
    deno_core,
    permissions::{Permissions, PermissionsContainer},
    worker::{MainWorker, WorkerOptions},
};

// 运行一个完整的deno runtime代码
#[tokio::main]
async fn main() -> Result<()> {
    let mut options = WorkerOptions::default();
    let disable_ops_ext = Extension::builder("runtime")
        .middleware(|op| match op.name {
            // 禁用deno某个op函数，如禁用console.log()
            // "op_print" => op.disable(),
            _ => op,
        })
        .build();
    options.extensions.push(disable_ops_ext);
    let js_file = &format!("{}/examples/runtime_fetch.js", env!("CARGO_MANIFEST_DIR"));
    let path = format!("{}/examples/", env!("CARGO_MANIFEST_DIR"));
    let current_dir = Path::new(&path);
    let url = resolve_url_or_path(js_file, current_dir)?;
    // 设置允许网络访问的权限
    // let permissions = PermissionsContainer::new(Permissions {
    //     net: Permissions::new_net(&Some(vec![]), false)?,
    //     ..Default::default()
    // });
    // 允许全部权限
    let permissions = PermissionsContainer::new(Permissions::allow_all());
    let mut worker = MainWorker::bootstrap_from_options(url.clone(), permissions, options);
    worker.execute_main_module(&url).await?;
    Ok(())
}
