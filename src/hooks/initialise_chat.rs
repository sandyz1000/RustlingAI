use std::rc::Rc;
use yew::prelude::*;
use yewdux::use_store;

use crate::{store::slice::ChatSlice, types::chat::ChatInterface};

#[hook]
pub fn use_initialise_chat() -> Rc<dyn Fn()> {

    let (_, dispatch) = use_store::<ChatSlice>();
    let init_new_chat = {
        let dispatch = dispatch.clone();
        move || {
            dispatch.reduce_mut(|f| {
                f.chats = vec![ChatInterface::new(
                    "".into(),
                    None,
                    vec![],
                    None,
                    "".into(),
                )];
                f.curr_chat_index = 0;
            })
        }
    };
    Rc::new(init_new_chat)
}
