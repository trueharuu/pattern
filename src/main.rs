use clap::Parser;

use crate::{bag::Bag7, program::Program};

pub mod bag;
pub mod condition;
pub mod dedup;
pub mod find;
pub mod pattern;
pub mod program;
pub mod queue;

fn main() {
    Program::<Bag7>::parse().run();
}
