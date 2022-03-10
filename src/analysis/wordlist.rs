use crate::letter::Word;
use std::{
    borrow::{Borrow, BorrowMut},
    ops::{Deref, DerefMut},
};

#[derive(Clone, Copy, Debug)]
pub struct WordList<const N: usize> {
    words: [Word; N],
    len: usize,
}

impl<const N: usize> WordList<N> {
    pub const fn from_const(words: [Word; N]) -> Self {
        Self { words, len: N }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    unsafe fn set_len(&mut self, len: usize) {
        self.len = len;
    }

    pub fn clear(&mut self) {
        unsafe { self.set_len(0) };
    }

    pub unsafe fn push_unchecked(&mut self, new: Word) {
        *self.words.get_unchecked_mut(self.len) = new;
        self.set_len(self.len + 1);
    }
}

impl<const N: usize> Deref for WordList<N> {
    type Target = [Word];

    #[inline]
    fn deref(&self) -> &[Word] {
        &self.words[0..self.len]
    }
}

impl<const N: usize> DerefMut for WordList<N> {
    #[inline]
    fn deref_mut(&mut self) -> &mut [Word] {
        &mut self.words[0..self.len]
    }
}

impl<const N: usize> AsRef<[Word]> for WordList<N> {
    #[inline]
    fn as_ref(&self) -> &[Word] {
        self
    }
}

impl<const N: usize> AsMut<[Word]> for WordList<N> {
    #[inline]
    fn as_mut(&mut self) -> &mut [Word] {
        self
    }
}

impl<const N: usize> Borrow<[Word]> for WordList<N> {
    #[inline]
    fn borrow(&self) -> &[Word] {
        self
    }
}

impl<const N: usize> BorrowMut<[Word]> for WordList<N> {
    #[inline]
    fn borrow_mut(&mut self) -> &mut [Word] {
        self
    }
}
