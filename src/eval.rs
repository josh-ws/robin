use crate::parse::{BinaryOp, Expr, ExprType, Numeric, UnaryOp};

pub fn eval(expr: &Expr) -> Result<Numeric, String> {
    match &expr.typ {
        ExprType::Number(n) => Ok(*n),
        ExprType::Binary { op, lhs, rhs } => {
            let (a, b) = (eval(lhs)?, eval(rhs)?);
            match op {
                BinaryOp::Add => match (a, b) {
                    (Numeric::Int(x), Numeric::Int(y)) => {
                        x.checked_add(y).map(Numeric::Int).ok_or("Overflow".to_string())
                    }
                },
                BinaryOp::Sub => match (a, b) {
                    (Numeric::Int(x), Numeric::Int(y)) => {
                        x.checked_sub(y).map(Numeric::Int).ok_or("Overflow".to_string())
                    }
                },
            }
        }
        ExprType::Unary { op, operand } => {
            let x = eval(operand)?;
            match op {
                UnaryOp::Neg => match x {
                    Numeric::Int(n) => n.checked_neg().map(Numeric::Int).ok_or("Overflow".to_string()),
                },
            }
        }
    }
}
