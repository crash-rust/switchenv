use stylist::yew::styled_component;
use yew::prelude::*;

#[styled_component(HeaderLogo)]
pub fn header_logo() -> Html {
    html! {
      <div>
        <div class="header-logo-wrapper font-headline-medium ">
          <img class="header-logo" alt="logo"  src="images/enmac-logo.svg"/>
          <img class="header-text" alt="text" src="images/enmac-text.svg"/>
      </div>
      </div>
    }
}
