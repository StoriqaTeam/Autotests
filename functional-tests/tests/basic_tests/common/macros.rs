#[macro_export]
macro_rules! assert_eq_f32 {
    ($left:expr, $right:expr, $pow:expr) => {{
        match (&$left, &$right, &$pow) {
            (left_val, right_val, pow_val) => {
                let precision = (10f32).powi(*pow_val);

                let left_rounded = (left_val * precision).round() / precision;
                let right_rounded = (right_val * precision).round() / precision;

                assert_eq!(left_rounded, right_rounded);
            }
        }
    }};

    ($left:expr, $right:expr) => {{
        assert_eq_f32!($left, $right, 2);
    }};
}

#[macro_export]
macro_rules! assert_eq_f64 {
    ($left:expr, $right:expr, $pow:expr) => {{
        match (&$left, &$right, &$pow) {
            (left_val, right_val, pow_val) => {
                let precision = (10f64).powi(*pow_val);

                let left_rounded = (left_val * precision).round() / precision;
                let right_rounded = (right_val * precision).round() / precision;

                assert_eq!(left_rounded, right_rounded);
            }
        }
    }};

    ($left:expr, $right:expr) => {{
        assert_eq_f64!($left, $right, 2);
    }};
}
