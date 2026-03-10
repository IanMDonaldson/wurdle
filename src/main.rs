use dioxus::document::Style;
use dioxus::prelude::*;
use rand::Rng;
use std::fs::{File, read_to_string};
use wurdle::utils::*;
use wurdle::keyboard::Keyboard;

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
    let mut guessed_words: Vec<String> = Vec::new();
    // let mut curr_word: String = String::new();
    let mut curr_letter: u8 = 0;
    let mut max_words: usize = 6;
    let mut wurd_length: usize = 5;
    let winning_word = get_winning_word();
    println!("here's the winning word {}", winning_word);
    // let check_answer: FnMut(&String) = |answer: &String| {
    //     println!("sssss");
    //     if answer.len() != wurd_length {}
    //     /*  if answerchar = winchar -> letter = green
    //         else {
    //             if winword.contains(answerchar) -> letter = yellow
    //             else -> letter = black
    //     */
    // };

    use_context_provider(|| WinWord(winning_word));
    use_context_provider(|| CurrWord(String::new()));
    use_context_provider(|| GuessedWords(Vec::new()));
    use_context_provider(|| WordLen(5));
    rsx! {
        document::Stylesheet { href: MAIN_CSS }
        AttemptsView { max_words,}
        Keyboard {}
        // WordAttempts { max_words, curr_word }
        // Keyboard {curr_word}
    }
}

#[component]
fn AttemptsView(max_words: usize,) -> Element {
    let guessed_words = use_context::<GuessedWords>().0;
    rsx! {
        for x in (0..max_words) {
            div {
                display: "flex",
                justify_content: "center",
                font_size: "24px",
                if let Some(s) = guessed_words.get(x) {
                    AttemptRow { word: s.to_owned() }
                } else {
                    AttemptRow { word: Vec::new()}
                }
            }
        }
    }
}

#[component]
fn AttemptRow(word: Vec<LetterState>) -> Element {
    let wurd_len = use_context::<WordLen>();
    rsx! {
        if word.len() == 0 {
            for x in (0..wurd_len.0) {
                 div {
                    // class: "default_letter",
                    background_color: LetterColor::Incorrect.as_color(),
                    display: "block",
                    border_style: "solid",
                    width: "30px",
                    height: "50px",
                    font_size: "48px",
                    color: "white",
                    border_radius: "5px",
                    padding: "0.2% 3%",
                    margin: "5px 5px",
                    ""
                }
            }
        } else {
            for x in (0..wurd_len.0) {
                div {
                    // class: "default_letter",
                    background_color: "word.get(x).unwrap().color.as_color()",
                    display: "block",
                    border_style: "solid",
                    width: "30px",
                    height: "50px",
                    font_size: "48px",
                    color: "white",
                    border_radius: "5px",
                    padding: "0.2% 3%",
                    margin: "5px 5px",
                    "{word.get(x).unwrap().value}"
                }
            }
        }
    }

}

// #[component]
// fn Letter(letter_state: LetterState, letter: char) -> Element {
//     rsx! {
//
//     }
// }


