use gloo_timers::callback::Timeout;
use yew::prelude::*;
use yewdux::use_store;

use crate::components::icons::{CheckIcon, CloseIcon, WarningIcon, ErrorIcon};
use crate::store::ToastSlice;

#[derive(Debug, PartialEq, Default, Clone)]
pub(crate) enum ToastStatus {
    #[default]
    Success,
    Error,
    Warning,
}

#[function_component]
pub(crate) fn Toast() -> Html {
    let (toast_slice, toast_dispatch) = use_store::<ToastSlice>();
    let timeout_id = use_state(|| 0.0);
    let set_toast_show = move |val: bool| {
        toast_dispatch.reduce_mut_callback::<_, (), ()>(move |s| s.show = val);
    };
    {
        let set_toast_show = set_toast_show.clone();
        let toast_slice = toast_slice.clone();
        let timeout_id = timeout_id.clone();
        use_effect_with(toast_slice, move |toast_slice| {
            if toast_slice.show {
                let timeout = Timeout::new(5000, move || set_toast_show(false)).forget();
                let timeout = timeout.as_f64().unwrap();
                timeout_id.set(timeout);
            }
        });
    }
    html! {
      <div
        class="flex fixed right-5 bottom-5 z-[1000] items-center w-3/4 md:w-full max-w-xs p-4 mb-4 text-gray-500 dark:text-gray-400 rounded-lg shadow-md border border-gray-400/30 animate-bounce"
        role="alert"
      >
        <StatusIcon status={toast_slice.status.clone()} />
        <div class="ml-3 text-sm font-normal">{toast_slice.message.clone()}</div>
        <button
          type="button"
          class="ml-auto -mx-1.5 -my-1.5 bg-white text-gray-400 hover:text-gray-900 rounded-lg focus:ring-2 focus:ring-gray-300 p-1.5 hover:bg-gray-100 inline-flex h-8 w-8 dark:text-gray-500 dark:hover:text-white dark:bg-gray-800 dark:hover:bg-gray-700"
          aria-label="Close"
          onclick={  Callback::from(move |_e| set_toast_show(false)) }
        >
          <CloseIcon />
        </button>
      </div>
    }
}

#[derive(Debug, Properties, PartialEq)]
pub struct StatusIconProps {
    pub status: ToastStatus,
}

#[function_component]
pub fn StatusIcon(StatusIconProps { status }: &StatusIconProps) -> Html {
    match status {
        ToastStatus::Success => html! {<CheckIcon />},
        ToastStatus::Warning => html! {<WarningIcon />},
        ToastStatus::Error => html! {<ErrorIcon />},
    }
}
