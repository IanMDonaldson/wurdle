use crate::context::{GameContext, LetterColor, LetterState, Operation};
use crate::context::{Row, WonState};
use crate::wordlist::KEYWORDS;
use dioxus::hooks::use_context;
use dioxus::prelude::*;

pub fn get_winning_word() -> String {
    KEYWORDS
        .iter()
        .nth(rand::random_range(0..2307))
        .unwrap()
        .to_uppercase()
}

pub fn prepopulate_row(wordlen: usize) -> Vec<LetterState> {
    let mut letters: Vec<LetterState> = Vec::with_capacity(wordlen);
    for letter in 0..wordlen {
        letters.push(LetterState::default());
    }
    letters
}
pub fn prepopulate_table(max_words: usize, wordlen: usize) -> Vec<Row> {
    let mut table: Vec<Row> = Vec::with_capacity(max_words);
    for word in 0..max_words {
        table.push(Row::new(wordlen));
    }
    table
}
pub fn reset_table(max_words: usize, wordlen: usize) {
    let mut gamecx = use_context::<GameContext>();
    for row in 0..max_words {
        for letter in 0..wordlen {
            gamecx.change_letter_color(row, letter, LetterColor::Incorrect);
            gamecx.change_letter_val(row, letter, ' ');
        }
    }
    /*    uuuggghhhh
    for mut row in table.iter_mut() {
        for mut letter in row.letters.read().iter_mut() {
            letter.value.set(Option::None);
        }
    }*/
}

pub fn handle_key(letter: String) {
    //what's the point of global state if I can't fucking change it?? Jesus
    let mut gamecx = use_context::<GameContext>();
    let table = gamecx.table.read().cloned();
    let cur_row_index = gamecx.cur_row.read().cloned();
    let cur_letter_index = gamecx.cur_letter.read().cloned();
    let win_word = gamecx.winword.read().cloned();
    let winchars: Vec<char> = win_word.chars().collect();
    let mut show_popup = gamecx.show_popup;
    let mut message = gamecx.popup_message;
    let mut won = gamecx.won;
    // either done or prematurely hit enter
    // dont generate new vec, it's backed by an already pregenerated Vec<LetterState>
    let row = table.get(cur_row_index).unwrap().letters.read().cloned();
    let row2 = table.get(cur_row_index).unwrap();
    if letter.eq("⏎") {
        if cur_letter_index != win_word.len() {
            println!("Length not satisfied..");
        } else if win_word == *row2.get_as_str() {
            println!("You win!");
            for letter_index in 0..row.len() {
                gamecx.change_letter_color(cur_row_index, letter_index, LetterColor::Correct);
            }
            //uhh can I do a popup in here?
            message.set("You Win!".to_string());
            won.set(WonState::Won);
            show_popup.set(true);
        } else if !KEYWORDS.contains(&row2.get_as_str()) {
            won.set(WonState::NA);
            message.set(WonState::NA.as_message());
            show_popup.set(true);
        } else {
            // so just iterate over, changing the colors
            //should never panic, it's always gonna be populated
            for letter_index in 0..row.len() {
                let cur_letterstate = *row.get(letter_index).unwrap();
                let cur_letter = cur_letterstate.value.unwrap();

                if *winchars.get(letter_index).unwrap() == cur_letter {
                    gamecx.change_letter_color(cur_row_index, letter_index, LetterColor::Correct);
                } else if win_word.contains(cur_letter) {
                    gamecx.change_letter_color(cur_row_index, letter_index, LetterColor::WrongSpot);
                } else {
                    //it already had LetterColor::Incorrect when it was prepopulated ;)
                }
            }
            if cur_row_index == row.len() - 1 {
                println!("You lose");
                message.set("You Lose!".to_string());
                won.set(WonState::Lost);
                show_popup.set(true);
            }
            gamecx.inc_dec_curr_row(Operation::Increment);
            gamecx.zero_curr_letter();
        }
    } else if letter.eq("⌫") {
        //just remove a letter from row
        // actually no! needs to stay prepopulated since it's tied to view state..
        match cur_letter_index {
            0 => println!("backspace didn't remove a letter because it's empty"),
            _ => {
                println!(
                    "backspace removed {}",
                    row.get(cur_letter_index - 1).unwrap().value.read().unwrap()
                );
                gamecx.change_letterstate(
                    cur_row_index,
                    cur_letter_index - 1,
                    ' ',
                    LetterColor::Incorrect,
                );
                gamecx.inc_dec_curr_letter(Operation::Decrement);
            }
        }
    } else if cur_letter_index < win_word.len() {
        println!("the {} key was pressed", letter);
        gamecx.change_letterstate(
            cur_row_index,
            cur_letter_index,
            letter.chars().last().unwrap(),
            LetterColor::Incorrect,
        );
        gamecx.inc_dec_curr_letter(Operation::Increment);
    }
}

pub fn reset_game() {
    let gamecx = use_context::<GameContext>();
}
