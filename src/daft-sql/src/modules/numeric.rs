use daft_dsl::{ExprRef, LiteralValue};
use daft_functions::numeric::{
    abs::abs,
    ceil::ceil,
    clip::clip,
    exp::{exp, expm1},
    floor::floor,
    log::{ln, log, log10, log1p, log2},
    round::round,
    sign::{negative, sign},
    sqrt::sqrt,
    trigonometry::{
        arccos, arccosh, arcsin, arcsinh, arctan, arctanh, atan2, cos, cosh, cot, csc, degrees,
        radians, sec, sin, sinh, tan, tanh,
    },
};

use super::SQLModule;
use crate::{
    ensure,
    error::{PlannerError, SQLPlannerResult},
    functions::{SQLFunction, SQLFunctions},
    invalid_operation_err,
};

pub struct SQLModuleNumeric;

impl SQLModule for SQLModuleNumeric {
    fn register(parent: &mut SQLFunctions) {
        parent.add_fn("abs", SQLNumericExpr::Abs);
        parent.add_fn("ceil", SQLNumericExpr::Ceil);
        parent.add_fn("floor", SQLNumericExpr::Floor);
        parent.add_fn("sign", SQLNumericExpr::Sign);
        parent.add_fn("signum", SQLNumericExpr::Signum);
        parent.add_fn("negate", SQLNumericExpr::Negate);
        parent.add_fn("negative", SQLNumericExpr::Negative);
        parent.add_fn("round", SQLNumericExpr::Round);
        parent.add_fn("clip", SQLNumericExpr::Clip);
        parent.add_fn("sqrt", SQLNumericExpr::Sqrt);
        parent.add_fn("sin", SQLNumericExpr::Sin);
        parent.add_fn("cos", SQLNumericExpr::Cos);
        parent.add_fn("tan", SQLNumericExpr::Tan);
        parent.add_fn("csc", SQLNumericExpr::Csc);
        parent.add_fn("sec", SQLNumericExpr::Sec);
        parent.add_fn("cot", SQLNumericExpr::Cot);
        parent.add_fn("sinh", SQLNumericExpr::Sinh);
        parent.add_fn("cosh", SQLNumericExpr::Cosh);
        parent.add_fn("tanh", SQLNumericExpr::Tanh);
        parent.add_fn("asin", SQLNumericExpr::ArcSin);
        parent.add_fn("acos", SQLNumericExpr::ArcCos);
        parent.add_fn("atan", SQLNumericExpr::ArcTan);
        parent.add_fn("atan2", SQLNumericExpr::ArcTan2);
        parent.add_fn("radians", SQLNumericExpr::Radians);
        parent.add_fn("degrees", SQLNumericExpr::Degrees);
        parent.add_fn("log2", SQLNumericExpr::Log2);
        parent.add_fn("log10", SQLNumericExpr::Log10);
        parent.add_fn("log", SQLNumericExpr::Log);
        parent.add_fn("ln", SQLNumericExpr::Ln);
        parent.add_fn("log1p", SQLNumericExpr::Log1p);
        parent.add_fn("exp", SQLNumericExpr::Exp);
        parent.add_fn("expm1", SQLNumericExpr::Expm1);
        parent.add_fn("atanh", SQLNumericExpr::ArcTanh);
        parent.add_fn("acosh", SQLNumericExpr::ArcCosh);
        parent.add_fn("asinh", SQLNumericExpr::ArcSinh);
    }
}
enum SQLNumericExpr {
    Abs,
    Ceil,
    Exp,
    Expm1,
    Floor,
    Round,
    Clip,
    Sign,
    Signum,
    Negate,
    Negative,
    Sqrt,
    Sin,
    Cos,
    Tan,
    Csc,
    Sec,
    Cot,
    Sinh,
    Cosh,
    Tanh,
    ArcSin,
    ArcCos,
    ArcTan,
    ArcTan2,
    Radians,
    Degrees,
    Log,
    Log2,
    Log10,
    Ln,
    Log1p,
    ArcTanh,
    ArcCosh,
    ArcSinh,
}

