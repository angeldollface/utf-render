/*
UTF-8 RENDER by Alexander Abraham, a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

// We use Yew's APIs.
use yew::prelude::*;

// We need this to work with
// events.
use wasm_bindgen::JsCast;

use super::utils::*;

// We need this to
// copy the UTF-8 character
// into the clipboard.
use yew_hooks::prelude::*;

// We need this to capture event
// results.
use web_sys::EventTarget;

// We need this to interact
// with the HTML "input"
// element.
use web_sys::HtmlInputElement;

// The functional component to accept a unicode
// sequence and render it as an emoji.
#[function_component]
pub fn RenderCog() -> Html {

    // We instantiate a stateful clipboard handler.
    let clipboard = use_clipboard();

    // We instantiate a stateful container for our result.
    let result: UseStateHandle<String> = use_state(|| "Rendered symbols will appear here.".to_owned());

    // We instantiate a stateful container for our unicode sequence.
    let unicode: UseStateHandle<String> = use_state(|| "Unicode sequence goes here.".to_owned());

    // A handler to constantly update our unicode sequence's state.
    let onchange = {

        // Cloning our unicode sequence so that we can mutate it in the DOM.
        let unicode_cloned: UseStateHandle<String> = unicode.clone();

        // Closure to handle changes. (Inline function.)
        Callback::from(
            move |event: Event| {

                // Getting the target's properties.
                let target: EventTarget = event.target().unwrap();

                // Unwrapping it into an HTML Input element.
                let input: HtmlInputElement = target.unchecked_into::<HtmlInputElement>();

                // Setting the unicode sequence to the input's value.
                unicode_cloned.set(input.value());
            }
        )
    };

    // A handler to render our symbol when the button is clicked.
    let onclick = {

        // Cloning our result so that we can mutate it in the DOM.
        let result_clone = result.clone();

        // Cloning our clipboard handler so that we can mutate it and write to it.
        let clipboard_clone = clipboard.clone();

        // Closure to render the symbol on a button press. (Inline function.)
        move |_| {

            // We check if the user's input is valid. If it is,
            // we render it and copy the symbol to the clipboard.
            if is_utf8(&unicode) {
                let return_msg: String = format!("Copied and rendered: {}", convert_to_html(&unicode));
                clipboard_clone.write_text(convert_to_html(&unicode));
                result_clone.set(return_msg);
            }

            // If it isn't, we return an error message.
            else {
                let return_msg: String = String::from("Invalid unicode sequence!");
                result_clone.set(return_msg);
            }
        }
    };

    // Inserting our input field, button,
    // and feedback element into the DOM.
    return html!{
        <>
         <input type="text" {onchange} placeholder={ "Unicode here." }/>
         <button {onclick}>{ "RENDER" }</button>
         <p class="rendered">{ format!("{}", &result.clone().to_string()) }</p>
        </>
    };
}