use std::{cell::RefCell, ops::Deref, rc::Rc, str::FromStr};

use deno_core::{
    error::AnyError, op, ByteString, Extension, ExtensionFileSource, ExtensionFileSourceCode,
    JsBuffer, Op, OpState, ToJsBuffer,
};
use reqwest::{
    header::{HeaderName, HeaderValue},
    Method, RequestBuilder, Url,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FetchArgs {
    url: String,
    method: String,
    headers: Vec<(ByteString, ByteString)>,
    body: Option<JsBuffer>,
}

#[derive(Serialize)]
pub struct FetchResponse {
    status: u16,
    status_text: String,

    headers: Vec<(ByteString, ByteString)>,
    body: Option<ToJsBuffer>,
}

#[op]
async fn op_fetch(state: Rc<RefCell<OpState>>, args: FetchArgs) -> Result<FetchResponse, AnyError> {
    let req: RequestBuilder = {
        let state_ref = state.borrow();
        let client = state_ref.borrow::<reqwest::Client>().clone();

        let method = Method::from_str(&args.method.to_ascii_lowercase())?;
        let url = Url::parse(&args.url)?;
        let mut req = client.request(method, url);
        for (k, v) in &args.headers {
            req = req.header(HeaderName::from_bytes(k)?, HeaderValue::from_bytes(v)?);
        }

        if let Some(body) = args.body {
            req.body(Vec::from(&*body))
        } else {
            req
        }
    };

    let res = req.send().await?;
    let status = res.status();
    let status_text = status.canonical_reason().unwrap_or("").to_string();
    let status = status.as_u16();
    let headers = res
        .headers()
        .iter()
        .map(|(k, v)| (k.as_str().into(), v.as_bytes().into()))
        .collect();
    let body = res.bytes().await?;
    let body: Option<ToJsBuffer> = if body.is_empty() {
        None
    } else {
        let body: Box<[u8]> = body.to_vec().into_boxed_slice();
        Some(body.into())
    };

    Ok(FetchResponse {
        status,
        status_text,
        headers,
        body,
    })
}

#[op]
fn op_decode_utf8<T>(buf: T) -> Result<String, AnyError>
where
    T: DeserializeOwned + Deref<Target = [u8]>,
{
    Ok(String::from_utf8_lossy(&buf).into())
}

const SOURCE_CODE: &str = include_str!("fetch.js");

pub fn init() -> Extension {
    Extension::builder("[fetch.js]")
        .js(vec![ExtensionFileSource {
            specifier: "[fetch.js]",
            code: ExtensionFileSourceCode::IncludedInBinary(SOURCE_CODE),
        }])
        .ops(vec![op_fetch::DECL, op_decode_utf8::<JsBuffer>::DECL])
        .state(move |state| {
            state.put::<reqwest::Client>(reqwest::Client::new());
        })
        .build()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn op_decode_utf8_should_word() {
        let v = b"hello".to_vec();
        let res = op_decode_utf8::call(v).unwrap();
        assert_eq!(res, "hello")
    }
}
