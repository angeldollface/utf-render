/*
UTF-8 RENDER by Alexander Abraham, a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

// We use Yew's APIs.
use yew::prelude::*;

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