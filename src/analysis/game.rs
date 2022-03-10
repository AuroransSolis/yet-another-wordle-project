// use crate::{
//     analysis::Analyse,
//     letter::{LetterPos, LetterResponse, Responses, Word},
//     // letter::{Letter, Word, LETTER_MASK_NEG, LETTER_MASK_POS},
//     wordbuf::WordBuf,
//     words::WORDLIST,
// };
// use smallvec::SmallVec;
// use std::{simd::u8x8, slice::Iter};

/*
#[derive(Clone, Copy, Debug)]
pub enum Position {
    Known(Letter),
    // Using bitflags for now - maybe unpacking is faster?
    Unknown(u32),
    // Unknown([bool; 26])
}

impl Position {
    pub fn is(&mut self, letter: Letter) {
        *self = Position::Known(letter);
    }

    pub fn is_not(&mut self, letter: Letter) {
        if let Position::Unknown(flags) = self {
            *flags &= LETTER_MASK_NEG[letter as usize];
            // flags[letter as usize] = false;
        }
    }

    pub fn matches(self, letter: Letter) -> bool {
        match self {
            Position::Known(known) => known == letter,
            Position::Unknown(flags) => flags & LETTER_MASK_POS[letter as usize] > 0,
        }
    }

    pub fn matches_pos(self, word: Word, pos: LetterPos) -> bool {
        self.matches(word[pos as usize])
    }
}

impl Default for Position {
    fn default() -> Self {
        // Using bitflags for now - maybe unpacking is faster?
        Self::Unknown(0x03ffffff)
        // Self::Unknown([true; 26])
    }
}
*/

/*
#[derive(Clone, Debug)]
pub struct Game {
    pub wordlist: SmallVec<[Word; WORDLIST.len()]>,
    // pub possible_words: SmallVec<[Word; POSSIBLE_WORDS.len()]>,
    // pub allowed_words: SmallVec<[Word; ALLOWED_WORDS.len()]>,
    pub positions: [Position; 5],
}

impl WordList for Game {
    fn new() -> Self {
        Self {
            wordlist: SmallVec::from_const(WORDLIST),
            // possible_words: SmallVec::from_const(POSSIBLE_WORDS),
            // allowed_words: SmallVec::from_const(ALLOWED_WORDS),
            positions: [Position::default(); 5],
        }
    }

    fn update(&mut self, guessed: Word, responses: Response) {
        for pos in LetterPos::P0..=LetterPos::P4 {
            let response = responses[pos as usize];
            let guess = guessed[pos as usize];
            match response {
                LetterResponse::Correct => {
                    self.positions[pos as usize].is(guess);
                    self.wordlist
                        .retain(|word| self.positions[pos as usize].matches_pos(*word, pos));
                    // self.possible_words
                    //     .retain(|word| self.positions[pos as usize].matches_pos(*word, pos));
                    // self.allowed_words
                    //     .retain(|word| self.positions[pos as usize].matches_pos(*word, pos));
                }
                LetterResponse::Include => {
                    self.positions[pos as usize].is_not(guess);
                    self.wordlist.retain(|word| {
                        self.positions[pos as usize].matches_pos(*word, pos)
                            && word.contains(&guess)
                    });
                }
                LetterResponse::Exclude => {
                    self.positions.iter_mut().for_each(|pos| pos.is_not(guess));
                    self.wordlist.retain(|&mut word| {
                        word.iter()
                            .zip(self.positions.iter())
                            .all(|(&letter, position)| position.matches(letter))
                    });
                }
            }
        }
    }

    fn words(&self) -> Iter<'_, Word> {
        self.wordlist.iter()
    }

    fn is_empty(&self) -> bool {
        self.wordlist.is_empty()
    }
}
*/
