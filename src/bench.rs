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

use analysis::{levels::DefaultLevels, response_from, wordbuf::DefaultWordBuf};
use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};
use letter::LetterResponse;
use rand::prelude::*;
use words::WORDLIST;

/*pub fn bench_game_update(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    c.bench_function("Game::update (all correct)", |b| {
        b.iter_batched(
            || {
                let guessed = *WORDLIST.choose(&mut rng).unwrap();
                (Game::new(), guessed, [LetterResponse::Correct; 5])
            },
            |(mut game, guessed, responses)| black_box(game.update(guessed, responses)),
            BatchSize::SmallInput,
        )
    });
    c.bench_function("Game::update (all included)", |b| {
        b.iter_batched(
            || {
                let guessed = *WORDLIST.choose(&mut rng).unwrap();
                (Game::new(), guessed, [LetterResponse::Include; 5])
            },
            |(mut game, guessed, responses)| black_box(game.update(guessed, responses)),
            BatchSize::SmallInput,
        )
    });
    c.bench_function("Game::update (all excluded)", |b| {
        b.iter_batched(
            || {
                let guessed = *WORDLIST.choose(&mut rng).unwrap();
                (Game::new(), guessed, [LetterResponse::Exclude; 5])
            },
            |(mut game, guessed, responses)| black_box(game.update(guessed, responses)),
            BatchSize::SmallInput,
        )
    });
    c.bench_function("Game::update (random)", |b| {
        b.iter_batched(
            || {
                let correct = *WORDLIST.choose(&mut rng).unwrap();
                let guessed = *WORDLIST.choose(&mut rng).unwrap();
                (Game::new(), guessed, response_from(correct, guessed))
            },
            |(mut game, guessed, responses)| black_box(game.update(guessed, responses)),
            BatchSize::SmallInput,
        )
    });
}*/

pub fn bench_wordbuf_update(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    c.bench_function("Wordlist::update (all correct)", |b| {
        b.iter_batched(
            || {
                let guessed = *WORDLIST.choose(&mut rng).unwrap();
                (DefaultWordBuf::default(), guessed, [LetterResponse::Correct; 5])
            },
            |(mut wordlist, guessed, responses)| black_box(wordlist.update_iter(guessed, responses)),
            BatchSize::SmallInput,
        )
    });
    c.bench_function("Wordlist::update (all included)", |b| {
        b.iter_batched(
            || {
                let guessed = *WORDLIST.choose(&mut rng).unwrap();
                (DefaultWordBuf::default(), guessed, [LetterResponse::Include; 5])
            },
            |(mut wordlist, guessed, responses)| black_box(wordlist.update_iter(guessed, responses)),
            BatchSize::SmallInput,
        )
    });
    c.bench_function("Wordlist::update (all excluded)", |b| {
        b.iter_batched(
            || {
                let guessed = *WORDLIST.choose(&mut rng).unwrap();
                (DefaultWordBuf::default(), guessed, [LetterResponse::Exclude; 5])
            },
            |(mut wordlist, guessed, responses)| black_box(wordlist.update_iter(guessed, responses)),
            BatchSize::SmallInput,
        )
    });
    c.bench_function("Wordlist::update (random)", |b| {
        b.iter_batched(
            || {
                let correct = *WORDLIST.choose(&mut rng).unwrap();
                let guessed = *WORDLIST.choose(&mut rng).unwrap();
                (DefaultWordBuf::default(), guessed, response_from(correct, guessed))
            },
            |(mut wordlist, guessed, responses)| black_box(wordlist.update_iter(guessed, responses)),
            BatchSize::SmallInput,
        )
    });
    c.bench_function("Wordlist::update (all correct)", |b| {
        b.iter_batched(
            || {
                let guessed = *WORDLIST.choose(&mut rng).unwrap();
                (DefaultWordBuf::default(), guessed, [LetterResponse::Correct; 5])
            },
            |(mut wordlist, guessed, responses)| black_box(wordlist.update_simd(guessed, responses)),
            BatchSize::SmallInput,
        )
    });
    c.bench_function("Wordlist::update (all included)", |b| {
        b.iter_batched(
            || {
                let guessed = *WORDLIST.choose(&mut rng).unwrap();
                (DefaultWordBuf::default(), guessed, [LetterResponse::Include; 5])
            },
            |(mut wordlist, guessed, responses)| black_box(wordlist.update_simd(guessed, responses)),
            BatchSize::SmallInput,
        )
    });
    c.bench_function("Wordlist::update (all excluded)", |b| {
        b.iter_batched(
            || {
                let guessed = *WORDLIST.choose(&mut rng).unwrap();
                (DefaultWordBuf::default(), guessed, [LetterResponse::Exclude; 5])
            },
            |(mut wordlist, guessed, responses)| black_box(wordlist.update_simd(guessed, responses)),
            BatchSize::SmallInput,
        )
    });
    c.bench_function("Wordlist::update (random)", |b| {
        b.iter_batched(
            || {
                let correct = *WORDLIST.choose(&mut rng).unwrap();
                let guessed = *WORDLIST.choose(&mut rng).unwrap();
                (DefaultWordBuf::default(), guessed, response_from(correct, guessed))
            },
            |(mut wordlist, guessed, responses)| black_box(wordlist.update_simd(guessed, responses)),
            BatchSize::SmallInput,
        )
    });
}

