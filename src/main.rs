use dioxus::document::Style;
use dioxus::prelude::*;
use rand::Rng;
use std::fs::{File, read_to_string};
use wurdle::context::GameContext;
use wurdle::keyboard::Keyboard;
use wurdle::utils::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");


fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut won = false;
    let max_words: usize = 6;
    let wordlen: usize = 5;
    let winning_word = get_winning_word();
    println!("here's the winning word {}", winning_word);

    //prepopulate table :|

    /* apparently not idiomatiic rust according to dioxus :)
    let table: Signal<Vec<Vec<LetterState>>> = use_signal(||
        prepopulate_table(max_words, wordlen)
    );
    let cur_row: Signal<usize> = use_signal(|| 0);
    let cur_letter = use_signal(|| 0);*/

    use_context_provider(|| GameContext::new(max_words, wordlen));
    rsx! {
        Stylesheet { href: MAIN_CSS }
        TableView {}
        Keyboard {}
    }
}

#[component]
fn TableView() -> Element {
    let gamecx = use_context::<GameContext>();
    let max_words = gamecx.max_words;
    let wordlen = gamecx.wordlen;
    let table = gamecx.table.read();

    rsx! {
        Stylesheet { href: MAIN_CSS }
        for row_index in 0..max_words {
            div {
                class: "row",
                for letter_index in 0..wordlen {
                    div {
                        background_color: "{table.get(row_index).unwrap().letters.get(letter_index).unwrap().color.as_color()}",
                        class: "letter",
                        "{table.get(1).unwrap().letters.get(2).unwrap().value.unwrap_or(' ')}"
                    }
                }
                ""
            }
        }
    }
}

