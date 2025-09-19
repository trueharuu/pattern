use std::{ops::Deref, str::FromStr, time::Instant};

use crate::{
    bag::Bag,
    pattern::{Optimization, Pattern},
};

#[derive(clap::Parser, Clone)]
pub struct Program<B>
where
    B: Bag,
{
    #[command(subcommand)]
    cmd: Cmd<B>,
    #[command(flatten)]
    extra: Extra,
}

#[derive(clap::Subcommand, Clone)]
pub enum Cmd<B>
where
    B: Bag,
{
    Expand {
        #[arg(short = 'p', long = "pattern")]
        pattern: Text<Pattern<B>>,
    },
    Ast {
        #[arg(short = 'p', long = "pattern")]
        pattern: Text<Pattern<B>>,
    },
    Count {
        #[arg(short = 'p', long = "pattern")]
        pattern: Text<Pattern<B>>,
    },
    Simplify {
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

#[derive(clap::Args, Clone)]
pub struct Extra {
    #[arg(short = 't', long = "timing", default_value_t = false)]
    timing: bool,
}

impl<B> Program<B>
where
    B: Bag,
{
    pub fn run(self) {
        let i = Instant::now();
        match self.cmd {
            Cmd::Expand { pattern } => {
                for q in pattern.queues() {
                    println!("{q:?}");
                }
            }
            Cmd::Simplify { pattern } => {
                let x = pattern.simplify();
                println!("{}", x);
            }
            Cmd::Ast { pattern } => {
                println!("{:#?}", *pattern);
            }
            Cmd::Count { pattern } => {
                println!("{}", pattern.count());
            }

            Cmd::Find {
                universe,
                set,
                opt_level,
            } => match Pattern::<B>::find(&universe.queues(), &set.queues(), opt_level) {
                Some(z) => println!("{z}"),
                None => eprintln!("failed to find a pattern"),
            },
        }

        if self.extra.timing {
            let el = i.elapsed();
            println!("finished in \x1b[33m{:.3}ms\x1b[0m", el.as_secs_f64() * 1000.0);
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
