pub const VEC3_LEN: usize = 3;
pub type Vec3 = [i32; VEC3_LEN];
pub fn default_vec3() -> Vec3 {
    [0; 3]
}

pub fn vec3_vector_sum(a: Vec3, b: Vec3) -> Vec3 {
    let mut c = default_vec3();
    for i in 0..3 {
        c[i] = a[i] + b[i];
    }
    c
}

pub fn vec3_scalar_sum(a: Vec3, b: Vec3) -> i32 {
    let mut c = 0;
    for i in 0..VEC3_LEN {
        c += a[i] + b[i];
    }
    c
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_default_vec3() {
        let tmp1: Vec3 = [0; VEC3_LEN];
        assert_eq!(default_vec3(), tmp1);
    }

    #[test]
    fn it_vec3_vector_sum() {
        let a = [1; 3] as Vec3;
        let b = [1; 3] as Vec3;
        let c = [2; 3] as Vec3;
        assert_eq!(vec3_vector_sum(a, b), c);
    }

    #[test]
    fn it_vec3_scalar_sum() {
        let a = [1; 3] as Vec3;
        let b = [1; 3] as Vec3;
        assert_eq!(vec3_scalar_sum(a, b), 6i32);
    }
}
