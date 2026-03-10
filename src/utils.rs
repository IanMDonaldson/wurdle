use std::fmt;
use std::fmt::Formatter;
use std::fs::read_to_string;
use std::io::Read;
use dioxus::hooks::use_context;

pub fn get_winning_word() -> String {
    let wordle_list = read_to_string("wordle-list").unwrap();
    let winvec = Vec::from_iter(wordle_list.lines());
    let winningword = winvec.get(rand::random_range(0..2307)).unwrap().to_string();
    winningword
}

pub fn handle_key(letter: String, /*answer: String, winning_word: String*/) {
    let guessed_words = use_context::<GuessedWords>().0;
    let mut curr_word = use_context::<CurrWord>().0;
    let win_word = use_context::<WinWord>().0;
    let winchars: Vec<char> = win_word.chars().collect();
    // either done or prematurely hit enter
    if letter.eq("⏎") {
        if curr_word.len() != win_word.len() {
            println!("Length not satisfied..");
            return
        } else {
            //now actually gen a vec<LetterState> to push to guessed words
            // todo refactor later
            let mut new_guess: Vec<LetterState> = Vec::new();
            for (index , cur_letter) in curr_word.chars().enumerate() {
                if *winchars.get(index).unwrap() == cur_letter {
                    //correct!
                    new_guess.push(LetterState {
                        value: cur_letter.to_string(),
                        color: LetterColor::Correct,
                    });
                } else if win_word.contains(cur_letter) {
                    //bad spot
                    new_guess.push(LetterState {
                        value: cur_letter.to_string(),
                        color: LetterColor::WrongSpot,
                    })
                } else {
                    new_guess.push(LetterState {
                        value: cur_letter.to_string(),
                        color: LetterColor::Incorrect,
                    })
                }
            }
        }
    } else if letter.eq("⌫") {
        //just remove a letter from currword
        match curr_word.pop() {
            Some(ch) => println!("backspace removed letter"),
            None => println!("backspace didn't remove a letter because empty")
        }
    } else if curr_word.len() < win_word.len() {
        // Now add the letter to currword
        curr_word.push(letter.chars().last().unwrap());
    }
}

#[derive(Clone, Default)]
pub struct GuessedWords(pub Vec<Vec<LetterState>>);

#[derive(Clone)]
pub struct CurrWord(pub String);

#[derive(Clone)]
pub struct WinWord(pub String);

#[derive(Clone)]
pub struct WordLen(pub usize);

#[derive(Clone, PartialEq)]
pub struct LetterState {
    pub value: String,
    pub color: LetterColor,
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
            LetterColor::Incorrect => "#434742",
            LetterColor::WrongSpot => "#848484",
            LetterColor::Correct => "#a800a8",
        }
    }
}
impl fmt::Display for LetterState {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

// #[derive(Props, Clone, PartialEq)]
// pub struct WordOption {
//     word: Option<String>,
// }
