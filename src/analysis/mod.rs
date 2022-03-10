use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};

use crate::{
    letter::{print_word, Letter, LetterPos, LetterResponse, Responses, Word},
    words::{WORDLIST, WORD_LETTERS},
};
use std::{cmp::Ordering, time::Instant};

pub mod game;
pub mod levels;
pub mod wordbuf;
pub mod wordlist;

pub const MAX_GUESSES: usize = 6;
const MAX_CW: usize = 5;

pub trait Analyse: Copy + Default + Send + Sync {
    fn update(&mut self, depth: usize, guessed: Word, responses: Responses);
    fn recurse(
        &mut self,
        word_stats: &mut WordStats,
        guesses_made: usize,
        initial: usize,
        correct: Word,
        guess: Word,
    );
    fn at_sg_end(&mut self);
}

pub fn best_first_guess<T: Analyse>() -> [WordStats; WORDLIST.len()] {
    let mut analyser = T::default();
    let mut word_stats = DEFAULT_WORD_STATS;
    let total_start = Instant::now();
    for correct in (0..WORDLIST.len()).take(MAX_CW) {
        let correct_word = WORDLIST[correct];
        let correct_start = Instant::now();
        for (initial, guess) in WORDLIST.into_iter().enumerate() {
            // let initial_start = Instant::now();
            analyser.recurse(&mut word_stats[initial], 1, initial, correct_word, guess);
            // let initial_end = initial_start.elapsed();
            // println!("    g0: {initial:>4} | elapsed: {initial_end:?}");
            analyser.at_sg_end();
        }
        let correct_end = correct_start.elapsed();
        println!("cw: {correct:>4} | elapsed: {correct_end:?}");
    }
    let total_end = total_start.elapsed();
    println!("total: {total_end:?}");
    word_stats.sort_unstable_by(|s1, s2| s1.cmp(s2).reverse());
    word_stats
}

pub fn best_first_guess_rayon_fg<T: Analyse>() -> [WordStats; WORDLIST.len()] {
    let _ = rayon::ThreadPoolBuilder::new()
        .stack_size(1 << 25)
        .build_global();
    let mut word_stats = DEFAULT_WORD_STATS;
    let total_start = Instant::now();
    for correct in (0..WORDLIST.len()).take(MAX_CW) {
        let correct_word = WORDLIST[correct];
        let correct_start = Instant::now();
        let iter_stats = WORDLIST
            .into_par_iter()
            .enumerate()
            .map(|(initial, guess)| {
                let mut analyser = T::default();
                let mut word_stats = WordStats::new(guess, [0; MAX_GUESSES], 0);
                analyser.recurse(&mut word_stats, 1, initial, correct_word, guess);
                (initial, word_stats)
            })
            .fold(
                || DEFAULT_WORD_STATS,
                |mut all_ws, (word, ws)| {
                    all_ws[word].combine_with(&ws);
                    all_ws
                },
            )
            .reduce(|| DEFAULT_WORD_STATS, |ws1, ws2| combine_wordstats(ws1, &ws2));
        word_stats = combine_wordstats(word_stats, &iter_stats);
        let correct_end = correct_start.elapsed();
        println!("cw: {correct:>4} | elapsed: {correct_end:?}");
    }
    let total_end = total_start.elapsed();
    println!("total: {total_end:?}");
    word_stats.sort_unstable_by(|s1, s2| s1.cmp(s2).reverse());
    word_stats
}

pub fn best_first_guess_rayon_cw<T: Analyse>() -> [WordStats; WORDLIST.len()] {
    let _ = rayon::ThreadPoolBuilder::new()
        .stack_size(1 << 25)
        .build_global();
    let total_start = Instant::now();
    let mut word_stats = WORDLIST
        .into_par_iter()
        .enumerate()
        .take(MAX_CW)
        .map(|(correct, correct_word)| {
            let correct_start = Instant::now();
            let mut analyser = T::default();
            let mut iter_stats = DEFAULT_WORD_STATS;
            // let correct_start = Instant::now();
            for (initial, guess) in WORDLIST.into_iter().enumerate() {
                // let initial_start = Instant::now();
                analyser.recurse(&mut iter_stats[initial], 1, initial, correct_word, guess);
                // let initial_end = initial_start.elapsed();
                // println!("    g0: {initial:>4} | elapsed: {initial_end:?}");
                analyser.at_sg_end();
            }
            let correct_end = correct_start.elapsed();
            println!("cw: {correct:>4} | elapsed: {correct_end:?}");
            iter_stats
        })
        .reduce(|| DEFAULT_WORD_STATS, |ws1, ws2| combine_wordstats(ws1, &ws2));
    let total_end = total_start.elapsed();
    println!("total: {total_end:?}");
    word_stats.sort_unstable_by(|s1, s2| s1.cmp(s2).reverse());
    word_stats
}

pub fn best_first_guess_rayon_fm<T: Analyse>() -> [WordStats; WORDLIST.len()] {
    let _ = rayon::ThreadPoolBuilder::new()
        .stack_size(1 << 25)
        .build_global();
    let total_start = Instant::now();
    let mut word_stats = WORDLIST
        .into_par_iter()
        .take(MAX_CW)
        .flat_map_iter(|cw| {
            WORDLIST
                .into_iter()
                .enumerate()
                .map(move |(i, g)| (cw, i, g))
        })
        .map(
            |(correct_word, initial, guess)| {
                let mut iter_stats = WordStats::new(correct_word, [0; MAX_GUESSES], 0);
                T::default().recurse(&mut iter_stats, 1, initial, correct_word, guess);
                (initial, iter_stats)
            },
        )
        .fold(
            || DEFAULT_WORD_STATS,
            |mut all_ws, (word, ws)| {
                all_ws[word].combine_with(&ws);
                all_ws
            },
        )
        .reduce(|| DEFAULT_WORD_STATS, |ws1, ws2| combine_wordstats(ws1, &ws2));
    let total_end = total_start.elapsed();
    println!("total: {total_end:?}");
    word_stats.sort_unstable_by(|s1, s2| s1.cmp(s2).reverse());
    word_stats
}

fn combine_wordstats(
    mut ws1: [WordStats; WORDLIST.len()],
    ws2: &[WordStats; WORDLIST.len()],
) -> [WordStats; WORDLIST.len()] {
    ws1.iter_mut()
        .zip(ws2.iter())
        .for_each(|(ws1, ws2)| ws1.combine_with(ws2));
    ws1
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
    pub word: Word,
    pub wins_per_turn: [usize; MAX_GUESSES],
    pub losses: usize,
}

const DEFAULT_WORD_STATS: [WordStats; WORDLIST.len()] = {
    let mut word_stats =
        [WordStats::new([Letter::A; WORD_LETTERS], [0; MAX_GUESSES], 0); WORDLIST.len()];
    let mut i = 0;
    while i < WORDLIST.len() {
        word_stats[i] = WordStats::new(WORDLIST[i], [0; MAX_GUESSES], 0);
        i += 1;
    }
    word_stats
};

impl WordStats {
    const fn new(word: Word, wins_per_turn: [usize; MAX_GUESSES], losses: usize) -> Self {
        Self {
            word,
            wins_per_turn,
            losses,
        }
    }

    pub fn combine_with(&mut self, other: &Self) {
        self.wins_per_turn
            .iter_mut()
            .zip(other.wins_per_turn.iter())
            .for_each(|(ws1, ws2)| *ws1 += ws2);
        self.losses += other.losses;
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
