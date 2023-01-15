/*
UTF-8 RENDER by Alexander Abraham, a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

// We declare
// a module for the
// components in "modules".
mod modules;

// We use Yew's APIs.
use yew::prelude::*;

// Importing the Information Cog
// from "modules/InfoCog.rs".
use modules::InfoCog::InfoCog;

// Importing the Render Cog
// from "modules/RenderCog.rs".
use modules::RenderCog::RenderCog;

// Importing the Heading Cog
// from "modules/HeadingCog.rs".
use modules::HeadingCog::HeadingCog;

// Our main "App" component
// that is rendered out in the DOM.
#[function_component]
pub fn App() -> Html {

    // Returning our full app with
    // all components united in one
    // parent component.
    return html!{
        <>
         <br/>
         <br/>
         <div class="content">
          <br/>
          <div class="container">
           <HeadingCog/>
           <RenderCog/>
           <InfoCog/>
          </div>
          <br/>
         </div>
         <br/>
         <br/>
        </>
    };
}

// Main entry point for 
// the Rust compiler.
fn main() {

    // We render the app.
    yew::Renderer::<App>::new().render();
}
