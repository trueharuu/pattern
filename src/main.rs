#![feature(box_patterns)]

use clap::Parser;

use crate::{bag::Bag7, program::Program};

pub mod bag;
pub mod condition;
pub mod util;
pub mod find;
pub mod pattern;
pub mod program;
pub mod queue;
pub mod rule;
pub mod simplify;

fn main() {
    Program::<Bag7>::parse().run();
}
