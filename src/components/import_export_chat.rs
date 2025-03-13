use yew::prelude::*;
use crate::components::icons::ExportIcon;
use crate::hooks::translation::use_translation;
use crate::components::popup_modal::PopupModal;

#[function_component]
pub(crate) fn ImportExportChat() -> Html {
  let t = use_translation(vec![]);
  let is_modal_open = use_state(|| false);
  let set_is_modal_open = {
    let is_modal_open = is_modal_open.clone();
    Callback::from(|value: bool| {
      is_modal_open.set(value);
    })
  };
  html! {
    <>
      <a
        class="flex py-2 px-2 items-center gap-3 rounded-md hover:bg-gray-500/10 transition-colors duration-200 text-white cursor-pointer text-sm"
        onclick={|e| {} }
      >
        <ExportIcon />
        {format!("{} / {}", t("import".to_string(), None), t("export".to_string(), None))}
      </a>
      if *is_modal_open {
        <PopupModal
          title={format!("{} / {}", t("import".to_string(), None), t("export".to_string(), None))}
          set_is_modal_open={set_is_modal_open}
          cancel_button={false}
        >
          <div class="p-6 border-b border-gray-200 dark:border-gray-600">
            <ImportChat />
            <ExportChat />
            <div class="border-t my-3 border-gray-200 dark:border-gray-600" />
            <ImportChatOpenAI set_is_modal_open={set_is_modal_open} />
          </div>
        </PopupModal>
      }
    </>
  }
}
