pub type UnsignedCounter = usize;

pub fn default_unsigned_counter() -> UnsignedCounter {
    0
}

pub fn next_unsigned(counter: UnsignedCounter) -> UnsignedCounter {
    counter + 1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_default_unsigned_counter() {
        let tmp1 = 0 as UnsignedCounter;
        assert_eq!(default_unsigned_counter(), tmp1);
    }

    #[test]
    fn it_next_unsigned() {
        let tmp1 = 1 as UnsignedCounter;
        let tmp2 = 2 as UnsignedCounter;
        assert_eq!(next_unsigned(tmp1), tmp2);
    }
}
