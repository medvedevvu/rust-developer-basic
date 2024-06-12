mod tests {
    //use crate::{double_int32, double_int64,double_float32,double_float64};
    use crate::{*};

    #[test]
    fn double_int32_test() {
        assert_eq!(double_int32(2i32), 4i32, "ok")
    }

    #[test]
    fn double_int64_test() {
        assert_eq!(double_int64(2i32), 4i64, "ok")
    }

    #[test]
    fn double_float32_test() {
        assert_eq!(double_float32(2f32), 4f32, "ok")
    }

    #[test]
    fn double_float64_test() {
        assert_eq!(double_float64(2f32), 4f64, "ok")
    }

    #[test]
    fn int_plus_float_to_float_test() {
        assert_eq!(int_plus_float_to_float(2i32, 2f32), 4f64, "ok")
    }

    #[test]
    fn int_plus_float_to_int_test() {
        assert_eq!(int_plus_float_to_int(2i32, 2f32), 4i64, "ok")
    }

    #[test]
    fn tuple_sum_test() {
        assert_eq!(tuple_sum((2i32, 2i32)), 4i32, "ok")
    }

    #[test]
    fn array_sum_test() {
        assert_eq!(array_sum(&[2i32, 2i32, 3i32]), 7i32, "ok")
    }
}