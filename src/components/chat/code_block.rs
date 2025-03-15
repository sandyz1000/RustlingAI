use web_sys::HtmlTextAreaElement;
use yew::prelude::*;
use yew_hooks::prelude::*;
use gloo_timers::callback::Timeout;

use crate::components::icons::{CopyIcon, TickIcon};


#[derive(Debug, Properties, PartialEq)]
pub struct CodeBlockProps {
    pub lang: String,
    pub children: Children
}

#[function_component]
pub fn CodeBlock(CodeBlockProps{lang, children}: &CodeBlockProps) -> Html {
    let code_ref = use_node_ref();
  
    html! {
        <div class="bg-black rounded-md">
            <CodeBar lang={lang.to_string()} code_ref={code_ref.clone()} />
            <div class="p-4 overflow-y-auto">
            <code ref={code_ref} class={classes!("whitespace-pre", "hljs", format!("language-{lang}"))}>
                { for children.iter() }
            </code>
            </div>
        </div>
    }    
}

#[derive(Debug, Properties, PartialEq)]
pub struct CodeBarProps {
    pub lang: String,
    pub code_ref: NodeRef,
}

#[function_component]
pub fn CodeBar(CodeBarProps{lang,code_ref}: &CodeBarProps) -> Html {
    let is_copied = use_state(|| false);
    let clipboard = use_clipboard();
    let on_click = {
        let code_ref = code_ref.clone();
        let is_copied = is_copied.clone();
        let clipboard = clipboard.clone();
        move |_e| {
            if let Some(d) = code_ref.cast::<HtmlTextAreaElement>() {
                let code = d.text_content();
                if code.is_some() {
                    clipboard.write_text("Your text to copy".to_owned());
                    is_copied.set(true);
                    let is_copied = is_copied.clone();
                    // Reset the state after 3 seconds
                    Timeout::new(3000, move || is_copied.set(false)).forget();
                }
            }
        }
    };
    html!{
        <div class="flex items-center relative text-gray-200 bg-gray-800 px-4 py-2 text-xs font-sans">
          <span class="">{lang}</span>
          <button
            class="flex ml-auto gap-2"
            aria-label="copy codeblock"
            onclick={on_click}
          >
            if *is_copied {
              <>
                <TickIcon />
                {"Copied!"}
              </>
            } else {
                <>
                <CopyIcon />
                {"Copy code"}
                </>
            }
          </button>
        </div>
    }
      
}
  