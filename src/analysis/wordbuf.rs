use crate::{
    analysis::{response_from, Analyse, MAX_GUESSES},
    letter::{print_word, LetterPos, LetterResponse, Responses, Word},
    words::{WORDLIST, WORD_LETTERS},
};
use std::{
    borrow::{Borrow, BorrowMut},
    mem::MaybeUninit,
    ops::{Deref, DerefMut},
    simd::u8x8,
    slice,
};

#[derive(Clone, Copy, Debug)]
pub struct WordBuf<const N: usize> {
    buf: MaybeUninit<[Word; N]>,
    len: usize,
}

pub type DefaultWordBuf = WordBuf<{ WORDLIST.len() }>;

impl<const N: usize> WordBuf<N> {
    pub const fn from_const(words: [Word; N]) -> Self {
        Self {
            buf: MaybeUninit::new(words),
            len: N,
        }
    }

    pub fn retain<F: FnMut(&mut Word) -> bool>(&mut self, mut f: F) {
        let mut del = 0;
        let len = self.len;
        for i in 0..len {
            if !f(&mut self[i]) {
                del += 1;
            } else {
                unsafe { self.swap_unchecked(i - del, i) };
            }
        }
        unsafe { self.set_len(len - del) };
    }

    unsafe fn swap_unchecked(&mut self, a: usize, b: usize) {
        let slice: &mut [Word] = self;
        slice.swap_unchecked(a, b);
    }

    pub unsafe fn set_len(&mut self, len: usize) {
        self.len = len;
    }

    pub fn update_iter(&mut self, guessed: Word, responses: Responses) {
        for pos in LetterPos::P0..=LetterPos::P4 {
            let response = responses[pos as usize];
            let guess = guessed[pos as usize];
            match response {
                LetterResponse::Correct => self.retain(|word| word[pos as usize] == guess),
                LetterResponse::Include => {
                    self.retain(|word| word[pos as usize] != guess && word.contains(&guess))
                }
                LetterResponse::Exclude => self.retain(|word| !word.contains(&guess)),
            }
        }
    }

    pub fn update_simd(&mut self, guessed: Word, responses: Responses) {
        for pos in LetterPos::P0..=LetterPos::P4 {
            let response = responses[pos as usize];
            let guess = guessed[pos as usize];
            match response {
                LetterResponse::Correct => self.retain(|word| word[pos as usize] == guess),
                LetterResponse::Include => {
                    let comp_include = u8x8::from_array([
                        guess as u8,
                        guess as u8,
                        guess as u8,
                        guess as u8,
                        guess as u8,
                        1,
                        1,
                        1,
                    ]);
                    self.retain(|word| {
                        word[pos as usize] != guess && simd_word(*word).lanes_eq(comp_include).any()
                    });
                }
                LetterResponse::Exclude => {
                    let comp_exclude = simd_word([guess; 5]);
                    self.retain(|word| {
                        simd_word(*word).lanes_ne(comp_exclude).to_array() == WORD_MASK
                    })
                }
            }
        }
    }
}

impl Default for DefaultWordBuf {
    fn default() -> Self {
        WordBuf::from_const(WORDLIST)
    }
}

impl Analyse for DefaultWordBuf {
    fn update(&mut self, _depth: usize, guessed: Word, responses: Responses) {
        self.update_iter(guessed, responses);
    }

    fn recurse(
        &mut self,
        win_counts: &mut [[usize; MAX_GUESSES]; WORDLIST.len()],
        loss_counts: &mut [usize; WORDLIST.len()],
        guesses_made: usize,
        initial: usize,
        correct: Word,
        guess: Word,
    ) {
        if guesses_made <= MAX_GUESSES {
            let responses = response_from(correct, guess);
            if responses == [LetterResponse::Correct; WORD_LETTERS] {
                win_counts[initial][guesses_made - 1] += 1;
            } else {
                self.update(guesses_made, guess, responses);
                if self.is_empty() {
                    loss_counts[initial] += 1;
                } else {
                    for &word in self.iter() {
                        let mut new = *self;
                        new.recurse(
                            win_counts,
                            loss_counts,
                            guesses_made + 1,
                            initial,
                            correct,
                            word,
                        );
                    }
                }
            }
        } else {
            loss_counts[initial] += 1;
        }
    }

    fn at_sg_end(&mut self) {
        *self = Self::default();
    }
}

const WORD_MASK: [bool; 8] = [true, true, true, true, true, false, false, false];

fn pack_word([w0, w1, w2, w3, w4]: Word) -> u64 {
    u64::from_ne_bytes([w0 as u8, w1 as u8, w2 as u8, w3 as u8, w4 as u8, 0, 0, 0])
}

fn simd_word(word: Word) -> u8x8 {
    u8x8::from_array(pack_word(word).to_ne_bytes())
}

impl<const N: usize> Deref for WordBuf<N> {
    type Target = [Word];

    #[inline]
    fn deref(&self) -> &[Word] {
        unsafe { slice::from_raw_parts(self.buf.as_ptr() as *const _, self.len) }
    }
}

impl<const N: usize> DerefMut for WordBuf<N> {
    #[inline]
    fn deref_mut(&mut self) -> &mut [Word] {
        unsafe { slice::from_raw_parts_mut(self.buf.as_mut_ptr() as *mut _, self.len) }
    }
}

impl<const N: usize> AsRef<[Word]> for WordBuf<N> {
    #[inline]
    fn as_ref(&self) -> &[Word] {
        self
    }
}

impl<const N: usize> AsMut<[Word]> for WordBuf<N> {
    #[inline]
    fn as_mut(&mut self) -> &mut [Word] {
        self
    }
}

impl<const N: usize> Borrow<[Word]> for WordBuf<N> {
    #[inline]
    fn borrow(&self) -> &[Word] {
        self
    }
}

impl<const N: usize> BorrowMut<[Word]> for WordBuf<N> {
    #[inline]
    fn borrow_mut(&mut self) -> &mut [Word] {
        self
    }
}
