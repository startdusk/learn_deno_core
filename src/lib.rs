use std::path::Path;
use std::time::Duration;
use std::{slice::from_raw_parts, str::from_utf8_unchecked};

use deno_core::{
    anyhow::{anyhow, Result},
    resolve_url_or_path, serde_v8, v8, JsRuntime,
};
use serde::de::DeserializeOwned;

pub mod module;
pub mod ops;

pub async fn execute_main_module(rt: &mut JsRuntime, path: impl AsRef<str>) -> Result<()> {
    let path = Path::new(path.as_ref());
    let url = resolve_url_or_path("", path)?;
    let id = rt.load_main_module(&url, None).await?;
    let mut receiver = rt.mod_evaluate(id);

    // 执行js脚本的超时时间(目前不是很准确, 包含了网络请求等待的时间)
    let timeout = tokio::time::sleep(Duration::from_millis(100));
    let fut = async move {
        loop {
            tokio::select! {
                resolved = &mut receiver => {
                    return resolved.expect("failed to evaluate module");
                }
                _ = rt.run_event_loop(false) => {}
            }
        }
    };

    tokio::select! {
        ret = fut => ret,
        // 超时
        _ = timeout => Err(anyhow!("js script timeout"))
    }
}

pub async fn eval<T>(rt: &mut JsRuntime, code: &str) -> Result<T>
where
    T: DeserializeOwned,
{
    let code = to_static_str(code);
    let ret = rt.execute_script_static("demo", code)?;
    let result = rt.resolve_value(ret).await?;
    let scope = &mut rt.handle_scope();
    let result = v8::Local::new(scope, result);
    Ok(serde_v8::from_v8(scope, result)?)
}

pub fn to_static_str(s: &str) -> &'static str {
    let pointer = s.as_ptr() as usize;
    let length = s.len();
    unsafe { from_utf8_unchecked(from_raw_parts(pointer as *const u8, length)) }
}
