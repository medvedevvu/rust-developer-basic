mod tests {
    use crate::{*};

    #[test]
    fn double_int32_test() {
        assert_eq!(double_int32(2i32), (4i32, false), "ok")
    }

    #[test]
    fn double_int32_overflowing_test() {
        assert_eq!(double_int32(i32::MAX), (0i32, true), "ok")
    }

    #[test]
    fn double_int64_test() {
        assert_eq!(double_int64(2i32), (4i64, false), "ok")
    }

    #[test]
    fn double_int64_overflowing_test() {
        assert_eq!(double_int64(2_000_000_000i32), (4_000_000_000i64, false), "ok")
    }

    #[test]
    fn double_float32_test() {
        assert_eq!(double_float32(2f32), 4f32, "ok")
    }

    #[test]
    fn double_float64_test() {
        assert_eq!(double_float64(2f32), 4f64, "ok");
    }

    #[test]
    fn int_plus_float_to_float_test() {
        assert_eq!(int_plus_float_to_float(2i32, 2f32), (4f64, false), "ok")
    }

    #[test]
    fn int_plus_float_to_float_overflowing_test() {
        assert_eq!(int_plus_float_to_float(i32::MAX, 2f32), (0f64, true), "ok");
        assert_eq!(int_plus_float_to_float(2i32, f32::MAX), (0f64, true), "ok");
    }


    #[test]
    fn int_plus_float_to_int_test() {
        assert_eq!(int_plus_float_to_int(2i32, 2f32), (4i64, false), "ok")
    }

    #[test]
    fn int_plus_float_to_int_overflowing_test() {
        assert_eq!(int_plus_float_to_int(2i32, f32::MAX), (0, true), "ok");
        assert_eq!(int_plus_float_to_int(i32::MAX, 2f32), (0, true), "ok");
    }

    #[test]
    fn tuple_sum_test() {
        assert_eq!(tuple_sum((2i32, 2i32)), (4i32, false), "ok")
    }

    #[test]
    fn tuple_sum_overflowing_test() {
        assert_eq!(tuple_sum((2i32, i32::MAX)), (0, true), "ok")
    }

    #[test]
    fn array_sum_test() {
        assert_eq!(array_sum(&[2i32, 2i32, 7i32]), (11i32, false), "ok")
    }
    #[test]
    fn array_sum_overflowing_test() {
        assert_eq!(array_sum(&[2i32, 2i32, i32::MAX]), (0, true), "ok")
    }

}