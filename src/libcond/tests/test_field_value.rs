use std::time;

use crate::FieldValue;

macro_rules! field_value_tests {
    ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (a, b, op, expected) = $value;
                match op {
                    "=" => {
                        assert_eq!(a == String::from(b), expected);
                    },
                    ">" => {
                        assert_eq!(a > String::from(b), expected);
                    },
                    ">=" => {
                        assert_eq!(a >= String::from(b), expected);
                    }
                    _ => unimplemented!()
                }
            }
        )*
    }
}

field_value_tests! {
    // String fieldvalue tests
    eq_string_string: (FieldValue::from("asdf"), "asdf", "=", true),
    neq_string_string: (FieldValue::from("asdf"), "bing", "=", false),
    neq_string_int: (FieldValue::from("asdf"), "18", "=", false),
    gt_string_string: (FieldValue::from("bsdf"), "asdf", ">", true),
    gt_string_string_2: (FieldValue::from("asdf"), "bsdf", ">", false),
    geq_string_string: (FieldValue::from("bsdf"), "asdf", ">=", true),
    geq_string_string_2: (FieldValue::from("asdf"), "bsdf", ">=", false),
    gt_string_int: (FieldValue::from("asdf"), "18", ">", true),  // Ensure comparison is ASCII

    // Int fieldvalue tests
    eq_int_int: (FieldValue::from(18), "18", "=", true),
    neq_int_int: (FieldValue::from(18), "19", "=", false),
    neq_int_string: (FieldValue::from(18), "asdf", "=", false),
    neq_int_float: (FieldValue::from(18), "18.18", "=", false),
    gt_int_int_1: (FieldValue::from(18), "17", ">", true),
    gt_int_int_2: (FieldValue::from(18), "18", ">", false),
    geq_int_int: (FieldValue::from(18), "18", ">=", true),
    geq_int_int_2: (FieldValue::from(19), "18", ">=", true),
    geq_int_int_3: (FieldValue::from(17), "18", ">=", false),

    // Duration fieldvalue tests
    eq_dur_dur_1: (FieldValue::from(time::Duration::from_secs(60)), "1 minute", "=", true),
    eq_dur_dur_2: (FieldValue::from(time::Duration::from_secs(60)), "1 min", "=", true),
    eq_dur_dur_3: (FieldValue::from(time::Duration::from_secs(60)), "1m", "=", true),
    eq_dur_dur_4: (FieldValue::from(time::Duration::from_secs(90)), "30 seconds 1m", "=", true),
    eq_dur_dur_5: (FieldValue::from(time::Duration::from_secs(3600)), "1 hour", "=", true),
    neq_dur_dur_1: (FieldValue::from(time::Duration::from_secs(60)), "61 seconds", "=", false),
    neq_dur_dur_2: (FieldValue::from(time::Duration::from_secs(61)), "1 minute", "=", false),
    eq_dur_int: (FieldValue::from(time::Duration::from_secs(10)), "10", "=", true), // Validate sec by default
    neq_dur_int: (FieldValue::from(time::Duration::from_millis(10)), "10", "=", false),
    neq_dur_str: (FieldValue::from(time::Duration::from_secs(10)), "ten seconds", "=", false),
    gt_dur_dur_1: (FieldValue::from(time::Duration::from_secs(10)), "10 seconds", ">", false),
    gt_dur_dur_2: (FieldValue::from(time::Duration::from_secs(10)), "11 seconds", ">", false),
    gt_dur_dur_3: (FieldValue::from(time::Duration::from_secs(11)), "10 seconds", ">", true),
    gt_dur_dur_4: (FieldValue::from(time::Duration::from_secs(3600)), "20 seconds 59 minutes", ">", true),
    geq_dur_dur_1: (FieldValue::from(time::Duration::from_secs(10)), "10 seconds", ">=", true),
    gt_dur_int_1: (FieldValue::from(time::Duration::from_secs(10)), "9", ">", true),
    geq_dur_int_1: (FieldValue::from(time::Duration::from_secs(10)), "10", ">=", true),
}
