use super::SQLModule;
use crate::{
    ensure,
    error::{PlannerError, SQLPlannerResult},
    functions::SQLFunctions,
    invalid_operation_err,
};
use daft_dsl::{
    functions::{self, numeric::NumericExpr, FunctionExpr},
    ExprRef, LiteralValue,
};

pub struct SQLModuleNumeric;

/// SQLModule for FunctionExpr::Numeric
impl SQLModule for SQLModuleNumeric {
    fn register(parent: &mut SQLFunctions) {
        use FunctionExpr::Numeric as f;
        use NumericExpr::*;
        parent.add_fn("abs", f(Abs));
        parent.add_fn("ceil", f(Ceil));
        parent.add_fn("floor", f(Floor));
        parent.add_fn("sign", f(Sign));
        parent.add_fn("round", f(Round(0)));
        parent.add_fn("sqrt", f(Sqrt));
        parent.add_fn("sin", f(Sin));
        parent.add_fn("cos", f(Cos));
        parent.add_fn("tan", f(Tan));
        parent.add_fn("cot", f(Cot));
        parent.add_fn("asin", f(ArcSin));
        parent.add_fn("acos", f(ArcCos));
        parent.add_fn("atan", f(ArcTan));
        parent.add_fn("atan2", f(ArcTan2));
        parent.add_fn("radians", f(Radians));
        parent.add_fn("degrees", f(Degrees));
        parent.add_fn("log2", f(Log2));
        parent.add_fn("log10", f(Log10));
        // parent.add("log", f(Log(FloatWrapper(0.0))));
        parent.add_fn("ln", f(Ln));
        parent.add_fn("exp", f(Exp));
        parent.add_fn("atanh", f(ArcTanh));
        parent.add_fn("acosh", f(ArcCosh));
        parent.add_fn("asinh", f(ArcSinh));
    }
}

pub(crate) fn to_expr(expr: &NumericExpr, args: &[ExprRef]) -> SQLPlannerResult<ExprRef> {
    use functions::numeric::*;
    use NumericExpr::*;
    match expr {
        Abs => {
            ensure!(args.len() == 1, "abs takes exactly one argument");
            Ok(abs(args[0].clone()))
        }
        Ceil => {
            ensure!(args.len() == 1, "ceil takes exactly one argument");
            Ok(ceil(args[0].clone()))
        }
        Floor => {
            ensure!(args.len() == 1, "floor takes exactly one argument");
            Ok(floor(args[0].clone()))
        }
        Sign => {
            ensure!(args.len() == 1, "sign takes exactly one argument");
            Ok(sign(args[0].clone()))
        }
        Round(_) => {
            ensure!(args.len() == 2, "round takes exactly two arguments");
            let precision = match args[1].as_ref().as_literal() {
                Some(LiteralValue::Int32(i)) => *i,
                Some(LiteralValue::UInt32(u)) => *u as i32,
                Some(LiteralValue::Int64(i)) => *i as i32,
                _ => invalid_operation_err!("round precision must be an integer"),
            };
            Ok(round(args[0].clone(), precision))
        }
        Sqrt => {
            ensure!(args.len() == 1, "sqrt takes exactly one argument");
            Ok(sqrt(args[0].clone()))
        }
        Sin => {
            ensure!(args.len() == 1, "sin takes exactly one argument");
            Ok(sin(args[0].clone()))
        }
        Cos => {
            ensure!(args.len() == 1, "cos takes exactly one argument");
            Ok(cos(args[0].clone()))
        }
        Tan => {
            ensure!(args.len() == 1, "tan takes exactly one argument");
            Ok(tan(args[0].clone()))
        }
        Cot => {
            ensure!(args.len() == 1, "cot takes exactly one argument");
            Ok(cot(args[0].clone()))
        }
        ArcSin => {
            ensure!(args.len() == 1, "asin takes exactly one argument");
            Ok(arcsin(args[0].clone()))
        }
        ArcCos => {
            ensure!(args.len() == 1, "acos takes exactly one argument");
            Ok(arccos(args[0].clone()))
        }
        ArcTan => {
            ensure!(args.len() == 1, "atan takes exactly one argument");
            Ok(arctan(args[0].clone()))
        }
        ArcTan2 => {
            ensure!(args.len() == 2, "atan2 takes exactly two arguments");
            Ok(arctan2(args[0].clone(), args[1].clone()))
        }
        Degrees => {
            ensure!(args.len() == 1, "degrees takes exactly one argument");
            Ok(degrees(args[0].clone()))
        }
        Radians => {
            ensure!(args.len() == 1, "radians takes exactly one argument");
            Ok(radians(args[0].clone()))
        }
        Log2 => {
            ensure!(args.len() == 1, "log2 takes exactly one argument");
            Ok(log2(args[0].clone()))
        }
        Log10 => {
            ensure!(args.len() == 1, "log10 takes exactly one argument");
            Ok(log10(args[0].clone()))
        }
        Ln => {
            ensure!(args.len() == 1, "ln takes exactly one argument");
            Ok(ln(args[0].clone()))
        }
        Log(_) => {
            ensure!(args.len() == 2, "log takes exactly two arguments");
            let base = args[1]
                .as_literal()
                .and_then(|lit| match lit {
                    LiteralValue::Float64(f) => Some(*f),
                    LiteralValue::Int32(i) => Some(*i as f64),
                    LiteralValue::UInt32(u) => Some(*u as f64),
                    LiteralValue::Int64(i) => Some(*i as f64),
                    LiteralValue::UInt64(u) => Some(*u as f64),
                    _ => None,
                })
                .ok_or_else(|| PlannerError::InvalidOperation {
                    message: "log base must be a float or a number".to_string(),
                })?;

            Ok(log(args[0].clone(), base))
        }
        Exp => {
            ensure!(args.len() == 1, "exp takes exactly one argument");
            Ok(exp(args[0].clone()))
        }
        ArcTanh => {
            ensure!(args.len() == 1, "atanh takes exactly one argument");
            Ok(arctanh(args[0].clone()))
        }
        ArcCosh => {
            ensure!(args.len() == 1, "acosh takes exactly one argument");
            Ok(arccosh(args[0].clone()))
        }
        ArcSinh => {
            ensure!(args.len() == 1, "asinh takes exactly one argument");
            Ok(arcsinh(args[0].clone()))
        }
    }
}
