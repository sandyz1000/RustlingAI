use wasm_bindgen::closure::Closure;
use web_sys::KeyboardEvent;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PopupModalProps {
    #[prop_or("Information".to_string())]
    pub title: String,
    #[prop_or(None)]
    pub message: Option<String>,
    pub set_is_modal_open: Callback<bool>,
    #[prop_or(None)]
    pub handle_confirm: Option<Callback<()>>,
    #[prop_or(None)]
    pub handle_close: Option<Callback<()>>,
    #[prop_or(None)]
    pub handle_click_backdrop: Option<Callback<()>>,
    #[prop_or(true)]
    pub cancel_button: bool,
    pub children: Children,
}

#[function_component]
pub fn PopupModal(props: &PopupModalProps) -> Html {
    let on_close = {
        let set_is_modal_open = props.set_is_modal_open.clone();
        let handle_close = props.handle_close.clone();
        Callback::from(move |_e: MouseEvent| {
            if let Some(cb) = &handle_close {
                cb.emit(());
            }
            set_is_modal_open.emit(false);
        })
    };

    let on_backdrop_close = {
        let handle_click_backdrop = props.handle_click_backdrop.clone();
        let on_close = on_close.clone();
        Callback::from(move |e| {
            if let Some(cb) = &handle_click_backdrop {
                cb.emit(());
            } else {
                on_close.emit(e);
            }
        })
    };

    let on_key_down = {
        let handle_confirm = props.handle_confirm.clone();
        let handle_close = props.handle_close.clone();
        let set_is_modal_open = props.set_is_modal_open.clone();
        Callback::from(move |event: KeyboardEvent| match event.key().as_str() {
            "Escape" => {
                if let Some(cb) = &handle_close {
                    cb.emit(());
                } else {
                    set_is_modal_open.emit(false);
                }
            }
            "Enter" => {
                if let Some(cb) = &handle_confirm {
                    cb.emit(());
                }
            }
            _ => {}
        })
    };

    // Attach event listener for key events
    use_effect(|| {
        if let Some(win) = web_sys::window() {
            let doc = win.document().unwrap();
            let closure = Closure::wrap(Box::new(move |event: KeyboardEvent| {
                on_key_down.emit(event);
            }) as Box<dyn FnMut(_)>);
            // doc.add_event_listener_with_callback("keydown", closure.as_ref()).unwrap();
            closure.forget();
        }
        || {}
    });

    html! {
        <div class="fixed top-0 left-0 z-[999] w-full p-4 overflow-x-hidden overflow-y-auto h-full flex justify-center items-center">
            <div class="relative z-2 max-w-2xl md:h-auto flex justify-center max-h-full">
                <div class="relative bg-gray-50 rounded-lg shadow dark:bg-gray-700 max-h-full overflow-y-auto hide-scroll-bar">
                    <div class="flex items-center justify-between p-4 border-b rounded-t dark:border-gray-600">
                        <h3 class="ml-2 text-lg font-semibold text-gray-900 dark:text-white">{ &props.title }</h3>
                        <button
                            type="button"
                            class="text-gray-400 bg-transparent hover:bg-gray-200 hover:text-gray-900 rounded-lg text-sm p-1.5 ml-auto inline-flex items-center dark:hover:bg-gray-600 dark:hover:text-white"
                            onclick={on_close.clone()}
                            aria-label="close modal"
                        >
                            { "×" } // Replace with an SVG icon if needed
                        </button>
                    </div>

                    if let Some(msg) = &props.message {
                        <div class="p-6 border-b border-gray-200 dark:border-gray-600">
                            <div class="min-w-fit text-gray-900 dark:text-gray-300 text-sm mt-4">{ msg }</div>
                        </div>
                    }
                    { for props.children.iter() }

                    <div class="flex items-center justify-center p-6 gap-4">
                        if let Some(handle_confirm) = &props.handle_confirm {
                            <button
                                type="button"
                                class="btn btn-primary"
                                onclick={
                                    let _confirm = handle_confirm.clone();
                                    Callback::from(move |_e: MouseEvent| {
                                        _confirm.emit(())
                                    })
                                }
                                aria-label="confirm"
                            >
                                { "Confirm" }
                            </button>
                        }
                        if props.cancel_button {
                            <button
                                type="button"
                                class="btn btn-neutral"
                                onclick={on_close.clone()}
                                aria-label="cancel"
                            >
                                { "Cancel" }
                            </button>
                        }
                    </div>
                </div>
            </div>
            <div
                class="bg-gray-800/90 absolute top-0 left-0 h-full w-full z-[-1]"
                onclick={on_backdrop_close.clone()}
            />
        </div>
    }
}
