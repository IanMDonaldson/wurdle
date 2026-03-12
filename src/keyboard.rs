use crate::utils::handle_key;
use dioxus::prelude::*;

#[component]
pub fn Keyboard() -> Element {
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
                KeyboardRow { row }
            }
        }
    }
}

#[component]
fn KeyboardRow(row: Vec<String>) -> Element {
    rsx! {
        div {
            display: "flex",
            flex_direction: "row",
            justify_content: "center",
            for letter in row {
                KeyboardButton {letter}
            }
        }
    }
}

#[component]
fn KeyboardButton(letter: String) -> Element {
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
            max_width: "6rem",
            onclick: move |_| {
                handle_key(letter.clone());
            },
            "{letter}"
        }
    }
}