pub fn bench_levels_update(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    c.bench_function("Wordlist::update (all correct)", |b| {
        b.iter_batched(
            || {
                let guessed = *WORDLIST.choose(&mut rng).unwrap();
                (DefaultLevels::default(), guessed, [LetterResponse::Correct; 5])
            },
            |(mut levels, guessed, responses)| black_box(levels.update_cont(1, guessed, responses)),
            BatchSize::SmallInput,
        )
    });
    c.bench_function("Wordlist::update (all included)", |b| {
        b.iter_batched(
            || {
                let guessed = *WORDLIST.choose(&mut rng).unwrap();
                (DefaultLevels::default(), guessed, [LetterResponse::Include; 5])
            },
            |(mut levels, guessed, responses)| black_box(levels.update_cont(1, guessed, responses)),
            BatchSize::SmallInput,
        )
    });
    c.bench_function("Wordlist::update (all excluded)", |b| {
        b.iter_batched(
            || {
                let guessed = *WORDLIST.choose(&mut rng).unwrap();
                (DefaultLevels::default(), guessed, [LetterResponse::Exclude; 5])
            },
            |(mut levels, guessed, responses)| black_box(levels.update_cont(1, guessed, responses)),
            BatchSize::SmallInput,
        )
    });
    c.bench_function("Wordlist::update (random)", |b| {
        b.iter_batched(
            || {
                let correct = *WORDLIST.choose(&mut rng).unwrap();
                let guessed = *WORDLIST.choose(&mut rng).unwrap();
                (DefaultLevels::default(), guessed, response_from(correct, guessed))
            },
            |(mut levels, guessed, responses)| black_box(levels.update_cont(1, guessed, responses)),
            BatchSize::SmallInput,
        )
    });
    c.bench_function("Wordlist::update (all correct)", |b| {
        b.iter_batched(
            || {
                let guessed = *WORDLIST.choose(&mut rng).unwrap();
                (DefaultLevels::default(), guessed, [LetterResponse::Correct; 5])
            },
            |(mut levels, guessed, responses)| black_box(levels.update_simd(1, guessed, responses)),
            BatchSize::SmallInput,
        )
    });
    c.bench_function("Wordlist::update (all included)", |b| {
        b.iter_batched(
            || {
                let guessed = *WORDLIST.choose(&mut rng).unwrap();
                (DefaultLevels::default(), guessed, [LetterResponse::Include; 5])
            },
            |(mut levels, guessed, responses)| black_box(levels.update_simd(1, guessed, responses)),
            BatchSize::SmallInput,
        )
    });
    c.bench_function("Wordlist::update (all excluded)", |b| {
        b.iter_batched(
            || {
                let guessed = *WORDLIST.choose(&mut rng).unwrap();
                (DefaultLevels::default(), guessed, [LetterResponse::Exclude; 5])
            },
            |(mut levels, guessed, responses)| black_box(levels.update_simd(1, guessed, responses)),
            BatchSize::SmallInput,
        )
    });
    c.bench_function("Wordlist::update (random)", |b| {
        b.iter_batched(
            || {
                let correct = *WORDLIST.choose(&mut rng).unwrap();
                let guessed = *WORDLIST.choose(&mut rng).unwrap();
                (DefaultLevels::default(), guessed, response_from(correct, guessed))
            },
            |(mut levels, guessed, responses)| black_box(levels.update_simd(1, guessed, responses)),
            BatchSize::SmallInput,
        )
    });
}

criterion_group! {
    name = wordle_bench;
    config = Criterion::default();
    targets = bench_wordbuf_update, bench_levels_update
}

criterion_main! {
    wordle_bench
}
