use std::marker::PhantomData;

use crate::{bag::Bag, pattern::Pattern, rule::Rule};

pub struct SharedSuffix<B: Bag>(pub PhantomData<B>);

impl<B: Bag> Rule<B> for SharedSuffix<B> {
    fn name(&self) -> &'static str {
        "shared_suffix"
    }

    fn apply(&self, pattern: &Pattern<B>) -> Pattern<B> {
        match pattern {
            Pattern::Either(box Pattern::Seq(t, b), box Pattern::Seq(u, c)) if b == c => {
                Pattern::Seq(
                    Box::new(Pattern::Group(Box::new(Pattern::Either(
                        t.clone(),
                        u.clone(),
                    )))),
                    b.clone(),
                )
            }
            _ => pattern.clone(),
        }
    }
}
