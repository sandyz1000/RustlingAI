use yew::prelude::*;

#[derive(Debug, Properties, PartialEq)]
pub(crate) struct ToggleProps {
  pub(crate) is_checked: UseStateHandle<bool>,
  pub(crate) label: String
}

#[function_component]
pub(crate) fn Toggle(ToggleProps { is_checked, label }: &ToggleProps) -> Html {
  let on_change = {
    let is_checked = is_checked.clone();
    move |_e| {
      is_checked.set(!(*is_checked));
    }
  };
  html!{ 
    <label class="relative flex items-center cursor-pointer">
      <input
        type="checkbox"
        class="sr-only peer"
        checked={*(*is_checked)}
        onchange={ on_change }
      />
      <div class="w-9 h-5 bg-gray-200 dark:bg-gray-600 rounded-full peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-4 after:w-4 after:transition-all dark:border-gray-600 peer-checked:bg-green-500/70"></div>
      <span class="ml-3 text-sm font-medium text-gray-900 dark:text-gray-300">
        {label}
      </span>
    </label>
  }
}
