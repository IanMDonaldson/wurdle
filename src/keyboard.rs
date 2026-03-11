use dioxus::prelude::*;
use crate::utils::{handle_key};

#[component]
pub fn Keyboard(/*curr_word: Signal<CurrWord>*/) -> Element {
    let keyboard_rows = vec![
        "QWERTYUIOP".chars().map(|s| s.to_string()).collect(),
        "ASDFGHJKL".chars().map(|s| s.to_string()).collect(),
        "⏎ZXCVBNM⌫".chars().map(|s| s.to_string()).collect(),
    ];
    rsx! {
        div {
            display: "flex",
            flex_direction: "column",
            for row in keyboard_rows {
                KeyboardRow {/*letter_handle, */row }
            }
        }
    }
}

#[component]
fn KeyboardRow(/*letter_handle: Signal<String>,*/ row: Vec<String>) -> Element {
    rsx! {
        div {
            display: "flex",
            flex_direction: "row",
            justify_content: "center",
            for letter in row {
                KeyboardButton {/*letter_handle,*/ letter}
            }

        }
    }
}

#[component]
fn KeyboardButton(/*letter_handle: Signal<String>,*/ letter: String) -> Element {

    rsx! {
        button {
            display: "flex",
            flex_direction: "row",
            margin_left: if letter.eq("A") {"1rem"} else {""},
            color: "white",
            background_color: "#a8a8a8",
            font_size: "1.2rem",
            padding: "1rem",
            margin: "5px",
            max_width: "2rem",
            onclick: move |_| {
                handle_key(letter.clone()/*, post_action_word, win_word*/);
            },
            "{letter}"
        }
    }
}
