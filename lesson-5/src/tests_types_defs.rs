#[cfg(test)]
mod tests {
    use crate::utils::default::defs;
    use crate::utils::types::my_types;

    #[test]
    fn it_default_signed_counter() {
        let tmp1 = 0 as my_types::SignedCounter;
        let tmp2 = 0 as my_types::SignedCounter;
        assert_eq!(defs::default_signed_counter(tmp1), tmp2);
    }

    #[test]
    fn it_default_unsigned_counter() {
        let tmp1 = 0 as my_types::UnsignedCounter;
        assert_eq!(defs::default_unsigned_counter(), tmp1);
    }

    #[test]
    fn it_default_vec3() {
        let tmp1: my_types::Vec3 = [0; my_types::VEC3_LEN];
        assert_eq!(defs::default_vec3(), tmp1);
    }

    #[test]
    fn it_default_pair() {
        let tmp1: my_types::Pair = (0i32, 0i32);
        assert_eq!(defs::default_pair(), tmp1);
    }
}
