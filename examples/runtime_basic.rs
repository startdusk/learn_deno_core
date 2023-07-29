use std::path::Path;

use deno_core::{anyhow::Result, Extension, ModuleSpecifier};
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
    let js_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/example.ts");
    let main_module = ModuleSpecifier::from_file_path(js_path).unwrap();
    // 设置允许网络访问的权限
    // let permissions = PermissionsContainer::new(Permissions {
    //     net: Permissions::new_net(&Some(vec![]), false)?,
    //     ..Default::default()
    // });
    // 允许全部权限
    let permissions = PermissionsContainer::new(Permissions::allow_all());
    let mut worker = MainWorker::bootstrap_from_options(main_module.clone(), permissions, options);
    worker.execute_main_module(&main_module).await?;
    Ok(())
}
