use std::{fmt, ops::Neg};

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
            Rung::Int(x, y) => match x.checked_sub(y) {
                Some(n) => Numeric::Int(n),
                None => Numeric::from_big(IBig::from(x) - IBig::from(y)),
            },
            Rung::BigInt(x, y) => Numeric::from_big(x - y),
        }
    }

    pub fn mul(self, rhs: Self) -> Self {
        match Self::promote(self, rhs) {
            Rung::Int(x, y) => match x.checked_mul(y) {
                Some(n) => Numeric::Int(n),
                None => Numeric::from_big(IBig::from(x) * IBig::from(y)),
            },
            Rung::BigInt(x, y) => Numeric::from_big(x * y),
        }
    }

    pub fn pow(self, rhs: Self) -> Self {
        let exp = match rhs {
            Numeric::Int(e) => e,
            _ => todo!("overflow on pow rhs, this should be capped"),
        };
        if exp < 0 {
            todo!("negative exponent");
        }
        if exp == 0 && self.is_zero() {
            todo!("0 pow 0, this should return an error")
        }

        let exp = exp as usize;
        Numeric::from_big(self.clone().into_big().pow(exp)) // TODO(jw) PERF don't always promote to bigint; pointless.
    }

    pub fn neg(self) -> Self {
        match self {
            Numeric::Int(x) => match x.checked_neg() {
                Some(result) => Numeric::Int(result),
                None => Numeric::from_big(IBig::from(x).neg()),
            },
            Numeric::BigInt(x) => Numeric::BigInt(x.neg()),
        }
    }

    fn is_zero(&self) -> bool {
        match self {
            Numeric::Int(n) => *n == 0,
            Numeric::BigInt(_) => false,
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
