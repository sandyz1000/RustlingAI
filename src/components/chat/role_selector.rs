use std::str::FromStr;

use yew::prelude::*;
use yewdux::use_store;

use crate::{
    components::icons::DownChevronArrow,
    hooks::{hide_on_outside::use_hideon_outside_click, translation::use_translation},
    store::slice::{ChatSlice, InputSlice},
    types::chat::Role,
};

pub const ROLES: &[&str] = &["user", "assistant", "system"];

#[derive(Debug, Properties, PartialEq)]
pub struct RoleSelectorProps {
    pub role: Role,
    pub message_index: i32,
    pub sticky: bool,
}

#[function_component]
pub fn RoleSelector(
    RoleSelectorProps {
        role,
        message_index,
        sticky,
    }: &RoleSelectorProps,
) -> Html {
    let (dropdown, dropdown_ref) = use_hideon_outside_click();
	let (store, _) = use_store::<InputSlice>();
	let input_role = use_state(|| store.input_role.clone());
    let (_state, _dispatch) = use_store::<ChatSlice>();
    let t = use_translation(vec![]);
    let on_click_me = {
		let dropdown = dropdown.clone();
		let input_role = input_role.clone();
		let sticky = *sticky;
		let message_index = *message_index as usize;
		let _dispatch = _dispatch.clone();
		move |r| {
			if !sticky {
				_dispatch.reduce_mut(|s| s.chats[s.curr_chat_index as usize].messages[message_index].role = Role::from_str(r).unwrap())
			} else {
				input_role.set(Role::from_str(r).unwrap());
			}
			dropdown.set(false);
        }
    };
    html! {
        <div class="prose dark:prose-invert relative">
        <button
          class="btn btn-neutral btn-small flex gap-1"
          aria-label={t(role.to_string(), None)}
          type="button"
          onclick={let dropdown = dropdown.clone(); move |_e| dropdown.set(!*dropdown) }
        >
          {t(role.to_string(), None)}
          <DownChevronArrow />
        </button>
        <div
          ref={dropdown_ref}
          id="dropdown"
          class={classes!("absolute", "top-100", "bottom-100", "z-10", "bg-white", "rounded-lg", "shadow-xl", "border-b", "border-black/10", "dark:border-gray-900/50", "text-gray-800", "dark:text-gray-100", "group", "dark:bg-gray-800", "opacity-90", if *dropdown {""} else {"hidden"})}
        >
          <ul
            class="text-sm text-gray-700 dark:text-gray-200 p-0 m-0"
            aria-labelledby="dropdownDefaultButton"
          >
            {
                ROLES.iter().map(|r| {
                    html! {
                        <li
                            class="px-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-600 dark:hover:text-white cursor-pointer"
                            onclick={ let on_click_me = on_click_me.clone(); move |_e: MouseEvent| on_click_me(*r) }
                            key={*r}
                        >
                        {t(r.to_string(), None)}
                        </li>
                    }
                }).collect::<Html>()
            }
          </ul>
        </div>
      </div>
    }
}
