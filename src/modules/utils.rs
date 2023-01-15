/*
UTF-8 RENDER by Alexander Abraham, a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

// We need this to verify formatting
// of unicode sequences.
use regex::Regex;

// We need this to manufacture
// unicode strings.
use std::char::from_u32;

// Returns an emoji as a string from a UTF-8 number sequence.
pub fn render_from_string(subject: &String) -> String {
    let int = u32::from_str_radix(&subject, 16).unwrap();
    let char: char = from_u32(int).unwrap();
    return char.to_string();
}

// Checks whether the supplied characters are a
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