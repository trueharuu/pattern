use std::{ops::Deref, str::FromStr};

use crate::{bag::Bag, find::Optimization, pattern::Pattern};

#[derive(clap::Parser)]
pub enum Program<B>
where
    B: Bag,
{
    Expand {
        #[arg(short = 'p', long = "pattern")]
        pattern: Text<Pattern<B>>,
    },
    Count {
        #[arg(short = 'p', long = "pattern")]
        pattern: Text<Pattern<B>>,
    },
    Find {
        #[arg(short = 'u', long = "universe")]
        universe: Text<Pattern<B>>,
        #[arg(short = 's', long = "set")]
        set: Text<Pattern<B>>,
        #[arg(short = 'O', long = "opt-level")]
        opt_level: Optimization,
    },
}

impl<B> Program<B>
where
    B: Bag,
{
    pub fn run(self) {
        match self {
            Self::Expand { pattern } => {
                for q in pattern.queues() {
                    println!("{q:?}");
                }
            }
            Self::Count { pattern } => {
                println!("{}", pattern.count());
            }

            Self::Find {
                universe,
                set,
                opt_level,
            } => {
                match Pattern::<B>::find(&universe.queues(), &set.queues(), opt_level) {
                    Some(z) => println!("{z}"),
                    None => eprintln!("failed to find a pattern"),
                }

            }
        }
    }
}

#[derive(Clone)]
pub struct Text<T>(T);
impl<T> Deref for Text<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> FromStr for Text<T>
where
    T: FromStr,
{
    type Err = T::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(path) = s.strip_prefix("file:") {
            Ok(Self(T::from_str(&std::fs::read_to_string(path).unwrap())?))
        } else {
            Ok(Self(T::from_str(s)?))
        }
    }
}
