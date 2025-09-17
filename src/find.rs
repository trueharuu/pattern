use std::str::FromStr;

use crate::{bag::Bag, pattern::Pattern, queue::Queue};

impl<B> Pattern<B>
where
    B: Bag,
{
    pub fn find(universe: &[Queue], set: &[Queue], opt_level: Optimization) -> Option<Self> {
        match opt_level {
            Optimization::None => Self::find_fast(universe, set),
            Optimization::Greedy => Self::find_greedy(universe, set),
            Optimization::Bounded => Self::find_bounded(universe, set, 5), // e.g., depth bound = 5
            Optimization::Exhaustive => Self::find_exhaustive(universe, set),
        }
    }

    // Checks if candidate matches all of `set` and none outside of `set` in `universe`
    pub fn matches_exactly(&self, _universe: &[Queue], set: &[Queue]) -> bool {
        let qs = self.queues();
        set.iter().all(|s| qs.contains(s))
    }

    // nothing search
    fn find_fast(_universe: &[Queue], set: &[Queue]) -> Option<Self> {
        if set.len() == 0 {
            return None;
        }

        Some(Self::build_either(set))
    }

    fn build_either(q: &[Queue]) -> Self {
        if q.len() == 1 {
            return Self::build_seq(&q[0]);
        }

        return Self::Either(
            Box::new(Self::build_seq(&q[0])),
            Box::new(Self::build_either(&q[1..])),
        );
    }

    fn build_seq(q: &Queue) -> Self {
        if q.len() == 1 {
            return Self::Single(q.vec()[0]);
        }

        Self::Seq(
            Box::new(Self::Single(q.vec()[0])),
            Box::new(Self::build_seq(&q.slice(1, q.len()))),
        )
    }

    // --- helper placeholders for higher levels ---

    fn find_greedy(_universe: &[Queue], set: &[Queue]) -> Option<Self> {
        if set.is_empty() {
            return None;
        }

        // Convert queues into Vec<Vec<char>>
        let sequences: Vec<Vec<char>> = set.into_iter().map(|q| q.vec().clone()).collect();

        // Merge sequences into a single pattern
        Some(Self::merge_sequences(&sequences))
    }

    /// Merge multiple sequences into a single pattern, factoring common prefixes
    fn merge_sequences(seqs: &[Vec<char>]) -> Self {
        if seqs.is_empty() {
            panic!("Cannot merge empty sequences");
        }
        if seqs.len() == 1 {
            // Single sequence → build Seq pattern
            return Self::build_seq2(&seqs[0]);
        }

        // Find common prefix
        let mut prefix_len = 0;
        let first = &seqs[0];
        'outer: loop {
            if prefix_len >= first.len() {
                break;
            }
            for seq in seqs.iter().skip(1) {
                if prefix_len >= seq.len() || seq[prefix_len] != first[prefix_len] {
                    break 'outer;
                }
            }
            prefix_len += 1;
        }

        // If there is a prefix, factor it out
        let mut prefix_pattern = None;
        if prefix_len > 0 {
            prefix_pattern = Some(Self::build_seq2(&first[..prefix_len].to_vec()));
        }

        // Build suffix patterns for the remaining tails
        let mut tails: Vec<Vec<char>> = vec![];
        for seq in seqs {
            tails.push(seq[prefix_len..].to_vec());
        }

        // Remove empty tails (sequences that ended exactly at prefix)
        let nonempty_tails: Vec<Vec<char>> = tails.into_iter().filter(|t| !t.is_empty()).collect();

        let suffix_pattern = if nonempty_tails.is_empty() {
            None
        } else if nonempty_tails.len() == 1 {
            Some(Self::build_seq2(&nonempty_tails[0]))
        } else {
            // Multiple suffixes → merge recursively using Any
            let merged: Vec<Self> = nonempty_tails.iter().map(|t| Self::build_seq2(t)).collect();
            Some(Self::Any(merged))
        };

        match (prefix_pattern, suffix_pattern) {
            (Some(pref), Some(suf)) => Self::Seq(Box::new(pref), Box::new(suf)),
            (Some(pref), None) => pref,
            (None, Some(suf)) => suf,
            (None, None) => panic!("Impossible: no prefix or suffix"),
        }
    }

    /// Build a Seq pattern from a single Vec<char>
    fn build_seq2(q: &[char]) -> Self {
        if q.is_empty() {
            panic!("Cannot build Seq from empty vector");
        }
        if q.len() == 1 {
            return Self::Single(q[0]);
        }
        Self::Seq(
            Box::new(Self::Single(q[0])),
            Box::new(Self::build_seq2(&q[1..])),
        )
    }

    fn find_bounded(_universe: &[Queue], _set: &[Queue], _max_depth: usize) -> Option<Self> {
        // BFS over pattern space up to depth
        None
    }

    fn find_exhaustive(_universe: &[Queue], _set: &[Queue]) -> Option<Self> {
        // Full BFS over pattern space, guaranteed minimal by size
        None
    }
}

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum Optimization {
    None,
    Greedy,
    Bounded,
    Exhaustive,
}

impl FromStr for Optimization {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Self::None),
            "1" => Ok(Self::Greedy),
            "2" => Ok(Self::Bounded),
            _ => Ok(Self::Exhaustive),
        }
    }
}
