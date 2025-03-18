use yew::prelude::*;
use yewdux::use_store;
use crate::ChatSlice;
use gloo_storage::{LocalStorage, Storage};

#[hook]
fn use_save_to_local_storage() {
    let (state, dispatch) = use_store::<ChatSlice>();
    let chats_ref = use_state(|| state.chats.clone());

    use_effect({
        let dispatch = dispatch.clone();
        let chats_ref = chats_ref.clone();
        move || {
            let unsubscribe = dispatch.subscribe({
                let chats_ref = chats_ref.clone();
                move |new_state: std::rc::Rc<ChatSlice>| {
                    if *chats_ref != new_state.chats {
                        chats_ref.set(new_state.chats.clone());
                        if let Ok(chats_json) = serde_json::to_string(&new_state.chats) {
                            LocalStorage::set("chats", chats_json).expect("Failed to save chats to local storage");
                        }
                    }
                }
            });

            // Cleanup function to unsubscribe when the component unmounts
            move || drop(unsubscribe)
        }
    });
}
