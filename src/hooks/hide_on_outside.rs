use gloo::events::EventListener;
use yew::prelude::*;

#[hook]
pub(crate) fn use_hideon_outside_click() -> (UseStateHandle<bool>, NodeRef) {
    let element_ref = use_node_ref();
    let show_element = use_state(|| false);

    {
        let (show_element, element_ref) = (show_element.clone(), element_ref.clone());

        use_effect_with(
            (show_element, element_ref),
            move |(show_element, _element_ref)| {
                let handle_click_outside = {
                    let show_element = show_element.clone();
                    EventListener::new(&web_sys::window().unwrap(), "mousedown", move |_event| {
                        // TODO: Add condition here
                        show_element.set(false);
                    })
                };

                if **show_element {
                    handle_click_outside.forget();
                }

                let show_element = show_element.clone();
                move || {
                    if *show_element {
                        // The EventListener will automatically be removed when it is dropped
                    }
                }
            },
        );
    }

    (show_element, element_ref)
}
