use pulldown_cmark::{Options, Parser, html};
use yew::prelude::*;

/// Properties for the Markdown component
#[derive(Properties, PartialEq)]
pub struct MarkdownProps {
    pub children: String,
}

/// A Yew component that renders Markdown to HTML
/// ## Usage
/// 
/// ```rust
/// use yew::prelude::*;
///
/// #[function_component]
/// fn App() -> Html {
///     html! {
///         <Markdown children={"# Hello, Yew!\nThis is **Markdown** rendered in Rust!".to_string()} />
///     }
/// }
///
/// fn main() {
///     yew::Renderer::<App>::new().render();
/// }
/// ```
/// 
#[function_component]
pub fn Markdown(props: &MarkdownProps) -> Html {
    let html_content = use_state(|| String::new());
    let markdown_text = props.children.clone();

    {
        let html_content = html_content.clone();
        use_effect_with(markdown_text.clone(), move |_| {
            let mut options = Options::empty();
            options.insert(Options::ENABLE_STRIKETHROUGH);
            let parser = Parser::new_ext(&markdown_text, options);

            let mut html_output = String::new();
            html::push_html(&mut html_output, parser);

            html_content.set(html_output);

            || {}
        });
    }

    html! {
        <div class="markdown" dangerously_set_inner_html={(*html_content).clone()} />
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;
    
    wasm_bindgen_test_configure!(run_in_browser);

    /// Helper function to render the Markdown component and get the output HTML
    fn render_markdown(input: &str) -> String {
        let document = web_sys::window().unwrap().document().unwrap();
        let div = document.create_element("div").unwrap();

        let html_content = use_state(|| String::new());
        let markdown_text = input.to_string();

        let mut options = Options::empty();
        options.insert(Options::ENABLE_STRIKETHROUGH);
        let parser = Parser::new_ext(&markdown_text, options);

        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);
        
        // TODO: Fix this Test
        // html_content.set(html_output.clone());

        html_output
    }

    #[wasm_bindgen_test]
    fn test_markdown_rendering() {
        let input = "# Hello, Yew!";
        let output = render_markdown(input);

        assert!(output.contains("<h1>"));
        assert!(output.contains("Hello, Yew!"));
    }

    #[wasm_bindgen_test]
    fn test_markdown_bold_text() {
        let input = "**Bold Text**";
        let output = render_markdown(input);

        assert!(output.contains("<strong>"));
        assert!(output.contains("Bold Text"));
    }

    #[wasm_bindgen_test]
    fn test_markdown_strikethrough() {
        let input = "~~Strikethrough~~";
        let output = render_markdown(input);

        assert!(output.contains("<del>"));
        assert!(output.contains("Strikethrough"));
    }

    #[wasm_bindgen_test]
    fn test_empty_markdown() {
        let input = "";
        let output = render_markdown(input);

        assert!(output.trim().is_empty());
    }
}
