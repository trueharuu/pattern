use std::marker::PhantomData;

use crate::{bag::Bag, pattern::Pattern, rule::Rule};

pub struct SharedPrefix<B: Bag>(pub PhantomData<B>);

impl<B: Bag> Rule<B> for SharedPrefix<B> {
    fn name(&self) -> &'static str {
        "shared_prefix"
    }

    fn apply(&self, pattern: &Pattern<B>) -> Pattern<B> {
        match pattern {
            Pattern::Either(box Pattern::Seq(t, b), box Pattern::Seq(u, c)) if t == u => {
                Pattern::Seq(
                    t.clone(),
                    Box::new(Pattern::Group(Box::new(Pattern::Either(
                        b.clone(),
                        c.clone(),
                    )))),
                )
            }
            _ => pattern.clone(),
        }
    }
}