impl SQLFunction for SQLNumericExpr {
    fn to_expr(
        &self,
        inputs: &[sqlparser::ast::FunctionArg],
        planner: &crate::planner::SQLPlanner,
    ) -> SQLPlannerResult<ExprRef> {
        let inputs = self.args_to_expr_unnamed(inputs, planner)?;
        to_expr(self, inputs.as_slice())
    }

    fn docstrings(&self, _alias: &str) -> String {
        let docstring = match self {
            Self::Abs => "Gets the absolute value of a number.",
            Self::Ceil => "Rounds a number up to the nearest integer.",
            Self::Exp => "Calculates the exponential of a number (e^x).",
            Self::Expm1 => "Calculates the exponential of a number minus one (e^x - 1).",
            Self::Floor => "Rounds a number down to the nearest integer.",
            Self::Round => "Rounds a number to a specified number of decimal places.",
            Self::Clip => "Clips a number to a specified range. If left bound is None, no lower clipping is applied. If right bound is None, no upper clipping is applied. Panics if right bound < left bound.",
            Self::Sign => "Returns the sign of a number (-1, 0, or 1).",
            Self::Signum => "Returns the signum of a number (-1, 0, or 1).",
            Self::Negate => "Returns the negative of a number.",
            Self::Negative => "Returns the negative of a number.",
            Self::Sqrt => "Calculates the square root of a number.",
            Self::Sin => "Calculates the sine of an angle in radians.",
            Self::Cos => "Calculates the cosine of an angle in radians.",
            Self::Tan => "Calculates the tangent of an angle in radians.",
            Self::Csc => "Calculates the cosecant of an angle in radians.",
            Self::Sec => "Calculates the secant of an angle in radians.",
            Self::Cot => "Calculates the cotangent of an angle in radians.",
            Self::Sinh => "Calculates the hyperbolic sine of an angle in radians.",
            Self::Cosh => "Calculates the hyperbolic cosine of an angle in radians.",
            Self::Tanh => "Calculates the hyperbolic tangent of an angle in radians.",
            Self::ArcSin => "Calculates the inverse sine (arc sine) of a number.",
            Self::ArcCos => "Calculates the inverse cosine (arc cosine) of a number.",
            Self::ArcTan => "Calculates the inverse tangent (arc tangent) of a number.",
            Self::ArcTan2 => {
                "Calculates the angle between the positive x-axis and the ray from (0,0) to (x,y)."
            }
            Self::Radians => "Converts an angle from degrees to radians.",
            Self::Degrees => "Converts an angle from radians to degrees.",
            Self::Log => "Calculates the first argument-based logarithm of the second argument log_x(y).",
            Self::Log2 => "Calculates the base-2 logarithm of a number.",
            Self::Log10 => "Calculates the base-10 logarithm of a number.",
            Self::Ln => "Calculates the natural logarithm of a number.",
            Self::Log1p => "Calculates the natural logarithm of a number plus one (ln(x + 1)).",
            Self::ArcTanh => "Calculates the inverse hyperbolic tangent of a number.",
            Self::ArcCosh => "Calculates the inverse hyperbolic cosine of a number.",
            Self::ArcSinh => "Calculates the inverse hyperbolic sine of a number.",
        };
        docstring.to_string()
    }

    fn arg_names(&self) -> &'static [&'static str] {
        match self {
            Self::Abs
            | Self::Ceil
            | Self::Floor
            | Self::Sign
            | Self::Signum
            | Self::Negate
            | Self::Negative
            | Self::Sqrt
            | Self::Sin
            | Self::Cos
            | Self::Tan
            | Self::Csc
            | Self::Sec
            | Self::Cot
            | Self::Sinh
            | Self::Cosh
            | Self::Tanh
            | Self::ArcSin
            | Self::ArcCos
            | Self::ArcTan
            | Self::Radians
            | Self::Degrees
            | Self::Log2
            | Self::Log10
            | Self::Ln
            | Self::Log1p
            | Self::Exp
            | Self::Expm1
            | Self::ArcTanh
            | Self::ArcCosh
            | Self::ArcSinh => &["input"],
            Self::Log => &["input", "base"],
            Self::Round => &["input", "precision"],
            Self::ArcTan2 => &["y", "x"],
            Self::Clip => &["input", "min", "max"],
        }
    }
}

