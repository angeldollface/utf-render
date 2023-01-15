/*
UTF-8 RENDER by Alexander Abraham, a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

// We use Yew's APIs.
use yew::prelude::*;

// The functional component to display
// a footer.
#[function_component]
pub fn FooterCog() -> Html {
    return html!{
        <div class="footer">
         <p class="footer">
          { "Made with love! " }
          <a class="footer">
          <i class="fa fa-github footer"></i>
          </a>
         </p>
        </div>
    };
}