/*
UTF-8 RENDER by Alexander Abraham, a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

// We use Yew's APIs.
use yew::prelude::*;

// The functional component to display
// a heading.
#[function_component]
pub fn HeadingCog() -> Html {
    return html!{
        <>
         <h1>{ "\u{16910} UTF-8 Render \u{16910}" }</h1>
        </>
    };
}