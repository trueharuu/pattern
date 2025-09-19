use std::collections::HashSet;
use crate::bag::Bag;
use crate::pattern::Pattern;
use crate::queue::Queue;

impl<B> Pattern<B>
where
    B: Bag,
{
    pub fn find_exhaustive(universe: &[Queue], set: &[Queue]) -> Option<Self> {
        // early return if set is empty or all queues are in set
        if set.is_empty() {
            return None;
        }

        // sanity check
        let set_queues: HashSet<&Queue> = set.iter().collect();
        let universe_queues: HashSet<&Queue> = universe.iter().collect();

        if !set_queues.is_subset(&universe_queues) {
            println!("no sanity");
            return None;
        }

        // turn set into A;B;C;D;...
        let initial_pattern = Self::create_literal_pattern(set);

        // sanity check
        if !initial_pattern.check(universe, set) {
            println!("no sanity");
            return None;
        }

        // simplify forever
        Some(initial_pattern.simplify())
    }

    fn create_literal_pattern(queues: &[Queue]) -> Self {
        if queues.is_empty() {
            unreachable!()
        }

        if queues.len() == 1 {
            return Self::queue_to_pattern(&queues[0]);
        }

        // create Either pattern with all queues
        let mut result = Self::queue_to_pattern(&queues[0]);
        for queue in &queues[1..] {
            let queue_pattern = Self::queue_to_pattern(queue);
            result = Pattern::Either(Box::new(result), Box::new(queue_pattern));
        }

        result
    }

    fn queue_to_pattern(queue: &Queue) -> Self {
        let chars = Self::queue_to_chars(queue);

        if chars.is_empty() {
            return Pattern::Wildcard;
        }

        if chars.len() == 1 {
            return Pattern::Single(chars[0]);
        }

        // build sequence: first char, then rest
        let mut result = Pattern::Single(chars[0]);
        for &c in &chars[1..] {
            result = Pattern::Seq(Box::new(result), Box::new(Pattern::Single(c)));
        }

        result
    }

    fn queue_to_chars(queue: &Queue) -> Vec<char> {
        queue.vec().clone()
    }
}

