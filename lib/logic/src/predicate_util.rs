use config::ECompareFunction;

pub fn run_compare_function<T: Eq + Ord>(func: ECompareFunction, actual: T, condition: T) -> bool {
    use ECompareFunction::*;

    match func {
        Greater => actual > condition,
        GreaterEqual => actual >= condition,
        Equal => actual == condition,
        Less => actual < condition,
        LessEqual => actual <= condition,
        NotEqual => actual != condition,
    }
}
