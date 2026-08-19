use std::fmt;

use dashu::integer::IBig;

#[derive(Debug, Clone)]
pub enum Numeric {
    Int(i64),
    BigInt(IBig),
}

#[derive(Debug)]
pub enum Rung {
    Int(i64, i64),
    BigInt(IBig, IBig),
}

impl Numeric {
    pub fn from_big(v: IBig) -> Self {
        match i64::try_from(&v) {
            Ok(n) => Numeric::Int(n),
            Err(_) => Numeric::BigInt(v),
        }
    }

    pub fn into_big(self) -> IBig {
        match self {
            Numeric::Int(n) => IBig::from(n),
            Numeric::BigInt(n) => n,
        }
    }

    pub fn promote(a: Self, b: Self) -> Rung {
        match (a, b) {
            (Numeric::Int(x), Numeric::Int(y)) => Rung::Int(x, y),
            (x, y) => Rung::BigInt(x.into_big(), y.into_big()),
        }
    }

    pub fn add(self, rhs: Self) -> Self {
        match Self::promote(self, rhs) {
            Rung::Int(x, y) => match x.checked_add(y) {
                Some(n) => Numeric::Int(n),
                None => Numeric::from_big(IBig::from(x) + IBig::from(y)),
            },
            Rung::BigInt(x, y) => Numeric::from_big(x + y),
        }
    }

    pub fn sub(self, rhs: Self) -> Self {
        match Self::promote(self, rhs) {
            Rung::Int(x, y) => match x.checked_add(y) {
                Some(n) => Numeric::Int(n),
                None => Numeric::from_big(IBig::from(x) - IBig::from(y)),
            },
            Rung::BigInt(x, y) => Numeric::from_big(x - y),
        }
    }
}

impl fmt::Display for Numeric {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Numeric::Int(n) => write!(f, "{n}"),
            Numeric::BigInt(n) => write!(f, "{n}"),
        }
    }
}