fn to_expr(expr: &SQLNumericExpr, args: &[ExprRef]) -> SQLPlannerResult<ExprRef> {
    match expr {
        SQLNumericExpr::Abs => {
            ensure!(args.len() == 1, "abs takes exactly one argument");
            Ok(abs(args[0].clone()))
        }
        SQLNumericExpr::Ceil => {
            ensure!(args.len() == 1, "ceil takes exactly one argument");
            Ok(ceil(args[0].clone()))
        }
        SQLNumericExpr::Floor => {
            ensure!(args.len() == 1, "floor takes exactly one argument");
            Ok(floor(args[0].clone()))
        }
        SQLNumericExpr::Sign => {
            ensure!(args.len() == 1, "sign takes exactly one argument");
            Ok(sign(args[0].clone()))
        }
        SQLNumericExpr::Signum => {
            ensure!(args.len() == 1, "signum takes exactly one argument");
            Ok(sign(args[0].clone()))
        }
        SQLNumericExpr::Negate => {
            ensure!(args.len() == 1, "negate takes exactly one argument");
            Ok(negative(args[0].clone()))
        }
        SQLNumericExpr::Negative => {
            ensure!(args.len() == 1, "negative takes exactly one argument");
            Ok(negative(args[0].clone()))
        }
        SQLNumericExpr::Round => {
            ensure!(
                args.len() == 2 || args.len() == 1,
                "round takes one or two arguments"
            );
            let precision = match args.get(1).and_then(|arg| arg.as_literal()) {
                Some(LiteralValue::Int8(i)) => *i as i32,
                Some(LiteralValue::UInt8(u)) => *u as i32,
                Some(LiteralValue::Int16(i)) => *i as i32,
                Some(LiteralValue::UInt16(u)) => *u as i32,
                Some(LiteralValue::Int32(i)) => *i,
                Some(LiteralValue::UInt32(u)) => *u as i32,
                Some(LiteralValue::Int64(i)) => *i as i32,
                Some(LiteralValue::UInt64(u)) => *u as i32,
                None => 0,
                _ => invalid_operation_err!("round precision must be an integer"),
            };
            Ok(round(args[0].clone(), Some(precision)))
        }
        SQLNumericExpr::Clip => {
            ensure!(args.len() == 3, "clip takes exactly three arguments");
            Ok(clip(args[0].clone(), args[1].clone(), args[2].clone()))
        }
        SQLNumericExpr::Sqrt => {
            ensure!(args.len() == 1, "sqrt takes exactly one argument");
            Ok(sqrt(args[0].clone()))
        }
        SQLNumericExpr::Sin => {
            ensure!(args.len() == 1, "sin takes exactly one argument");
            Ok(sin(args[0].clone()))
        }
        SQLNumericExpr::Cos => {
            ensure!(args.len() == 1, "cos takes exactly one argument");
            Ok(cos(args[0].clone()))
        }
        SQLNumericExpr::Tan => {
            ensure!(args.len() == 1, "tan takes exactly one argument");
            Ok(tan(args[0].clone()))
        }
        SQLNumericExpr::Csc => {
            ensure!(args.len() == 1, "csc takes exactly one argument");
            Ok(csc(args[0].clone()))
        }
        SQLNumericExpr::Sec => {
            ensure!(args.len() == 1, "sec takes exactly one argument");
            Ok(sec(args[0].clone()))
        }
        SQLNumericExpr::Cot => {
            ensure!(args.len() == 1, "cot takes exactly one argument");
            Ok(cot(args[0].clone()))
        }
        SQLNumericExpr::Sinh => {
            ensure!(args.len() == 1, "sinh takes exactly one argument");
            Ok(sinh(args[0].clone()))
        }
        SQLNumericExpr::Cosh => {
            ensure!(args.len() == 1, "cosh takes exactly one argument");
            Ok(cosh(args[0].clone()))
        }
        SQLNumericExpr::Tanh => {
            ensure!(args.len() == 1, "tanh takes exactly one argument");
            Ok(tanh(args[0].clone()))
        }
        SQLNumericExpr::ArcSin => {
            ensure!(args.len() == 1, "asin takes exactly one argument");
            Ok(arcsin(args[0].clone()))
        }
        SQLNumericExpr::ArcCos => {
            ensure!(args.len() == 1, "acos takes exactly one argument");
            Ok(arccos(args[0].clone()))
        }
        SQLNumericExpr::ArcTan => {
            ensure!(args.len() == 1, "atan takes exactly one argument");
            Ok(arctan(args[0].clone()))
        }
        SQLNumericExpr::ArcTan2 => {
            ensure!(args.len() == 2, "atan2 takes exactly two arguments");
            Ok(atan2(args[0].clone(), args[1].clone()))
        }
        SQLNumericExpr::Degrees => {
            ensure!(args.len() == 1, "degrees takes exactly one argument");
            Ok(degrees(args[0].clone()))
        }
        SQLNumericExpr::Radians => {
            ensure!(args.len() == 1, "radians takes exactly one argument");
            Ok(radians(args[0].clone()))
        }
        SQLNumericExpr::Log2 => {
            ensure!(args.len() == 1, "log2 takes exactly one argument");
            Ok(log2(args[0].clone()))
        }
        SQLNumericExpr::Log10 => {
            ensure!(args.len() == 1, "log10 takes exactly one argument");
            Ok(log10(args[0].clone()))
        }
        SQLNumericExpr::Ln => {
            ensure!(args.len() == 1, "ln takes exactly one argument");
            Ok(ln(args[0].clone()))
        }
        SQLNumericExpr::Log1p => {
            ensure!(args.len() == 1, "log1p takes exactly one argument");
            Ok(log1p(args[0].clone()))
        }
        SQLNumericExpr::Log => {
            ensure!(args.len() == 2, "log takes exactly two arguments");
            let base = args[1]
                .as_literal()
                .and_then(|lit| match lit {
                    LiteralValue::Float64(f) => Some(*f),
                    LiteralValue::Int8(i) => Some(f64::from(*i)),
                    LiteralValue::UInt8(u) => Some(f64::from(*u)),
                    LiteralValue::Int16(i) => Some(f64::from(*i)),
                    LiteralValue::UInt16(u) => Some(f64::from(*u)),
                    LiteralValue::Int32(i) => Some(f64::from(*i)),
                    LiteralValue::UInt32(u) => Some(f64::from(*u)),
                    LiteralValue::Int64(i) => Some(*i as f64),
                    LiteralValue::UInt64(u) => Some(*u as f64),
                    _ => None,
                })
                .ok_or_else(|| PlannerError::InvalidOperation {
                    message: "log base must be a float or a number".to_string(),
                })?;

            Ok(log(args[0].clone(), base))
        }
        SQLNumericExpr::Exp => {
            ensure!(args.len() == 1, "exp takes exactly one argument");
            Ok(exp(args[0].clone()))
        }
        SQLNumericExpr::Expm1 => {
            ensure!(args.len() == 1, "expm1 takes exactly one argument");
            Ok(expm1(args[0].clone()))
        }
        SQLNumericExpr::ArcTanh => {
            ensure!(args.len() == 1, "atanh takes exactly one argument");
            Ok(arctanh(args[0].clone()))
        }
        SQLNumericExpr::ArcCosh => {
            ensure!(args.len() == 1, "acosh takes exactly one argument");
            Ok(arccosh(args[0].clone()))
        }
        SQLNumericExpr::ArcSinh => {
            ensure!(args.len() == 1, "asinh takes exactly one argument");
            Ok(arcsinh(args[0].clone()))
        }
    }
}
