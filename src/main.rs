use dioxus::prelude::*;
use wurdle::context::GameContext;
use wurdle::keyboard::Keyboard;
use wurdle::popup::Popup;

const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    launch(App);
}

#[component]
fn App() -> Element {
    let max_words: usize = 6;
    let wordlen: usize = 5;

    //prepopulate table :|

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
        if *gamecx.show_popup.read() {
            Popup {}
        }
        for row_index in 0..max_words {
            div {
                class: "row",
                for letter_index in 0..wordlen {
                    div {
                        background_color: "{table.get(row_index).unwrap().letters.get(letter_index).unwrap().color.read().as_color()}",
                        class: "letter",
                        "{table.get(row_index).unwrap().letters.get(letter_index).unwrap().value.read().unwrap_or(' ')}"
                    }
                }
                ""
            }
        }
    }
}
