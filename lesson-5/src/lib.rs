mod tests_types_defs;
mod utils;

#[cfg(test)]
mod tests {
    use super::utils::functions::my_funcs;
    use super::utils::types::my_types;

    #[test]
    fn it_next_signed() {
        let tmp1 = 1 as my_types::SignedCounter;
        let tmp2 = 2 as my_types::SignedCounter;
        assert_eq!(my_funcs::next_signed(tmp1), tmp2);
    }

    #[test]
    fn it_next_unsigned() {
        let tmp1 = 1 as my_types::UnsignedCounter;
        let tmp2 = 2 as my_types::UnsignedCounter;
        assert_eq!(my_funcs::next_unsigned(tmp1), tmp2);
    }

    #[test]
    fn it_prev_signed() {
        let tmp1 = 2 as my_types::SignedCounter;
        let tmp2 = 1 as my_types::SignedCounter;
        assert_eq!(my_funcs::prev_signed(tmp1), tmp2);
    }

    #[test]
    fn it_vec3_vector_sum() {
        let a = [1; 3] as my_types::Vec3;
        let b = [1; 3] as my_types::Vec3;
        let c = [2; 3] as my_types::Vec3;
        assert_eq!(my_funcs::vec3_vector_sum(a, b), c);
    }

    #[test]
    fn it_pair_vector_sum() {
        let a = (0, 3) as my_types::Pair;
        let b = (0, 3) as my_types::Pair;
        let c = (0, 6) as my_types::Pair;
        assert_eq!(my_funcs::pair_vector_sum(a, b), c);
    }

    #[test]
    fn it_vec3_scalar_sum() {
        let a = [1; 3] as my_types::Vec3;
        let b = [1; 3] as my_types::Vec3;
        assert_eq!(my_funcs::vec3_scalar_sum(a, b), 6i32);
    }

    #[test]
    fn it_pair_scalar_sum() {
        let a = (0, 3) as my_types::Pair;
        let b = (0, 3) as my_types::Pair;
        assert_eq!(my_funcs::pair_scalar_sum(a, b), 6i32);
    }
}
