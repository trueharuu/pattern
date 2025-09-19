use std::marker::PhantomData;

use crate::{bag::Bag, pattern::Pattern, rule::Rule};

pub struct DedupEither<B: Bag>(pub PhantomData<B>);

impl<B: Bag> Rule<B> for DedupEither<B> {
    fn name(&self) -> &'static str {
        "dedup_either"
    }

    fn apply(&self, pattern: &Pattern<B>) -> Pattern<B> {
        match pattern {
            Pattern::Either(left, right) => {
                let left_simplified = self.apply(left);
                let right_simplified = self.apply(right);

                // if both sides are identical, just return one
                if left_simplified.to_string() == right_simplified.to_string() {
                    return left_simplified;
                }

                Pattern::Either(Box::new(left_simplified), Box::new(right_simplified))
            }
            _ => pattern.clone(),
        }
    }
}
