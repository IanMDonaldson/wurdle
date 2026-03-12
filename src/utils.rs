use crate::context::Row;
use std::fs::read_to_string;
use std::process::exit;
use dioxus::hooks::use_context;
use dioxus::prelude::*;
use crate::context::{GameContext, LetterColor, LetterState, Operation};

pub fn get_winning_word() -> String {
    let wordle_list = read_to_string("wordle-list").unwrap();
    let winvec = Vec::from_iter(wordle_list.lines());
    let winningword = winvec.get(rand::random_range(0..2307)).unwrap().to_string().to_uppercase();
    winningword
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

pub fn handle_key(letter: String) {
    //what's the point of global state if I can't fucking change it?? Jesus
    let mut gamecx = use_context::<GameContext>();
    let  table = gamecx.table.read().cloned();
    let  cur_row_index = gamecx.cur_row.read().cloned();
    let cur_letter_index = gamecx.cur_letter.read().cloned();
    let  win_word = gamecx.winword.read().cloned();
    let  winchars: Vec<char> = win_word.chars().collect();
    // either done or prematurely hit enter
    // dont generate new vec, it's backed by an already pregenerated Vec<LetterState>
    let  row = table.get(cur_row_index).unwrap().letters.read().cloned();
    let row2 = table.get(cur_row_index).unwrap();
    if letter.eq("⏎") {
        if cur_letter_index != win_word.len() {
            println!("Length not satisfied..");
        } else if win_word == *row2.get_as_str() {
            println!("You win!");
            exit(1);
        } else {
            // so just iterate over, changing the colors
            //should never panic, it's always gonna be populated
            for letter_index in 0..row.len() {
                let cur_letterstate = *row.get(letter_index).unwrap();
                let cur_letter = cur_letterstate.value.unwrap();

                if *winchars.get(letter_index).unwrap() == cur_letter {
                    gamecx.change_letter_color(
                        cur_row_index, letter_index,
                        LetterColor::Correct,
                        );
                } else if win_word.contains(cur_letter) {
                    gamecx.change_letter_color(
                        cur_row_index, letter_index,
                            LetterColor::WrongSpot
                        );
                } else {
                    //it already had LetterColor::Incorrect when it was prepopulated ;)
                }
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
                println!("backspace removed {}", row.get(cur_letter_index-1).unwrap().value.read().unwrap());
                gamecx.change_letterstate(
                    cur_row_index, cur_letter_index -1,
                    ' ', LetterColor::Incorrect
                );
                gamecx.inc_dec_curr_letter(Operation::Decrement);
            }
        }
    } else if cur_letter_index < win_word.len() {
        println!("the {} key was pressed", letter);
        gamecx.change_letterstate(
            cur_row_index, cur_letter_index,
            letter.chars().last().unwrap(), LetterColor::Incorrect
        );
        gamecx.inc_dec_curr_letter(Operation::Increment);
    }
}




