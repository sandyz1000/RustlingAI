use yew::prelude::*;
use std::sync::Arc;
use std::future::Future;

pub type HandleAsyncFunc = Arc<dyn Fn() -> Box<dyn Future<Output = Option<String>> + Send> + Send + Sync>;


#[hook]
pub fn use_submit() -> (HandleAsyncFunc, String) {
    todo!()
}
