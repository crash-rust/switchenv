use yew::prelude::*;

#[function_component(HeaderLogo)]
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
