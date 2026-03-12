use std::fmt;
use std::fmt::Formatter;
use dioxus::prelude::{ReadableExt, ReadableOptionExt, ReadableVecExt, Signal, WritableExt, WritableVecExt};
use crate::utils::{get_winning_word, prepopulate_row, prepopulate_table};


#[derive(Clone, Copy, PartialEq)]
pub struct Row {
    pub letters: Signal<Vec<LetterState>>
}
impl Row {
    pub fn new(wordlen: usize) -> Row {
        Row {
            letters: Signal::new(prepopulate_row(wordlen))
        }
    }
    pub fn get_as_str(&self) -> String {
        self.letters.iter().map(|x|
            x.value.read().unwrap()
        ).collect::<String>()
    }
}
#[derive(Clone, Default, PartialEq)]
pub struct GameContext {
    pub table:  Signal<Vec<Row>>,
    pub cur_row: Signal<usize>,
    pub cur_letter: Signal<usize>,
    pub winword: Signal<String>,
    pub max_words: usize,
    pub wordlen: usize,
}
impl GameContext {
    pub fn new(max_words: usize, wordlen: usize) -> GameContext {
        let winning_word = get_winning_word();
        println!("This is the winning word: {}", winning_word);
        GameContext {
            table: Signal::new(prepopulate_table(max_words, wordlen)),
            cur_row: Signal::new(0),
            cur_letter: Signal::new(0),
            winword: Signal::new(winning_word),
            max_words,
            wordlen

        }
    }

    pub fn change_letterstate(&mut self, row_index: usize,
      letter_index: usize, letter: char, color: LetterColor
    ) {
        // What the hell? can't do let row.. let letterstate...have to do it in one big line
        self.table.get_mut(row_index).unwrap().letters.get_mut(letter_index).unwrap().color.set(color);
        self.table.get_mut(row_index).unwrap().letters.get_mut(letter_index).unwrap().value.set(Some(letter));

    }
    pub fn change_letter_color(&mut self, row_index: usize,
        letter_index: usize, color: LetterColor
    ) {
        self.table.get_mut(row_index).unwrap().letters.get_mut(letter_index).unwrap().color.set(color);
    }

    pub fn inc_dec_curr_row(&mut self, operation: Operation) {
        match operation {
            Operation::Increment => self.cur_row += 1,
            Operation::Decrement => self.cur_row -= 1,
        }
    }

    pub fn inc_dec_curr_letter(&mut self, operation: Operation) {
        match operation {
            Operation::Increment => self.cur_letter += 1,
            Operation::Decrement => self.cur_letter -= 1,
        }
    }
    pub fn zero_curr_letter(&mut self) {
        self.cur_letter.set(0);
    }
}
pub enum Operation {
    Decrement,
    Increment,
}
pub enum RowOrLetter {
    Row,
    Letter,
}
#[derive(Clone, Copy, PartialEq)]
pub struct LetterState {
    pub value: Signal<Option<char>>,
    pub color: Signal<LetterColor>,
    // pub value: Option<char>,
    // pub color: LetterColor,
}

impl LetterState {
    pub fn new(value: char, color: LetterColor) -> LetterState {
        LetterState {
            value: Signal::new(Some(value)),
            color: Signal::new(color),
            // value: Some(value),
            // color: color,
        }
    }
    pub fn default() -> LetterState {
        LetterState {
            value: Signal::new(None),
            color: Signal::new(LetterColor::Incorrect),
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum LetterColor {
    Incorrect,
    WrongSpot,
    Correct,
}
impl LetterColor {
    pub fn as_color(&self) -> &str {
        match self {
            LetterColor::Incorrect => "#7d7d7d",
            LetterColor::WrongSpot => "#c9c544",
            LetterColor::Correct => "#32a852",
        }
    }
}
impl fmt::Display for LetterState {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.value)
    }
}
