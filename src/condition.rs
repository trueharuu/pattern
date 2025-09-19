use std::{fmt::Display, marker::PhantomData};

use chumsky::{
    IterParser, Parser,
    error::Rich,
    extra::Err,
    prelude::{choice, group, just, recursive},
    text::digits,
};

use crate::{bag::Bag, pattern::Pattern, queue::Queue};

#[derive(Clone, Debug, PartialEq)]
pub enum Condition<B>
where
    B: Bag,
{
    Before(Pattern<B>, Pattern<B>), // A<B
    After(Pattern<B>, Pattern<B>),  // A>B
    Group(Box<Self>),               // (A)
    Count(Pattern<B>, usize),       // #A=N
    Location(Pattern<B>, usize),    // @A=N
    And(Box<Self>, Box<Self>),      // A&B
    Or(Box<Self>, Box<Self>),       // A|B
    // idk why but commenting this out causes like 600 type errors
    Phantom(PhantomData<B>),
}

impl<B> Condition<B>
where
    B: Bag,
{
    pub fn parser<'a>(
        recurse: impl Parser<'a, &'a str, Pattern<B>, Err<Rich<'a, char>>> + Clone + 'a,
    ) -> impl Parser<'a, &'a str, Self, Err<Rich<'a, char>>>
    where
        B: 'a,
    {
        recursive(|a| {
            let number = digits(10)
                .collect::<String>()
                .from_str::<usize>()
                .unwrapped();
            let before = group((recurse.clone(), just('<'), recurse.clone()))
                .map(|(a, _, b)| Self::Before(a, b));
            let after = group((recurse.clone(), just('>'), recurse.clone()))
                .map(|(a, _, b)| Self::After(a, b));
            let count = group((just('#'), recurse.clone(), just('='), number))
                .map(|(_, p, _, n)| Self::Count(p, n));
            let location = group((just('@'), recurse.clone(), just('='), number))
                .map(|(_, p, _, n)| Self::Location(p, n));

            let gr = a
                .delimited_by(just('('), just(')'))
                .map(|x| Self::Group(Box::new(x)));
            let atom = choice((count, location, before, after, gr)).boxed();

            let and = atom
                .clone()
                .foldl(just('&').then(atom.clone()).repeated(), |a, (_, b)| {
                    Self::And(Box::new(a), Box::new(b))
                })
                .or(atom.clone());

            let or = and
                .clone()
                .foldl(just('|').then(and.clone()).repeated(), |a, (_, b)| {
                    Self::Or(Box::new(a), Box::new(b))
                })
                .or(and.clone());

            choice((or, and, atom)).boxed()
        })
    }

    pub fn has(&self, queue: &Queue) -> bool {
        match self {
            Self::Group(p) => p.has(queue),
            Self::Before(a, b) => {
                let mut earliest_a: Option<usize> = None;
                let mut earliest_b: Option<usize> = None;

                for start in 0..=queue.len() {
                    for aq in a.queues() {
                        if queue.vec()[start..].starts_with(&aq.vec()) {
                            earliest_a = Some(start.min(earliest_a.unwrap_or(usize::MAX)));
                        }
                    }
                    for bq in b.queues() {
                        if queue.vec()[start..].starts_with(&bq.vec()) {
                            earliest_b = Some(start.min(earliest_b.unwrap_or(usize::MAX)));
                        }
                    }
                }

                match (earliest_a, earliest_b) {
                    (Some(a_idx), Some(b_idx)) => a_idx < b_idx,
                    (Some(_), None) => true, // `a` occurs, but `b` never does
                    _ => false,
                }
            }
            Self::After(a, b) => Self::Before(b.clone(), a.clone()).has(queue),
            Self::Count(pat, n) => {
                let mut count = 0;
                let mut start = 0;
                while start < queue.len() {
                    let mut matched = false;
                    for pq in pat.queues() {
                        if queue.vec()[start..].starts_with(&pq.vec()) {
                            count += 1;
                            start += pq.len();
                            matched = true;
                            break;
                        }
                    }
                    if !matched {
                        start += 1;
                    }
                }
                count == *n
            }
            Self::And(a, b) => a.has(queue) && b.has(queue),
            Self::Or(a, b) => a.has(queue) || b.has(queue),
            Self::Phantom(..) => unsafe { std::hint::unreachable_unchecked() },
            Self::Location(pat, n) => {
                if *n > queue.len() {
                    return false;
                }
                for pq in pat.queues() {
                    if queue.vec()[*n..].starts_with(&pq.vec()) {
                        return true;
                    }
                }
                false
            }
        }
    }

    pub fn size(&self) -> usize {
        match self {
            Self::After(a, b) | Self::Before(a, b) => 1 + a.size() + b.size(),
            Self::And(a, b) | Self::Or(a, b) => 1 + a.size() + b.size(),
            Self::Count(a, ..) | Self::Location(a, ..) => 1 + a.size(),
            Self::Group(a) => 1 + a.size(),
            Self::Phantom(..) => 0,
        }
    }
}

impl<B> Display for Condition<B>
where
    B: Bag,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Phantom(..) => unsafe { std::hint::unreachable_unchecked() },
                Self::Before(a, b) => format!("{a}<{b}"),
                Self::After(a, b) => format!("{a}>{b}"),
                Self::Count(a, b) => format!("#{a}={b}"),
                Self::And(a, b) => format!("{a}&{b}"),
                Self::Or(a, b) => format!("{a}|{b}"),
                Self::Group(p) => format!("({p})"),
                Self::Location(a, b) => format!("@{a}={b}"),
            }
        )
    }
}
