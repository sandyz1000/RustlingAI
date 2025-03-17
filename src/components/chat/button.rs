use gloo_timers::callback::Timeout;
use yew::prelude::*;
use yewdux::use_store;

use crate::{components::icons::{CopyIcon, DeleteIcon, DownChevronArrow, EditIcon2, FileTextIcon, MarkdownIcon, RefreshIcon, TickIcon}, store::slice::ConfigSlice};

#[derive(Debug, Properties, PartialEq)]
pub struct BaseButtonProps {
    pub on_click: Callback<MouseEvent>,
    pub icon: Html,
    #[prop_or_default]
    pub button_props: AttrValue,
}

#[function_component]
pub fn BaseButton(BaseButtonProps {on_click, icon, button_props}: &BaseButtonProps) -> Html {
    let icon = icon.clone();
    let button_props = button_props.clone();
    html!{ 
        <div class="text-gray-400 flex self-end lg:self-center justify-center gap-3 md:gap-4  visible">
            <button
                class={classes!("p-1", "rounded-md", "hover:bg-gray-100", "hover:text-gray-700", "dark:text-gray-400", "dark:hover:bg-gray-700", "dark:hover:text-gray-200", "disabled:dark:hover:text-gray-400", "md:invisible", "md:group-hover:visible")}
                onclick={on_click.clone()}
                { button_props }
            >
                { icon }
            </button>
        </div>
    }
}

#[derive(Debug, Properties, PartialEq)]
pub struct ButtonProps {
    pub(crate) set_is_edit: Callback<bool>
}

#[function_component]
pub fn EditButton(ButtonProps { set_is_edit }: &ButtonProps) -> Html {
    let on_click = {
        let set_is_edit = set_is_edit.clone(); 
        move |_e| set_is_edit.emit(true) 
    };
    html! {
        <BaseButton 
        icon={ html! { <EditIcon2 /> } }
        button_props={ AttrValue::from("aria-label=\"edit message\"") }
        on_click={ on_click }
        />
    }
}

#[derive(Debug, Properties, PartialEq)]
pub struct DeleteBtnProps {
    pub set_is_delete: Callback<bool>
}

#[function_component]
pub fn DeleteButton(DeleteBtnProps {set_is_delete}: &DeleteBtnProps) -> Html {
    let on_click = {
        let set_is_delete = set_is_delete.clone();
        move |_| set_is_delete.emit(true)
    };
    html! {
        <BaseButton 
            icon={ html! { <DeleteIcon /> } }
            button_props={ AttrValue::from("aria-label=\"delete message\"") }
            on_click={ on_click }
        />
    }
}

#[function_component]
pub fn MarkdownModeButton() -> Html {
    let (store, _) = use_store::<ConfigSlice>();
    let markdown_mode = use_state(|| store.markdown_mode);
    let icon = if *markdown_mode { html! {<MarkdownIcon />} } else { html!{<FileTextIcon />} };
    html! {
        <BaseButton 
            icon={ icon } 
            button_props={ AttrValue::from("aria-label=\"toggle markdown mode\"") }
            on_click={ let markdown_mode = markdown_mode.clone(); move |_| markdown_mode.set(!*markdown_mode) }
        />
    }
}

#[derive(Debug, Properties, PartialEq)]
pub struct OnClickProps {
    pub on_click: Callback<MouseEvent>
}

#[function_component]
pub fn RefreshButton(OnClickProps {on_click}: &OnClickProps) -> Html {
    let on_click = on_click.clone();
    html! {
        <BaseButton 
            icon={ html! {<RefreshIcon />} } 
            button_props={ AttrValue::from("aria-label=\"regenerate message\"") }
            on_click={ on_click }
        />
    }
}
#[function_component]
pub fn UpButton(OnClickProps {on_click}: &OnClickProps) -> Html {
    let on_click= on_click.clone();
    html! {
        <BaseButton 
            icon={html! { <DownChevronArrow class_name="rotate-180" /> } }
            button_props={ AttrValue::from("aria-label=\"regenerate message\"") }
            on_click={ on_click }
        />
    }
}

#[function_component]
pub fn DownButton(OnClickProps {on_click}: &OnClickProps) -> Html {
    
    html! {
        <BaseButton 
        icon={ html! { <DownChevronArrow /> } }
        button_props={ AttrValue::from("aria-label=\"shift message down\"") }
        on_click={ on_click }
        />
    }
}

#[function_component]
pub fn CopyButton(OnClickProps {on_click}: &OnClickProps) -> Html {
    let is_copied = use_state(|| false);
    let icon = if *is_copied {html! { <TickIcon /> } } else { html! { <CopyIcon /> }};
    let on_click_this = {
        let is_copied  = is_copied.clone();
        let on_click = on_click.clone();
        move |e| {
            on_click.emit(e);
            is_copied.set(true);
            let is_copied = is_copied.clone();
            let _ = Timeout::new(3000, move || is_copied.set(false));
        }
    };
    html! {
        <BaseButton 
            icon={ icon }
            button_props={ AttrValue::from("aria-label=\"shift message down\"") }
            on_click={ on_click_this }
        />
    }
}
