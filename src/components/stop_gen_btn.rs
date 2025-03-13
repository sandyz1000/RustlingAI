use yew::prelude::*;


#[function_component]
pub(crate) fn StopGeneratingButton() -> Html {
  html! {
    <div
      class="absolute bottom-6 left-0 right-0 m-auto flex md:w-full md:m-auto gap-0 md:gap-2 justify-center"
      onclick={|e| {}}
    >
      <button
        class="btn relative btn-neutral border-0 md:border"
        aria-label="stop generating"
      >
        <div class="flex w-full items-center justify-center gap-2">
          <svg
            stroke="currentColor"
            fill="none"
            strokeWidth="1.5"
            viewBox="0 0 24 24"
            strokeLinecap="round"
            strokeLinejoin="round"
            class="h-3 w-3"
            height="1em"
            width="1em"
            xmlns="http://www.w3.org/2000/svg"
          >
            <rect x="3" y="3" width="18" height="18" rx="2" ry="2"></rect>
          </svg>
          {"Stop generating"}
        </div>
      </button>
    </div>
  }
}
