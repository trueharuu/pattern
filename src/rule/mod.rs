pub mod dedup_either; // A;A => A
pub mod join_any_any;
pub mod join_single_any;
pub mod join_single_single;
pub mod shared_prefix; // AB;AC => A(B;C)
pub mod take_max;
pub mod unused_group;
pub mod shared_suffix; // (A) => A


use crate::{bag::Bag, pattern::Pattern};

pub trait Rule<B>
where
    B: Bag,
    Self: Sync + Send,
{
    fn name(&self) -> &'static str;
    fn apply(&self, pat: &Pattern<B>) -> Pattern<B>;
}
