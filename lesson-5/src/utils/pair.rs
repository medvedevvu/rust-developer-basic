pub type Pair = (i32, i32);

pub fn default_pair() -> Pair {
    (0, 0)
}

pub fn pair_vector_sum(a: Pair, b: Pair) -> Pair {
    (a.0 + b.0, a.1 + b.1)
}

pub fn pair_scalar_sum(a: Pair, b: Pair) -> i32 {
    a.0 + a.1 + b.0 + b.1
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_default_pair() {
        let tmp1: Pair = (0i32, 0i32);
        assert_eq!(default_pair(), tmp1);
    }

    #[test]
    fn it_pair_scalar_sum() {
        let a = (0, 3) as Pair;
        let b = (0, 3) as Pair;
        assert_eq!(pair_scalar_sum(a, b), 6i32);
    }

    #[test]
    fn it_pair_vector_sum() {
        let a = (0, 3) as Pair;
        let b = (0, 3) as Pair;
        let c = (0, 6) as Pair;
        assert_eq!(pair_vector_sum(a, b), c);
    }
}
