use std::fmt::Debug;

use itertools::Itertools;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Queue(Vec<char>);

impl Queue {
    pub fn new(values: Vec<char>) -> Self {
        Self(values)
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn nth(&self, u: usize) -> Option<char> {
        self.0.get(u).copied()
    }

    pub fn join(&self, u: Self) -> Self {
        Self([self.0.clone(), u.0].concat())
    }

    pub fn vec(&self) -> &Vec<char> {
        &self.0
    }

    pub fn par(self, location: usize) -> (Queue, Queue) {
        (
            Self(self.0[0..location].to_vec()),
            Self(self.0[location..].to_vec()),
        )
    }

    pub fn slice(&self, start: usize, end: usize) -> Queue {
        Self(self.0[start..end].to_vec())
    }


    pub fn loose_eq(&self, rhs: &Self) -> bool {
        self.0.iter().counts() == rhs.0.iter().counts()    
    }
}

impl Debug for Queue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.iter().collect::<String>())
    }
}
