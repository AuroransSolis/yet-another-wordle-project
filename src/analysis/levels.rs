use crate::{
    analysis::{response_from, wordlist::WordList, Analyse, WordStats, MAX_GUESSES},
    letter::{LetterPos, LetterResponse, Responses, Word},
    words::{WORDLIST, WORD_LETTERS},
};
use std::simd::{u8x8, Mask};

#[derive(Clone, Copy, Debug)]
pub struct Levels<const L: usize, const N: usize> {
    pub levels: [WordList<N>; L],
}

pub type DefaultLevels = Levels<{ MAX_GUESSES + 1 }, { WORDLIST.len() }>;

impl Default for DefaultLevels {
    fn default() -> Self {
        Self {
            levels: [WordList::from_const(WORDLIST); MAX_GUESSES + 1],
        }
    }
}

impl DefaultLevels {
    pub fn update_cont(&mut self, depth: usize, guessed: Word, responses: Responses) {
        // self.levels[depth].clear();
        unsafe { self.levels.get_unchecked_mut(depth).clear() };
        let previous = depth - 1;
        // let words_max = self.levels[previous].len();
        let words_max = unsafe { self.levels.get_unchecked(previous).len() };
        'words: for i in 0..words_max {
            // let word = self.levels[previous][i];
            let word = unsafe { *self.levels.get_unchecked(previous).get_unchecked(i) };
            for pos in LetterPos::P0..=LetterPos::P4 {
                let response = responses[pos as usize];
                let guess = guessed[pos as usize];
                let matches_response = match response {
                    LetterResponse::Correct => word[pos as usize] == guess,
                    LetterResponse::Include => word[pos as usize] != guess && word.contains(&guess),
                    LetterResponse::Exclude => !word.contains(&guess),
                };
                if !matches_response {
                    continue 'words;
                }
            }
            // unsafe { self.levels[depth].push_unchecked(word) };
            unsafe { self.levels.get_unchecked_mut(depth).push_unchecked(word) };
        }
    }

    pub fn update_simd(&mut self, depth: usize, guessed: Word, responses: Responses) {
        // self.levels[depth].clear();
        unsafe { self.levels.get_unchecked_mut(depth).clear() };
        let previous = depth - 1;
        // let words_max = self.levels[previous].len();
        let words_max = unsafe { self.levels.get_unchecked(previous).len() };
        'words: for i in 0..words_max {
            // let word = self.levels[previous][i];
            let word = unsafe { *self.levels.get_unchecked(previous).get_unchecked(i) };
            for pos in LetterPos::P0..=LetterPos::P4 {
                let response = responses[pos as usize];
                let guess = guessed[pos as usize];
                let matches_response = match response {
                    LetterResponse::Correct => word[pos as usize] == guess,
                    LetterResponse::Include => {
                        word[pos as usize] != guess
                            && simd_word(word)
                                .lanes_eq(u8x8::from_array([
                                    guess as u8,
                                    guess as u8,
                                    guess as u8,
                                    guess as u8,
                                    guess as u8,
                                    1,
                                    1,
                                    1,
                                ]))
                                .any()
                    }
                    LetterResponse::Exclude => !(simd_word(word).lanes_eq(simd_word([guess; 5]))
                        & Mask::from_array(WORD_MASK))
                    .any(),
                };
                if !matches_response {
                    continue 'words;
                }
            }
            // unsafe { self.levels[depth].push_unchecked(word) };
            unsafe { self.levels.get_unchecked_mut(depth).push_unchecked(word) };
        }
    }
}

const WORD_MASK: [bool; 8] = [true, true, true, true, true, false, false, false];

fn pack_word([w0, w1, w2, w3, w4]: Word) -> u64 {
    u64::from_ne_bytes([w0 as u8, w1 as u8, w2 as u8, w3 as u8, w4 as u8, 0, 0, 0])
}

fn simd_word(word: Word) -> u8x8 {
    u8x8::from_array(pack_word(word).to_ne_bytes())
}

impl Analyse for DefaultLevels {
    fn update(&mut self, depth: usize, guessed: Word, responses: Responses) {
        self.update_simd(depth, guessed, responses);
    }

    fn recurse(
        &mut self,
        word_stats: &mut WordStats,
        guesses_made: usize,
        initial: usize,
        correct: Word,
        guess: Word,
    ) {
        if guesses_made <= MAX_GUESSES {
            let responses = response_from(correct, guess);
            if responses == [LetterResponse::Correct; WORD_LETTERS] {
                // win_counts[initial][guesses_made - 1] += 1;
                unsafe { *word_stats.wins_per_turn.get_unchecked_mut(guesses_made - 1) += 1 };
            } else {
                self.update(guesses_made, guess, responses);
                // let is_empty = self.levels[guesses_made].is_empty();
                let is_empty = unsafe { self.levels.get_unchecked(guesses_made).is_empty() };
                if is_empty {
                    // loss_counts[initial] += 1;
                    word_stats.losses += 1;
                } else {
                    // let ind_max = self.levels[guesses_made].len();
                    let ind_max = unsafe { self.levels.get_unchecked(guesses_made).len() };
                    for ind in 0..ind_max {
                        // let word = self.levels[guesses_made][ind];
                        let word =
                            unsafe { *self.levels.get_unchecked(guesses_made).get_unchecked(ind) };
                        self.recurse(word_stats, guesses_made + 1, initial, correct, word);
                    }
                }
            }
        } else {
            // loss_counts[initial] += 1;
            word_stats.losses += 1;
        }
    }

    fn at_sg_end(&mut self) {}
}
