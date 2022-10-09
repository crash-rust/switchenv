use stylist::yew::styled_component;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TextInputProps {
    pub max_length: String,
}

#[styled_component(TextInput)]
pub fn text_input(TextInputProps { max_length }: &TextInputProps) -> Html {
    html! {
        <div>
          <input type="text" class="text-input" maxlength={max_length.clone()}/>
        </div>
    }
}
