/*
UTF RENDER by Alexander Abraham, a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

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
    let clipboard = use_clipboard();
    let result: UseStateHandle<String> = use_state(|| "Unicode sequence here.".to_owned());
    let unicode: UseStateHandle<String> = use_state(|| "Unicode sequence here.".to_owned());
    let onchange = {
        let unicode_cloned: UseStateHandle<String> = unicode.clone();
        Callback::from(
            move |event: Event| {
                let target: EventTarget = event.target().unwrap();
                let input: HtmlInputElement = target.unchecked_into::<HtmlInputElement>();
                unicode_cloned.set(input.value());
            }
        )
    };
    let onclick = {
        let result_clone = result.clone();
        let clipboard_clone = clipboard.clone();
        move |_| {
            clipboard_clone.write_text(convert_to_html(&unicode));
            result_clone.set(convert_to_html(&unicode));
        }
    };
    return html!{
        <>
         <input type="text" {onchange} placeholder={ "Unicode here." }/>
         <button {onclick}>{ "RENDER" }</button>
         <p class="rendered">{ format!("Rendered: {}", &result.clone().to_string()) }</p>
        </>
    };
}

// A short functional component
// that displays some info.
#[function_component]
pub fn InfoCog() -> Html {
    return html!{
        <>
         <h2>{ "What?" }</h2>
         <p>
          { "If you want fancy characters like in the heading for whatever, 
          these characters are part of the UTF-8 character standard. 
          Each character has a special unicode sequence assocociated with it. 
          To render this character out, you need the sequence. 
          This app helps you by rendering the sequence out for you to copy 
          and use for whatever you want." }
          <a href="https://unicode-table.com/">
          { "Here's a link" }
          </a>
          {" to a table for some popular UTF-8 characters and their 
          unicode sequences." }
         </p>
         <h2>{ "Tutorial" }</h2>
         <p>
          { "For example: If you see something like U+16910 for a character, 
          you copy that and type it into the input field. 
          A pentagram should be rendered out." }
         </p>
        </>
    };
}


// Our main "App" component
// that is rendered out in the DOM.
#[function_component]
pub fn App() -> Html {
    return html!{
        <>
        <div class="content">
         <div class="container">
          <HeadingCog/>
          <RenderCog/>
          <InfoCog/>
         </div>
        </div>
        <br/>
        <br/>
        </>
    };
}

// Main entry point for 
// the Rust compiler.
fn main() {
    yew::Renderer::<App>::new().render();
}
