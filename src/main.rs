/*
UTF-8 RENDER by Alexander Abraham, a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

// We need this to verify formatting
// of unicode sequences.
use regex::Regex;

// We use Yew's APIs.
use yew::prelude::*;

// We need this to manufacture
// unicode strings.
use std::char::from_u32;

// We need this to work with
// events.
use wasm_bindgen::JsCast;

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


// Returns an emoji as a string from a UTF-8 number sequence.
pub fn render_from_string(subject: &String) -> String {
    let int = u32::from_str_radix(&subject, 16).unwrap();
    let char: char = from_u32(int).unwrap();
    return char.to_string();
}

// Checks whether the supplied characters is a
// UTF-8 string.
pub fn is_utf8(subject: &String) -> bool {
    let mut result: bool = false;
    let unicode_sequence_pattern: Regex = Regex::new(r"^U\+[0-9A-F]{4,6}$").unwrap();
    if unicode_sequence_pattern.is_match(subject) {
        result = true;
    }
    else {
        // Do nothing.
    }
    return result;
}

// Converts a full unicode sequence to an emoji string.
pub fn convert_to_html(subject: &String) -> String {
    let mut result: String = String::from("");
    for i in subject.split("+") {
        if i != String::from("U") {
            result = render_from_string(&String::from(i));
        }
        else {
            // Do nothing.
        }
    }
    return result;
}

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

// The functional component to accept a unicode
// sequence and render it as an emoji.
#[function_component]
pub fn RenderCog() -> Html {
    // We instantaite a stateful clipboard handler.
    let clipboard = use_clipboard();

    // We instantiate a stateful container for our result.
    let result: UseStateHandle<String> = use_state(|| "Rendered symbols will appear here.".to_owned());

    // We instantiate a stateful container for our unicode sequence.
    let unicode: UseStateHandle<String> = use_state(|| "Unicode sequence goes here.".to_owned());

    // A handler to constantly update our unicode sequence's state.
    let onchange = {

        // Cloning our unicode sequence so that we can mutate it in the DOM.
        let unicode_cloned: UseStateHandle<String> = unicode.clone();

        // Closure to handle changes. (In-line function.)
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

        // Closure to do something on a button press. (In-line function.)
        move |_| {

            // We check if the user's input is valid. If it is,
            // We render it and copy the symbol to the clipboard.
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

    // Inserting our input field and button
    // into the DOM.
    return html!{
        <>
         <input type="text" {onchange} placeholder={ "Unicode here." }/>
         <button {onclick}>{ "RENDER" }</button>
         <p class="rendered">{ format!("{}", &result.clone().to_string()) }</p>
        </>
    };
}

// A short functional component
// that displays some info.
#[function_component]
pub fn InfoCog() -> Html {

    // Inserting our info text into the DOM.
    return html!{
        <>
         <h2>{ "What?" }</h2>
         <p>
          { "Emojis and fancy symbols are part of the UTF-8 
          character standard (The unicode standard.). 
          Each character has a special unicode sequence assocociated with it. 
          To render this character out, you need the sequence. 
          This app helps you by rendering the sequence as a symbol for you to copy 
          and use for whatever you want." }
          <a href="https://unicode-table.com/">
          { " Here's a link" }
          </a>
          {" to a table for some popular UTF-8 characters and their 
          unicode sequences." }
         </p>
         <h2>{ "Tutorial" }</h2>
         <p>
          { "For example: If you see something like U+13080 for a character, 
          you copy that unicode sequence and type it into the input field. 
          The corresponding symbol should be rendered out.
          You can also paste that character anywhere 
          with your system's clipboard tools. This app copies the symbol automatically." }
         </p>
        </>
    };
}


// Our main "App" component
// that is rendered out in the DOM.
#[function_component]
pub fn App() -> Html {

    // Returning our full app, with
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
