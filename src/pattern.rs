use std::{collections::HashSet, convert::Infallible, fmt::Display, str::FromStr};

use chumsky::{
    IterParser, Parser,
    error::Rich,
    extra::Err,
    prelude::{choice, group, just, one_of, recursive},
    text::digits,
};
use itertools::Itertools;

use crate::{bag::Bag, condition::Condition, util::FullDedup, queue::Queue};

#[derive(Clone, Debug, PartialEq)]
pub enum Pattern<B>
where
    B: Bag,
{
    Single(char),                            // T
    Either(Box<Self>, Box<Self>),            // T;O
    Seq(Box<Self>, Box<Self>),               // TO
    Any(Vec<Self>),                          // [TO]
    Group(Box<Self>),                        // (TO)
    Wildcard,                                // *
    Take(Box<Self>, usize),                  // T2
    All(Box<Self>),                          // [T]!
    Condition(Box<Self>, Box<Condition<B>>), // T{P}
    Unique(Box<Self>),                       // T?
}

impl<B> Pattern<B>
where
    B: Bag,
{
    pub fn new(i: impl Display) -> Result<Self, String> {
        let v = i.to_string();
        let p = Self::parser();
        let r = p.parse(&v);
        if r.has_errors() {
            for e in r.into_errors() {
                return Err(e.to_string());
            }

            std::process::exit(1);
        }

        Ok(p.parse(&v).unwrap())
    }

    pub fn parser<'a>() -> impl Parser<'a, &'a str, Self, Err<Rich<'a, char>>>
    where
        B: 'a,
    {
        recursive(|a| {
            let number = digits(10)
                .collect::<String>()
                .from_str::<usize>()
                .unwrapped();
            let wildcard = just('*').map(|_| Self::Wildcard);
            let single = one_of(B::wildcard()).map(Self::Single);
            let gr = a
                .delimited_by(just('('), just(')'))
                .map(|x| Self::Group(Box::new(x)));

            let atom = choice((single, wildcard, gr)).boxed();

            let any = atom
                .clone()
                .repeated()
                .collect()
                .delimited_by(just('['), just(']'))
                .map(|x| Self::Any(x))
                .or(atom.clone())
                .boxed();

            let all = group((any.clone(), just('!')))
                .map(|(a, _)| Self::All(Box::new(a)))
                .or(any.clone())
                .boxed();

            let take = group((all.clone(), just('p').or_not(), number))
                .map(|(a, _, b)| Self::Take(Box::new(a), b))
                .or(all.clone())
                .boxed();

            let cond = group((
                take.clone(),
                just('{'),
                Condition::parser(take.clone()),
                just('}'),
            ))
            .map(|(a, _, b, _)| Self::Condition(Box::new(a), Box::new(b)))
            .boxed()
            .or(take.clone());

            let seq = cond
                .clone()
                .foldl(
                    just(',').or_not().ignore_then(cond.clone()).repeated(),
                    |a, b| Self::Seq(Box::new(a), Box::new(b)),
                )
                .or(cond.clone())
                .boxed();

            let unique = group((seq.clone(), just('?')))
                .map(|(x, _)| Self::Unique(Box::new(x)))
                .or(seq.clone());

            let either = unique
                .clone()
                .foldl(
                    just(';').or(just('\n')).then(unique.clone()).repeated(),
                    |a, (_, b)| Self::Either(Box::new(a), Box::new(b)),
                )
                .or(unique.clone())
                .boxed();

            choice((either, seq, cond, take, all, any, atom)).boxed()
        })
    }

    pub fn count(&self) -> usize {
        match self {
            // Self::Phantom(..) => unsafe { std::hint::unreachable_unchecked() },
            Self::Single(..) => 1,
            Self::Either(t, u) => t.count() + u.count(),
            Self::Seq(t, u) => t.count() * u.count(),
            Self::Any(t) => t.iter().map(|x| x.count()).sum(),
            Self::Group(p) => p.count(),
            Self::Wildcard => B::wildcard().len(),
            Self::Take(p, n) => {
                let m = p.count();
                if *n > m { 0 } else { (m - n + 1..=m).product() }
            }

            Self::All(p) => (1..=p.count()).product(),
            Self::Condition(..) | Self::Unique(..) => self.queues().len(),
        }
    }

    pub fn set(&self) -> HashSet<Queue> {
        self.queues().into_iter().collect()
    }

    pub fn queues(&self) -> Vec<Queue> {
        match self {
            // Self::Phantom(..) => unsafe { std::hint::unreachable_unchecked() },
            Self::Single(c) => vec![Queue::new(vec![*c])],
            Self::Either(t, u) => [t.queues(), u.queues()].concat(),
            Self::Seq(t, u) => {
                let mut v = vec![];
                for tq in t.queues() {
                    for uq in u.queues() {
                        v.push(tq.join(uq))
                    }
                }

                v
            }

            Self::Any(t) => {
                let mut v = vec![];
                for tq in t {
                    for q in tq.queues() {
                        v.push(q);
                    }
                }

                v
            }

            Self::Group(c) => c.queues(),
            Self::Wildcard => B::wildcard().iter().map(|x| Queue::new(vec![*x])).collect(),
            Self::Take(c, b) => c
                .queues()
                .into_iter()
                .permutations(*b)
                .map(|x| Queue::new(x.into_iter().map(|x| x.vec().clone()).flatten().collect()))
                .collect::<Vec<_>>()
                .full_dedup(),
            Self::All(c) => {
                let q = c.queues();
                let l = q.len();
                q.iter()
                    .permutations(l)
                    .map(|x| Queue::new(x.into_iter().map(|x| x.vec().clone()).flatten().collect()))
                    .collect::<Vec<_>>()
                    .full_dedup()
            }
            Self::Condition(p, c) => p.queues().into_iter().filter(|x| c.has(x)).collect(),
            Self::Unique(p) => {
                let mut v = vec![];
                for q in p.queues() {
                    if !v.iter().any(|x| q.loose_eq(x)) {
                        v.push(q);
                    }
                }

                v
            }
        }
    }

    pub fn find(universe: &[Queue], set: &[Queue], opt_level: Optimization) -> Option<Self> {
        match opt_level {
            Optimization::Exhaustive => Self::find_exhaustive(universe, set),
            _ => todo!(),
        }
    }

    // Checks if candidate matches all of `set` and none outside of `set` in `universe`
    pub fn check(&self, _universe: &[Queue], set: &[Queue]) -> bool {
        let qs = self.queues();

        qs.iter().all(|x| set.contains(x)) && !qs.iter().any(|x| !set.contains(x))
    }

    pub fn size(&self) -> usize {
        match self {
            Self::All(c) => 1 + c.size(),
            Self::Any(c) => 1 + c.len(),
            Self::Condition(c, p) => 1 + c.size() + p.size(),
            Self::Either(a, b) => 1 + a.size() + b.size(),
            Self::Group(c) => 1 + c.size(),
            // Self::Phantom(..) => 1,
            Self::Seq(a, b) => 1 + a.size() + b.size(),
            Self::Single(..) => 1,
            Self::Take(a, ..) => 1 + a.size(),
            Self::Unique(a) => 1 + a.size(),
            Self::Wildcard => 1,
        }
    }
}

impl<B> Display for Pattern<B>
where
    B: Bag,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                // Self::Phantom(..) => String::new(),
                Self::Single(p) => format!("{p}"),
                Self::Either(t, u) => format!("{t};{u}"),
                Self::Seq(t, u) => format!("{t}{u}"),
                Self::Any(t) => format!(
                    "[{}]",
                    t.iter().map(|x| x.to_string()).collect::<Vec<_>>().join("")
                ),
                Self::Group(p) => format!("({p})"),
                Self::Wildcard => "*".to_string(),
                Self::Take(p, n) => format!("{p}{n}"),
                Self::All(p) => format!("{p}!"),
                Self::Condition(p, c) => format!("{p}{{{c}}}"),
                Self::Unique(p) => format!("{p}?"),
            }
        )
    }
}

impl<B> FromStr for Pattern<B>
where
    B: Bag,
{
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Optimization {
    None,
    Exhaustive,
}

impl FromStr for Optimization {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Self::None),
            _ => Ok(Self::Exhaustive),
        }
    }
}
