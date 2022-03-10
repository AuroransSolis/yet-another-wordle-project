use crate::{
    letter::{print_word, LetterPos, LetterResponse, Responses, Word},
    words::{WORDLIST, WORD_LETTERS},
};
use std::{cmp::Ordering, time::Instant};

pub mod game;
pub mod levels;
pub mod wordbuf;
pub mod wordlist;

pub const MAX_GUESSES: usize = 6;

pub trait Analyse: Copy + Default {
    fn update(&mut self, depth: usize, guessed: Word, responses: Responses);
    fn recurse(
        &mut self,
        win_counts: &mut [[usize; MAX_GUESSES]; WORDLIST.len()],
        loss_counts: &mut [usize; WORDLIST.len()],
        guesses_made: usize,
        initial: usize,
        correct: Word,
        guess: Word,
    );
    fn at_sg_end(&mut self);
}

pub fn best_first_guess<T: Analyse>() -> [WordStats; WORDLIST.len()] {
    // pub fn best_first_guess() -> [WordStats; WORDLIST.len()] {
    // let wl = WL::new();
    let mut analyser = T::default();
    let mut win_counts = [[0; 6]; WORDLIST.len()];
    let mut loss_counts = [0; WORDLIST.len()];
    // let mut word_stats = STARTING_STATS;
    // let total_start = Instant::now();
    for correct in (0..WORDLIST.len()).take(1) {
        let correct_word = WORDLIST[correct];
        // let correct_start = Instant::now();
        for (initial, guess) in WORDLIST.into_iter().enumerate() {
            // let initial_start = Instant::now();
            analyser.recurse(
                &mut win_counts,
                &mut loss_counts,
                1,
                initial,
                correct_word,
                guess,
            );
            // let initial_end = initial_start.elapsed();
            // println!("    g0: {initial:>4} | elapsed: {initial_end:?}");
            analyser.at_sg_end();
        }
        // let correct_end = correct_start.elapsed();
        // println!("cw: {correct:>4} | elapsed: {correct_end:?}");
    }
    // let total_end = total_start.elapsed();
    // println!("total: {total_end:?}");
    let mut word = 0;
    let mut word_stats = win_counts.zip(loss_counts).map(|(wins_per_turn, losses)| {
        let stats = WordStats::new(WORDLIST[word], wins_per_turn, losses);
        word += 1;
        stats
    });
    word_stats.sort_unstable_by(|s1, s2| s1.cmp(s2).reverse());
    word_stats
}

pub fn response_from(correct: Word, guess: Word) -> Responses {
    let mut response = [LetterResponse::Exclude; WORD_LETTERS];
    for (letter, pos) in guess.iter().zip(LetterPos::P0..=LetterPos::P4) {
        if *letter == correct[pos as usize] {
            response[pos as usize] = LetterResponse::Correct;
        } else if correct.contains(letter) {
            response[pos as usize] = LetterResponse::Include;
        }
    }
    response
}

#[derive(Clone, Copy, Debug, Eq)]
pub struct WordStats {
    word: Word,
    wins_per_turn: [usize; MAX_GUESSES],
    losses: usize,
}

impl WordStats {
    const fn new(word: Word, wins_per_turn: [usize; 6], losses: usize) -> Self {
        Self {
            word,
            wins_per_turn,
            losses,
        }
    }
}

impl WordStats {
    pub fn display_rank_with_indent(&self, rank: usize, indent: usize, tabsize: usize) {
        const EMPTY: &str = "";
        print!("{EMPTY:>width$}rank {rank}: ", width = indent);
        print_word(self.word);
        println!();
        for (guess, wins) in self.wins_per_turn.iter().enumerate() {
            println!(
                "{EMPTY:>width$}G{}: {wins}",
                guess + 1,
                width = indent + tabsize
            );
        }
        println!(
            "{EMPTY:>width$}losses: {}",
            self.losses,
            width = indent + tabsize
        );
    }
}

impl PartialEq for WordStats {
    fn eq(&self, other: &Self) -> bool {
        self.losses == other.losses && self.wins_per_turn == other.wins_per_turn
    }

    fn ne(&self, other: &Self) -> bool {
        self.losses != other.losses && self.wins_per_turn != other.wins_per_turn
    }
}

impl PartialOrd for WordStats {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(
            self.losses
                .cmp(&other.losses)
                .reverse()
                .then_with(|| self.wins_per_turn[1..].cmp(&other.wins_per_turn[1..])),
        )
    }
}

impl Ord for WordStats {
    fn cmp(&self, other: &Self) -> Ordering {
        self.losses
            .cmp(&other.losses)
            .reverse()
            .then_with(|| self.wins_per_turn[1..].cmp(&other.wins_per_turn[1..]))
    }
}
