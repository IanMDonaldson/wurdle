
pub struct Wurdle {
    attempts: Vec<Wurd>,
    curr_try: u8,
    max_tries: u8,
    wurd_length: u8,
}
struct Wurd {
    letters: Vec<LetterBox>,
    length: u8,

}

#[derive( Clone)]
struct LetterBox {
    value: String,
    state: LetterState,
}
#[derive(Clone, Copy)]
enum LetterState {
    Incorrect,
    WrongSpot,
    Correct,
}

impl Wurdle {

    pub fn new() -> Wurdle {
        let (curr_try, max_tries, wurd_length) = (0,6,5);
        let mut attempts: Vec<Wurd> = Vec::new();
        let mut letters: Vec<LetterBox> = Vec::new();


        for x in (0..wurd_length) {
            letters.push(LetterBox{
                value: "A".to_string(),
                state: LetterState::Incorrect});

        }
        for x in (0..max_tries) {
            attempts.push(Wurd {
                letters: letters.clone(),
                length: wurd_length,
            });
        }
        Wurdle {
            attempts,
            curr_try,
            max_tries,
            wurd_length,
        }
    }
}
