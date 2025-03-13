use yew::{prelude::*, virtual_dom::VNode};

#[derive(Debug, Properties, PartialEq)]
pub(crate) struct TransProps {
  pub(crate) i18n_key: String,
  pub(crate) ns: String,
  #[prop_or_default]
  pub(crate) components: Vec<VNode>
}

#[function_component]
pub(crate) fn Trans(TransProps { i18n_key, ns, components }: &TransProps) -> Html {
    // Here we assume a `translate` function that fetches the localized string
    let translated_text = translate(i18n_key, ns); 

    // If components are provided, replace placeholders in the translated string
    let mut html_output = vec![];
    let mut component_iter = components.iter();

    for part in translated_text.split("{}") {
        html_output.push(html! { part });
        if let Some(component) = component_iter.next() {
            html_output.push(component.clone());
        }
    }

    html! { <>{ for html_output }</> }
}

// Mock function to simulate translation retrieval
fn translate(i18n_key: &str, ns: &str) -> String {
    format!("Translated text for key '{}' in namespace '{}'", i18n_key, ns)
}
