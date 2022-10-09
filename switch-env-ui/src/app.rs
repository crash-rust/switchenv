use crate::components::{HeaderLogo, SettingBox};
use stylist::yew::styled_component;
use yew::prelude::*;

#[styled_component(App)]
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
