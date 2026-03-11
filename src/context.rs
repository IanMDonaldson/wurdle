use std::fmt;
use std::fmt::Formatter;
use dioxus::prelude::{ReadableVecExt, Signal, WritableVecExt};
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
        GameContext {
            table: Signal::new(prepopulate_table(max_words, wordlen)),
            cur_row: Signal::new(0),
            cur_letter: Signal::new(0),
            winword: Signal::new(get_winning_word()),
            max_words,
            wordlen

        }
    }

    pub fn change_letterstate(&mut self, row_index: usize,
      letter_index: usize, letter_state: LetterState
    ) {
        let mut row = self.table.get_mut(row_index).unwrap();
        let mut letter = row.letters.get_mut(letter_index).unwrap();
        letter.color = letter_state.color;
        letter.value = letter_state.value;
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
    pub value: Option<char>,
    pub color: LetterColor,
}

impl LetterState {
    pub fn new(value: char, color: LetterColor) -> LetterState {
        LetterState {
            value: Option::Some(value),
            color: color,
        }
    }
    pub fn default() -> LetterState {
        LetterState {
            value: Option::None,
            color: LetterColor::Incorrect,
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
