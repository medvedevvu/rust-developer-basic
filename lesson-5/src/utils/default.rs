pub mod defs {
    use crate::utils::types::my_types;
    use crate::utils::types::my_types::SignedCounter;

    pub fn default_signed_counter(_i: SignedCounter) -> my_types::SignedCounter {
        0
    }

    pub fn default_unsigned_counter() -> my_types::UnsignedCounter {
        0
    }

    pub fn default_vec3() -> my_types::Vec3 {
        [0; 3]
    }

    pub fn default_pair() -> my_types::Pair {
        (0, 0)
    }
}
