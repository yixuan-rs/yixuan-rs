use std::{collections::HashMap, sync::LazyLock};

use config::{DynamicInteger, NapArithmeticOp, NapExpr};
use tracing::error;

use super::chessboard::EventGraphContext;

pub fn get_dynamic_integer(config: &DynamicInteger, context: &EventGraphContext<'_>) -> i32 {
    match config {
        DynamicInteger::Static(value) => *value,
        DynamicInteger::Dynamic(expression) => execute_expression(expression, context) as i32,
    }
}

fn execute_expression(exp: &NapExpr, context: &EventGraphContext<'_>) -> f64 {
    match exp {
        NapExpr::Call(name, args) => exp_call(name, args, context) as f64,
        NapExpr::Number(num) => *num,
        NapExpr::Identifier(_) => 0.0,
        NapExpr::UnaryOp(op, value) => exp_unary_op(op, execute_expression(value, context)),
        NapExpr::BinOp(op, lhs, rhs) => exp_bin_op(
            op,
            execute_expression(lhs, context),
            execute_expression(rhs, context),
        ),
    }
}

fn exp_unary_op(op: &NapArithmeticOp, value: f64) -> f64 {
    match op {
        NapArithmeticOp::Add => value,
        NapArithmeticOp::Sub => -value,
        _ => unreachable!(),
    }
}

fn exp_bin_op(op: &NapArithmeticOp, lhs: f64, rhs: f64) -> f64 {
    match op {
        NapArithmeticOp::Add => lhs + rhs,
        NapArithmeticOp::Sub => lhs - rhs,
        NapArithmeticOp::Mul => lhs * rhs,
        NapArithmeticOp::Div => lhs / rhs,
    }
}

type DynamicFunction = fn(&[NapExpr], &EventGraphContext<'_>) -> i32;

fn exp_call(name: &str, args: &[NapExpr], context: &EventGraphContext<'_>) -> i32 {
    static CALL_TABLE: LazyLock<HashMap<&'static str, DynamicFunction>> =
        LazyLock::new(|| HashMap::from([("specials", specials as _)]));

    if let Some(func) = CALL_TABLE.get(name) {
        func(args, context)
    } else {
        error!("exp_call: function '{name}' doesn't exist");
        0
    }
}

fn specials(args: &[NapExpr], context: &EventGraphContext<'_>) -> i32 {
    if args.len() != 1 {
        error!("specials: unexpected number of arguments ({})", args.len());
        return 0;
    }

    let arg_0 = args.first().unwrap();
    if let NapExpr::Identifier(ident) = arg_0 {
        context
            .event_specials
            .get(ident)
            .copied()
            .unwrap_or_else(|| {
                error!("specials: no key {ident}");
                0
            })
    } else {
        error!("specials: unexpected argument type, expected: Identifier, got: {arg_0:?}");
        0
    }
}
