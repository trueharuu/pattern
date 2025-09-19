use std::marker::PhantomData;

use crate::{bag::Bag, pattern::Pattern, rule::Rule};

pub struct TakeMax<B: Bag>(pub PhantomData<B>);

impl<B: Bag> Rule<B> for TakeMax<B> {
    fn name(&self) -> &'static str {
        "take_max"
    }

    fn apply(&self, pattern: &Pattern<B>) -> Pattern<B> {
        match pattern {
            Pattern::Take(c, n) => {
                if c.queues().len() == *n {
                    return Pattern::All(c.clone());
                }

                return *c.clone()
            },
            _ => pattern.clone(),
        }
    }
}
