pub struct Guess {
    pub letters: Vec<Letter>
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum LetterValue {
    A, B, C, D, E, F, G, H, I, J, K, L, M,
    N, O, P, Q, R, S, T, U, V, W, X, Y, Z,
}

impl TryFrom<u8> for LetterValue {
    type Error = ();

    fn try_from(c: u8) -> Result<Self, Self::Error> {
        match c.to_ascii_lowercase() {
            b'a' => Ok(LetterValue::A),
            b'b' => Ok(LetterValue::B),
            b'c' => Ok(LetterValue::C),
            b'd' => Ok(LetterValue::D),
            b'e' => Ok(LetterValue::E),
            b'f' => Ok(LetterValue::F),
            b'g' => Ok(LetterValue::G),
            b'h' => Ok(LetterValue::H),
            b'i' => Ok(LetterValue::I),
            b'j' => Ok(LetterValue::J),
            b'k' => Ok(LetterValue::K),
            b'l' => Ok(LetterValue::L),
            b'm' => Ok(LetterValue::M),
            b'n' => Ok(LetterValue::N),
            b'o' => Ok(LetterValue::O),
            b'p' => Ok(LetterValue::P),
            b'q' => Ok(LetterValue::Q),
            b'r' => Ok(LetterValue::R),
            b's' => Ok(LetterValue::S),
            b't' => Ok(LetterValue::T),
            b'u' => Ok(LetterValue::U),
            b'v' => Ok(LetterValue::V),
            b'w' => Ok(LetterValue::W),
            b'x' => Ok(LetterValue::X),
            b'y' => Ok(LetterValue::Y),
            b'z' => Ok(LetterValue::Z),
            _ => Err(()),
        }
    }
}

impl LetterValue {
    pub fn get_char(self) -> char {
        (b'A' + self as u8) as char
    }

    pub fn get_value(c: u8) -> LetterValue{
        match LetterValue::try_from(c) {
            Ok(value) => value,
            Err(()) => panic!("Invalid letter value!"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum LetterState{
    CORRECT, // In the word
    PRESENT, // Wrong place
    ABSENT
}

pub struct Letter {
    pub value: LetterValue,
    pub state: LetterState
}


impl Letter {
    pub fn get_letter(c: u8, state: LetterState) -> Letter{
        Letter {
            value: LetterValue::get_value(c),
            state 
        }
    }

}
