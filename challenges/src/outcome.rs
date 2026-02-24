use std::fmt;
use colored::Colorize;

// Used for challenges where a solution to obtain is given (e.g. Set 1 Challenge 1)
pub struct Verified<T> {
    pub result: T,
    pub expected: T,
    pub matches: bool
}

impl<T: fmt::Display> fmt::Display for Verified<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
       let icon = if self.matches { "✓".green() } else { "✗".red() };
        write!(f, "{icon}\n\tRes: {}\n\tExp: {}", self.result, self.expected)
    }
}

// Used for challenges where a solution should be discovered (e.g. Set 1 Challenge 3)
pub struct Discovered<T> {
    pub result: T
}

impl<T: fmt::Display> fmt::Display for Discovered<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\n  Recovered Solution: {}", self.result)
    }
}

pub enum Outcome<T> {
    Verified(Verified<T>),
    Discovered(Discovered<T>),
}

impl <T: fmt::Display + PartialEq> Outcome<T> {
    pub fn verified(result: T, expected: T) -> Self {
        let matches = result == expected;
        Self::Verified(Verified {matches, result, expected})
    }

    pub fn discovered(result: T) -> Self {
        Self::Discovered(Discovered { result })
    }

    pub fn is_ok(&self) -> bool {
        match self {
            Self::Verified(v) => v.matches,
            Self::Discovered(_) => true,
        }
    }
}

impl<T: fmt::Display + PartialEq> fmt::Display for Outcome<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Verified(v) => v.fmt(f),
            Self::Discovered(d) => d.fmt(f),
        }
    }
}