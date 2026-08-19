use crate::{
    numeric::Numeric,
    parse::{BinaryOp, Expr, ExprType, UnaryOp},
};

pub fn eval(expr: &Expr) -> Result<Numeric, String> {
    match &expr.typ {
        ExprType::Number(n) => Ok(n.clone()),
        ExprType::Binary { op, lhs, rhs } => {
            let (a, b) = (eval(lhs)?, eval(rhs)?);
            match op {
                BinaryOp::Add => Ok(a.add(b)),
                BinaryOp::Sub => Ok(a.sub(b)),
                BinaryOp::Mul => Ok(a.mul(b)),
                BinaryOp::Pow => Ok(a.pow(b)),
            }
        }
        ExprType::Unary { op, operand } => {
            unimplemented!();
        }
    }
}
