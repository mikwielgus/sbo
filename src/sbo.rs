// SPDX-FileCopyrightText: 2026 sbo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use core::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Clone, Copy)]
pub enum Sbo<S, L> {
    Small(S),
    Large(L),
}

impl<S: Default, L> Default for Sbo<S, L> {
    #[inline]
    fn default() -> Self {
        Sbo::Small(<S as Default>::default())
    }
}

impl<T, S: Deref<Target = T>, L: Deref<Target = T>> Deref for Sbo<S, L> {
    type Target = T;

    #[inline]
    fn deref(self: &Self) -> &Self::Target {
        match self {
            Sbo::Small(small) => small.deref(),
            Sbo::Large(large) => large.deref(),
        }
    }
}

impl<T, S: DerefMut<Target = T>, L: DerefMut<Target = T>> DerefMut for Sbo<S, L> {
    #[inline]
    fn deref_mut(self: &mut Self) -> &mut Self::Target {
        match self {
            Sbo::Small(small) => small.deref_mut(),
            Sbo::Large(large) => large.deref_mut(),
        }
    }
}

impl<I, O, S: Index<I, Output = O>, L: Index<I, Output = O>> Index<I> for Sbo<S, L> {
    type Output = O;

    fn index(&self, index: I) -> &Self::Output {
        match self {
            Sbo::Small(small) => small.index(index),
            Sbo::Large(large) => large.index(index),
        }
    }
}

impl<O, I, S: IndexMut<I, Output = O>, L: IndexMut<I, Output = O>> IndexMut<I> for Sbo<S, L> {
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        match self {
            Sbo::Small(small) => small.index_mut(index),
            Sbo::Large(large) => large.index_mut(index),
        }
    }
}
