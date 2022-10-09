use serde::{Deserialize, Serialize};
use yew::prelude::*;

#[derive(PartialEq, Clone, Deserialize, Serialize)]
pub enum Env {
    Test,
    Prod,
}

#[derive(Properties, PartialEq)]
pub struct TabGroupProps {
    pub left_tab: String,
    pub right_tab: String,
    pub default_env: Env,
    pub onchange: Callback<Env>,
}

#[function_component(TabGroup)]
pub fn tab_group(
    TabGroupProps {
        left_tab,
        right_tab,
        default_env,
        onchange,
    }: &TabGroupProps,
) -> Html {
    let left_tab_class = use_state(|| "tab active".to_string());
    let right_tab_class = use_state(|| "tab".to_string());

    let selected_env = use_state(|| default_env.clone());

    let cloned_onchange_left = onchange.clone();
    let cloned_onchange_right = onchange.clone();

    let cloned_selected_env_left = selected_env.clone();
    let cloned_selected_env_right = selected_env.clone();

    let cloned_left_tab_class = left_tab_class.clone();
    let cloned_right_tab_class = right_tab_class.clone();

    let cloned_left_tab_class1 = left_tab_class.clone();
    let cloned_right_tab_class1 = right_tab_class.clone();

    let on_left_tab_click = Callback::from(move |_| {
        cloned_left_tab_class.set("tab active".to_string());
        cloned_right_tab_class1.set("tab".to_string());
        cloned_selected_env_left.set(Env::Test);
        cloned_onchange_left.emit(Env::Test)
    });
    let on_right_tab_click = Callback::from(move |_| {
        cloned_right_tab_class.set("tab active".to_string());
        cloned_left_tab_class1.set("tab".to_string());
        cloned_selected_env_right.set(Env::Prod);
        cloned_onchange_right.emit(Env::Prod);
    });

    html! {
        <div>
            <div class="tab-group">
                <div class={&*left_tab_class.clone()} onclick={on_left_tab_click.clone()}>{left_tab}</div>
                <div class={&*right_tab_class.clone()} onclick={on_right_tab_click.clone()}>{right_tab}</div>
            </div>
        </div>
    }
}
