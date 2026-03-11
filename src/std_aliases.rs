// Unlicense — cochranblock.org
// Contributors: GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3
//! Std method aliases (f300+). f374 = iter.

/// f374 = iter
pub trait F374 {
    type Item;
    fn f374(&self) -> impl Iterator<Item = &Self::Item> + '_;
}

impl<T> F374 for [T] {
    type Item = T;
    fn f374(&self) -> impl Iterator<Item = &Self::Item> + '_ {
        self.iter()
    }
}

impl<T> F374 for Vec<T> {
    type Item = T;
    fn f374(&self) -> impl Iterator<Item = &Self::Item> + '_ {
        self.iter()
    }
}
