use crate::letter::Word;

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

    pub fn is_empty(&self) -> bool {
        self.len == 0
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

    pub unsafe fn get_unchecked(&self, ind: usize) -> &Word {
        self.words.get_unchecked(ind)
    }
}
