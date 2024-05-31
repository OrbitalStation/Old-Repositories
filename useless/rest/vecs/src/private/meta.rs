use crate::*;

pub auto trait NotTuple {}

impl !NotTuple for () {}

impl <T> !NotTuple for (T,) {}

impl <A, B> !NotTuple for (A, B) {}

impl <A, B, C> !NotTuple for (A, B, C) {}

impl <A, B, C, D> !NotTuple for (A, B, C, D) {}

pub auto trait NotList {}

impl <T, const N: usize> !NotList for [T; N] {}

pub auto trait NotVec {}

impl <T, const N: usize> !NotVec for vec <T, N> {}
