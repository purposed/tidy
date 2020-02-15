use super::virtual_document::{VirtualDocument, VirtualField};
use crate::{Condition, FieldValue};

macro_rules! condition_tests {
    ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (cond_str, fields, expected) = $value;
                let v_doc = VirtualDocument::new(fields);
                let cond: Condition<VirtualField> = Condition::parse(cond_str).unwrap();
                assert_eq!(cond.eval(&v_doc).unwrap(), expected);
            }
        )*
    }
}

condition_tests! {
    eq_str_1: ("@stringa == \"HELLO\"", vec![(VirtualField::StringA, FieldValue::from("HELLO"))], true),
    eq_str_2: ("@stringa == \"SKRT\"", vec![(VirtualField::StringA, FieldValue::from("HELLO"))], false),

    gt_str: ("@stringa > \"aaa\"", vec![(VirtualField::StringA, FieldValue::from("aab"))], true),
    gt_str_2: ("@stringa > \"aaa\"", vec![(VirtualField::StringA, FieldValue::from("aaa"))], false),
    gt_str_3: ("@stringa > \"b\"", vec![(VirtualField::StringA, FieldValue::from("aaa"))], false),

    lt_str_1: ("@stringa < \"aab\"", vec![(VirtualField::StringA, FieldValue::from("aaa"))], true),
    lt_str_2: ("@stringa < \"aaa\"", vec![(VirtualField::StringA, FieldValue::from("aaa"))], false),
    lt_str_3: ("@stringa < \"0\"", vec![(VirtualField::StringA, FieldValue::from("aaa"))], false),

    geq_str: ("@stringa >= \"aaa\"", vec![(VirtualField::StringA, FieldValue::from("aaa"))], true),
    geq_str_2: ("@stringa > \"aaa\"", vec![(VirtualField::StringA, FieldValue::from("aab"))], true),
    geq_str_3: ("@stringa > \"b\"", vec![(VirtualField::StringA, FieldValue::from("aaa"))], false),

    leq_str_1: ("@stringa <= \"aab\"", vec![(VirtualField::StringA, FieldValue::from("aaa"))], true),
    leq_str_2: ("@stringa <= \"aaa\"", vec![(VirtualField::StringA, FieldValue::from("aaa"))], true),
    leq_str_3: ("@stringa <= \"0\"", vec![(VirtualField::StringA, FieldValue::from("aaa"))], false),

    eq_int_1: ("@inta == 18", vec![(VirtualField::IntA, FieldValue::from(18))], true),
    eq_int_2: ("@inta == 1", vec![(VirtualField::IntA, FieldValue::from(18))], false),

    gt_int_1: ("@inta > 1", vec![(VirtualField::IntA, FieldValue::from(18))], true),
    gt_int_2: ("@inta > 18", vec![(VirtualField::IntA, FieldValue::from(18))], false),
    gt_int_3: ("@inta > 19", vec![(VirtualField::IntA, FieldValue::from(18))], false),

    lt_int_1: ("@inta < 100", vec![(VirtualField::IntA, FieldValue::from(18))], true),
    lt_int_2: ("@inta < 18", vec![(VirtualField::IntA, FieldValue::from(18))], false),
    lt_int_3: ("@inta < 3", vec![(VirtualField::IntA, FieldValue::from(18))], false),

    geq_int_1: ("@inta >= 1", vec![(VirtualField::IntA, FieldValue::from(18))], true),
    geq_int_2: ("@inta >= 18", vec![(VirtualField::IntA, FieldValue::from(18))], true),
    geq_int_3: ("@inta >= 19", vec![(VirtualField::IntA, FieldValue::from(18))], false),

    leq_int_1: ("@inta <= 100", vec![(VirtualField::IntA, FieldValue::from(18))], true),
    leq_int_2: ("@inta <= 18", vec![(VirtualField::IntA, FieldValue::from(18))], true),
    leq_int_3: ("@inta <= 3", vec![(VirtualField::IntA, FieldValue::from(18))], false),
}
