//! Crate providing traits to be used in the benches.
use ahash::AHashSet;
use hashbrown::HashSet as HashBrownHashSet;
use rustc_hash::FxHashSet;
use std::hash::Hasher;
use std::{collections::HashSet, hash::Hash};

pub trait Set<T> {
    fn create_new() -> Self;
    fn name() -> &'static str;
    fn insert_value(&mut self, value: T) -> bool;
}

impl<T: Hash + Eq> Set<T> for Vec<u64> {
    fn name() -> &'static str {
        "Vec"
    }

    fn create_new() -> Self {
        Vec::new()
    }

    fn insert_value(&mut self, value: T) -> bool {
        let mut hasher = ahash::AHasher::default();
        value.hash(&mut hasher);
        let hash = hasher.finish();

        if self.contains(&hash) {
            return false;
        }
        self.push(hash);
        true
    }
}

impl<T: Hash + Eq> Set<T> for HashSet<T> {
    fn name() -> &'static str {
        "std::collections::HashSet"
    }

    fn create_new() -> Self {
        HashSet::new()
    }

    fn insert_value(&mut self, value: T) -> bool {
        HashSet::insert(self, value)
    }
}

impl<T: Hash + Eq> Set<T> for HashBrownHashSet<T> {
    fn name() -> &'static str {
        "hashbrown::HashSet"
    }
    fn create_new() -> Self {
        HashBrownHashSet::new()
    }

    fn insert_value(&mut self, value: T) -> bool{
        HashBrownHashSet::insert(self, value)
    }
}

impl<T: Hash + Eq> Set<T> for FxHashSet<T> {
    fn name() -> &'static str {
        "rustc_hash::FxHashSet"
    }

    fn create_new() -> Self {
        FxHashSet::default()
    }

    fn insert_value(&mut self, value: T) -> bool {
        FxHashSet::insert(self, value)
    }
}

impl<T: Hash + Eq> Set<T> for AHashSet<T> {
    fn name() -> &'static str {
        "ahash::AHashSet"
    }

    fn create_new() -> Self {
        AHashSet::new()
    }

    fn insert_value(&mut self, value: T) -> bool {
        self.insert(value)
    }
}

impl<T: Hash + Ord + Eq> Set<T> for std::collections::BTreeSet<T> {
    fn name() -> &'static str {
        "std::collections::BTreeSet"
    }

    fn create_new() -> Self {
        std::collections::BTreeSet::new()
    }

    fn insert_value(&mut self, value: T) -> bool {
        std::collections::BTreeSet::insert(self, value)
    }
}


pub fn splitmix64(mut x: u64) -> u64 {
    x = x.wrapping_add(0x9e3779b97f4a7c15);
    x = (x ^ (x >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    x = (x ^ (x >> 27)).wrapping_mul(0x94d049bb133111eb);
    x ^ (x >> 31)
}

pub fn xorshift64(mut x: u64) -> u64 {
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    x
}