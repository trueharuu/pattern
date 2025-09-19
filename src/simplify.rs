use std::marker::PhantomData;

use crate::{
    bag::Bag,
    pattern::Pattern,
    rule::{
        Rule, dedup_either::DedupEither, join_any_any::JoinAnyAny, join_single_any::JoinSingleAny,
        join_single_single::JoinSingleSingle, shared_prefix::SharedPrefix,
        shared_suffix::SharedSuffix, take_max::TakeMax, unused_group::UnusedGroup,
    },
};

impl<B> Pattern<B>
where
    B: Bag,
{
    pub fn rules() -> Vec<Box<dyn Rule<B>>> {
        vec![
            Box::new(SharedPrefix(PhantomData)),
            Box::new(SharedSuffix(PhantomData)),
            Box::new(DedupEither(PhantomData)),
            Box::new(JoinAnyAny(PhantomData)),
            Box::new(JoinSingleAny(PhantomData)),
            Box::new(JoinSingleSingle(PhantomData)),
            Box::new(TakeMax(PhantomData)),
            Box::new(UnusedGroup(PhantomData)),
        ]
    }

    pub fn simplify(&self) -> Self {
        let mut current = self.clone();
        loop {
            let next = current.simplify_one();
            // println!("{current} -> {next}");
            if next == current {
                break;
            }

            current = next;
        }

        current
    }
    pub fn simplify_one(&self) -> Self {
        let post = match self.clone() {
            Self::Single(c) => Self::Single(c),
            Self::Wildcard => Self::Wildcard,
            Self::Either(box a, box b) => {
                Self::Either(Box::new(a.simplify()), Box::new(b.simplify()))
            }
            Self::Seq(box a, box b) => Self::Seq(Box::new(a.simplify()), Box::new(b.simplify())),
            Self::Any(patterns) => Self::Any(patterns.iter().map(|p| p.simplify()).collect()),
            Self::Group(box c) => Self::Group(Box::new(c.simplify())),
            Self::Take(box i, c) => Self::Take(Box::new(i.simplify()), c),
            Self::All(box i) => Self::All(Box::new(i)),
            Self::Condition(box i, c) => Self::Condition(Box::new(i.simplify()), c),
            Self::Unique(c) => Self::Unique(Box::new(c.simplify())),
        };

        Self::apply_all_rules(&post)
    }

    fn apply_all_rules(pattern: &Self) -> Self {
        let mut current = pattern.clone();
        let mut changed = true;

        while changed {
            changed = false;

            for rule in Self::rules().iter() {
                let new_pattern = rule.apply(&current);

                // only accept the change if it maintains correctness
                if new_pattern != current {
                    println!("\x1b[36m{}\x1b[0m {current} -> {new_pattern}", rule.name());
                    current = new_pattern;
                    changed = true;
                    break; // restart with all rules on the new pattern
                }
            }
        }

        current
    }
}
