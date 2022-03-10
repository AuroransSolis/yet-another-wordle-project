use crate::letter::{Letter, Word};

pub const WORD_LETTERS: usize = 5;

#[allow(dead_code)]
const ALLOWED_WORDS_BYTES: [u8; 77832] = *include_bytes!("../allowed-wordlist.txt");
#[allow(dead_code)]
pub const ALLOWED_WORDS: [Word; 12972] = {
    let mut words = [[Letter::A; WORD_LETTERS]; 12972];
    let mut i = 0;
    let mut j = 0;
    let mut buf = [Letter::A; WORD_LETTERS];
    while i * (WORD_LETTERS + 1) + j < ALLOWED_WORDS_BYTES.len() {
        if ALLOWED_WORDS_BYTES[i * (WORD_LETTERS + 1) + j] == b'\n' {
            words[i] = buf;
            buf = [Letter::A; WORD_LETTERS];
            i += 1;
            j = 0;
        } else {
            buf[j] = Letter::from_u8(
                ALLOWED_WORDS_BYTES[i * (WORD_LETTERS + 1) + j].to_ascii_lowercase() - b'a',
            );
            j += 1;
        }
    }
    words
};

#[allow(dead_code)]
const POSSIBLE_WORDS_BYTES: [u8; 13890] = *include_bytes!("../possible-wordlist.txt");
#[allow(dead_code)]
pub const POSSIBLE_WORDS: [Word; 2315] = {
    let mut words = [[Letter::A; WORD_LETTERS]; 2315];
    let mut i = 0;
    let mut j = 0;
    let mut buf = [Letter::A; WORD_LETTERS];
    while i * (WORD_LETTERS + 1) + j < POSSIBLE_WORDS_BYTES.len() {
        if POSSIBLE_WORDS_BYTES[i * (WORD_LETTERS + 1) + j] == b'\n' {
            words[i] = buf;
            buf = [Letter::A; WORD_LETTERS];
            i += 1;
            j = 0;
        } else {
            buf[j] = Letter::from_u8(
                POSSIBLE_WORDS_BYTES[i * (WORD_LETTERS + 1) + j].to_ascii_lowercase() - b'a',
            );
            j += 1;
        }
    }
    words
};

// pub const WORDLIST: [Word; 12972] = ALLOWED_WORDS;
pub const WORDLIST: [Word; 2315] = POSSIBLE_WORDS;
