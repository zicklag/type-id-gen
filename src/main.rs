use type_safe_id::{DynamicType, TypeSafeId};
use web_sys::HtmlInputElement as InputElement;
use yew::html::Scope;
use yew::{html, Component, Context, Html, TargetCast};

// Define the possible messages which can be sent to the component
pub enum Msg {
    /// Copy the ID to the clipboard.
    Copy,
    /// Re-generate the ID
    Generate,
    /// Set the prefix of the ID
    SetPrefix(String),
}

type Id = TypeSafeId<DynamicType>;

pub struct App {
    prefix: String,
    id: Id, // This will store the counter value
}

impl App {
    fn generate(&mut self) {
        self.id = Id::new_with_type(
            DynamicType::new(&self.prefix).unwrap_or_else(|_| DynamicType::new("error").unwrap()),
        );
    }

    fn prefix_input(&self, link: &Scope<Self>) -> Html {
        let oninput = link.batch_callback(|e: yew::InputEvent| {
            let input: InputElement = e.target_unchecked_into();
            let value = input.value();
            let sanitized = value.to_ascii_lowercase().replace([' ', '-', '_'], "");
            input.set_value(&sanitized);
            Some(Msg::SetPrefix(sanitized))
        });
        html! {
            <input
                class="input"
                placeholder="prefix"
                {oninput}
            />
        }
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let prefix = String::new();
        let ty = DynamicType::new(&prefix).unwrap();
        Self {
            id: Id::new_with_type(ty),
            prefix,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Generate => {
                self.generate();
            }
            Msg::SetPrefix(s) => {
                self.prefix = s;
                self.generate();
            }
            Msg::Copy => {
                use wasm_bindgen::prelude::*;
                #[wasm_bindgen]
                extern "C" {
                    type Clipboard;

                    #[wasm_bindgen(js_namespace = ["navigator", "clipboard"], js_name = writeText)]
                    fn write_clipboard(s: &str);
                }

                write_clipboard(&self.id.to_string());
            }
        };
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="wrapper">
                { Html::from_html_unchecked(include_str!("welcome.html").into()) }

                { self.prefix_input(ctx.link()) }

                <p class="id" alt="click to copy" onclick={ctx.link().callback(|_| Msg::Copy)}>
                    { self.id.to_string() }
                </p>

                <button class="generate" onclick={ctx.link().callback(|_| Msg::Generate)}>
                { "Regenerate" }
                </button>
            </div>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
