use std::marker::PhantomData;

use crate::{bag::Bag, pattern::Pattern, rule::Rule};

pub struct JoinAnyAny<B: Bag>(pub PhantomData<B>);

impl<B: Bag> Rule<B> for JoinAnyAny<B> {
    fn name(&self) -> &'static str {
        "join_any_any"
    }

    fn apply(&self, pattern: &Pattern<B>) -> Pattern<B> {
        match pattern {
             Pattern::Either(box Pattern::Any(a), box Pattern::Any(c)) => {
                return Pattern::Any([a.clone(), c.clone()].concat());
            }
            _ => pattern.clone(),
        }
    }
}
