use crate::words::WORD_LETTERS;
use std::iter::Step;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum Letter {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    E = 4,
    F = 5,
    G = 6,
    H = 7,
    I = 8,
    J = 9,
    K = 10,
    L = 11,
    M = 12,
    N = 13,
    O = 14,
    P = 15,
    Q = 16,
    R = 17,
    S = 18,
    T = 19,
    U = 20,
    V = 21,
    W = 22,
    X = 23,
    Y = 24,
    Z = 25,
}

pub type Word = [Letter; WORD_LETTERS];

impl Letter {
    pub const fn from_u8(other: u8) -> Self {
        ALPHABET[other as usize]
    }

    pub const fn to_char(self) -> char {
        (self as u8 + b'a') as char
    }
}

impl Step for Letter {
    fn steps_between(start: &Self, end: &Self) -> Option<usize> {
        let (a, b) = (*start as usize, *end as usize);
        (a <= b).then(|| a - b)
    }

    fn forward_checked(start: Self, count: usize) -> Option<Self> {
        ALPHABET.get(start as usize + count).copied()
    }

    fn backward_checked(start: Self, count: usize) -> Option<Self> {
        (start as usize)
            .checked_sub(count)
            .map(|letter| ALPHABET[letter])
    }
}

const ALPHABET: [Letter; 26] = [
    Letter::A,
    Letter::B,
    Letter::C,
    Letter::D,
    Letter::E,
    Letter::F,
    Letter::G,
    Letter::H,
    Letter::I,
    Letter::J,
    Letter::K,
    Letter::L,
    Letter::M,
    Letter::N,
    Letter::O,
    Letter::P,
    Letter::Q,
    Letter::R,
    Letter::S,
    Letter::T,
    Letter::U,
    Letter::V,
    Letter::W,
    Letter::X,
    Letter::Y,
    Letter::Z,
];

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum LetterPos {
    P0 = 0,
    P1 = 1,
    P2 = 2,
    P3 = 3,
    P4 = 4,
}

const LETTER_POSITIONS: [LetterPos; WORD_LETTERS] = [
    LetterPos::P0,
    LetterPos::P1,
    LetterPos::P2,
    LetterPos::P3,
    LetterPos::P4,
];

impl Step for LetterPos {
    fn steps_between(start: &Self, end: &Self) -> Option<usize> {
        let (a, b) = (*start as usize, *end as usize);
        (a <= b).then(|| b - a)
    }

    fn forward_checked(start: Self, count: usize) -> Option<Self> {
        LETTER_POSITIONS.get(start as usize + count).copied()
    }

    fn backward_checked(start: Self, count: usize) -> Option<Self> {
        (start as usize)
            .checked_sub(count)
            .map(|letter_pos| LETTER_POSITIONS[letter_pos])
    }
}

impl From<usize> for LetterPos {
    fn from(other: usize) -> Self {
        match other {
            0 => LetterPos::P0,
            1 => LetterPos::P1,
            2 => LetterPos::P2,
            3 => LetterPos::P3,
            4 => LetterPos::P4,
            other => panic!("Invalid letter position: {other}"),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LetterResponse {
    Correct,
    Include,
    Exclude,
}

pub type Responses = [LetterResponse; WORD_LETTERS];

pub fn print_word(word: Word) {
    for letter in word {
        print!("{}", letter.to_char());
    }
}
