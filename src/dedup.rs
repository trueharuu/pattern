use std::{collections::HashSet, hash::Hash};

pub trait FullDedup {
    fn full_dedup<T>(self) -> Self
    where
        Self: IntoIterator<Item = T> + FromIterator<T>,
        T: Eq + Hash + Clone,
    {
        let mut seen = HashSet::new();
        self
            .into_iter()
            .filter_map(|item| {
                if seen.insert(item.clone()) {
                    Some(item)
                } else {
                    None
                }
            }) // Keep only unique items
            .collect() // Collect back into the original collection type
    }
}

impl<T> FullDedup for T where Self: IntoIterator + FromIterator<<T as IntoIterator>::Item> {}