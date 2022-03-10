#![feature(
    array_zip,
    const_eval_limit,
    portable_simd,
    slice_swap_unchecked,
    step_trait
)]
#![const_eval_limit = "10000000"]

mod analysis;
mod letter;
mod words;

use analysis::{levels::DefaultLevels, response_from, wordbuf::DefaultWordBuf, Analyse, WordStats};
use letter::{print_word, Letter, LetterResponse};
use words::WORDLIST;

fn main() {
    // let words_stats_0 = analysis::best_first_guess::<Game>();
    // words_stats_0
    //     .iter()
    //     .take(10)
    //     .enumerate()
    //     .for_each(|(rank, stats)| stats.display_rank_with_indent(rank, 0, 4));
    // println!();
    // let words_stats_1 = analysis::best_first_guess::<DefaultWordBuf>();
    // words_stats_1
    //     .iter()
    //     .take(10)
    //     .enumerate()
    //     .for_each(|(rank, stats)| stats.display_rank_with_indent(rank, 0, 4));
    let words_stats_2 = analysis::best_first_guess::<DefaultLevels>();
    words_stats_2
        .iter()
        .take(3)
        .enumerate()
        .for_each(|(rank, stats)| stats.display_rank_with_indent(rank, 0, 4));
    let words_stats_3 = analysis::best_first_guess_rayon_fg::<DefaultLevels>();
    words_stats_3
        .iter()
        .take(3)
        .enumerate()
        .for_each(|(rank, stats)| stats.display_rank_with_indent(rank, 0, 4));
    let words_stats_4 = analysis::best_first_guess_rayon_cw::<DefaultLevels>();
    words_stats_4
        .iter()
        .take(3)
        .enumerate()
        .for_each(|(rank, stats)| stats.display_rank_with_indent(rank, 0, 4));
    let words_stats_5 = analysis::best_first_guess_rayon_fm::<DefaultLevels>();
    words_stats_5
        .iter()
        .take(3)
        .enumerate()
        .for_each(|(rank, stats)| stats.display_rank_with_indent(rank, 0, 4));
    // println!("same results? {}", words_stats_1 == words_stats_2);
    // let mut wordbuf = DefaultWordBuf::default();
    // let mut levels = DefaultLevels::default();
    // let correct = [Letter::C, Letter::I, Letter::G, Letter::A, Letter::R];
    // let guesses = [
    //     [Letter::R, Letter::E, Letter::B, Letter::U, Letter::T],
    //     // [Letter::C, Letter::L, Letter::I, Letter::P, Letter::T],
    //     // [Letter::T, Letter::R, Letter::E, Letter::N, Letter::D],
    //     // [Letter::V, Letter::O, Letter::T, Letter::E, Letter::R],
    // ];
    // for i in 0..guesses.len() {
    //     let guess = guesses[i];
    //     let guess_num = i + 1;
    //     let response = response_from(correct, guess);
    //     println!("{:?}", response);
    //     wordbuf.update(guess_num, guess, response);
    //     levels.update(guess_num, guess, response);
    //     let mut wb_words = wordbuf.iter();
    //     let mut ls_words = levels.levels[guess_num].iter();
    //     let mut all = true;
    //     loop {
    //         let wb_opt = wb_words.next();
    //         let ls_opt = ls_words.next();
    //         if wb_opt.is_none() && ls_opt.is_none() {
    //             break;
    //         } else {
    //             all &= wb_opt == ls_opt;
    //             print!("wl: ");
    //             if let Some(&word) = wb_opt {
    //                 print_word(word);
    //             } else {
    //                 print!("_____");
    //             }
    //             print!(" | ls: ");
    //             if let Some(&word) = ls_opt {
    //                 print_word(word);
    //             } else {
    //                 print!("_____");
    //             }
    //             println!();
    //         }
    //     }
    //     println!("all same: {all}");
    // }
    // println!();
    // wordbuf = DefaultWordBuf::default();
    // let correct = [Letter::C, Letter::I, Letter::G, Letter::A, Letter::R];
    // let guesses = [
    //     [Letter::S, Letter::I, Letter::S, Letter::S, Letter::Y],
    // ];
    // for i in 0..guesses.len() {
    //     let guess = guesses[i];
    //     let guess_num = i + 1;
    //     let response = response_from(correct, guess);
    //     println!("{:?}", response);
    //     wordbuf.update(guess_num, guess, response);
    //     levels.update(guess_num, guess, response);
    //     let mut wb_words = wordbuf.iter();
    //     let mut ls_words = levels.levels[guess_num].iter();
    //     let mut all = true;
    //     loop {
    //         let wb_opt = wb_words.next();
    //         let ls_opt = ls_words.next();
    //         if wb_opt.is_none() && ls_opt.is_none() {
    //             break;
    //         } else {
    //             all &= wb_opt == ls_opt;
    //             print!("wl: ");
    //             if let Some(&word) = wb_opt {
    //                 print_word(word);
    //             } else {
    //                 print!("_____");
    //             }
    //             print!(" | ls: ");
    //             if let Some(&word) = ls_opt {
    //                 print_word(word);
    //             } else {
    //                 print!("_____");
    //             }
    //             println!();
    //         }
    //     }
    //     println!("all same: {all}");
    // }
}
