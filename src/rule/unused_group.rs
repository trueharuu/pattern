use std::marker::PhantomData;

use crate::{bag::Bag, pattern::Pattern, rule::Rule};

pub struct UnusedGroup<B: Bag>(pub PhantomData<B>);

impl<B: Bag> Rule<B> for UnusedGroup<B> {
    fn name(&self) -> &'static str {
        "unused_group"
    }

    fn apply(&self, pattern: &Pattern<B>) -> Pattern<B> {
        match pattern {
            Pattern::Group(c) => *c.clone(),
            _ => pattern.clone(),
        }
    }
}
