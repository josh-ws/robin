use crate::parse::{Expr, ExprType, Numeric, OpType};

pub fn eval(expr: &Expr) -> Result<Numeric, String> {
    match &expr.typ {
        ExprType::Number(n) => Ok(*n),
        ExprType::Binary { op, lhs, rhs } => {
            let (a, b) = (eval(lhs)?, eval(rhs)?);
            match op.typ {
                OpType::Plus => match (a, b) {
                    (Numeric::Int(x), Numeric::Int(y)) => {
                        x.checked_add(y).map(Numeric::Int).ok_or("Overflow".to_string())
                    }
                },
            }
        }
    }
}
