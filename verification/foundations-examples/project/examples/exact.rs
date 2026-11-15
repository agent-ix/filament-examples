use filament_exact::{
    evaluate_rational_arithmetic as rational_op, Integer, Meter, Outcome, Rational,
    RationalArithmetic, ScalarLimits,
};

fn limits(work_units: u64) -> ScalarLimits {
    ScalarLimits {
        integer_bits: u64::MAX,
        decimal_digits: u64::MAX,
        scale_expansion: u64::MAX,
        text_input_bytes: u64::MAX,
        text_scalars: u64::MAX,
        normalized_scalars: u64::MAX,
        unit_edges: u64::MAX,
        value_occurrences: u64::MAX,
        work_units,
        result_units: u64::MAX,
    }
}

fn r(n: i64, d: i64) -> Rational {
    Rational::new(Integer::from(n), Integer::from(d)).unwrap()
}

fn main() {
    println!("f64: 0.1 + 0.2 = {}", 0.1_f64 + 0.2_f64);

    let mut meter = Meter::new(limits(u64::MAX));
    let sum = rational_op(RationalArithmetic::Add(&r(1, 10), &r(2, 10)), None, &mut meter);
    match &sum {
        Outcome::Completed(v) => println!("exact: 1/10 + 2/10 = {v}"),
        other => println!("unexpected: {other:?}"),
    }
    println!("equal fractions are equal values: {}", r(2, 4) == r(1, 2));

    let divided = rational_op(RationalArithmetic::Divide(&r(1, 2), &r(0, 1)), None, &mut meter);
    println!("1/2 divided by 0: {divided:?}");

    let mut small = Meter::new(limits(0));
    let starved = rational_op(RationalArithmetic::Add(&r(1, 10), &r(2, 10)), None, &mut small);
    println!("with no work units: {starved:?}");
}
