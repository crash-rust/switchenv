use crate::components::{HeaderLogo, SettingBox};
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div>
          <div class="layout-container">
           <div class="header">
            <HeaderLogo/>
           </div>
           <div class="main">
            <SettingBox/>
           </div>
          </div>
        </div>
    }
}
