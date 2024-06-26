pub type SignedCounter = isize;
pub fn default_signed_counter() -> SignedCounter {
    0
}
pub fn next_signed(counter: SignedCounter) -> SignedCounter {
    counter + 1
}
pub fn prev_signed(counter: SignedCounter) -> SignedCounter {
    counter - 1
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_default_signed_counter() {
        let tmp1 = 0 as SignedCounter;
        assert_eq!(default_signed_counter(), tmp1);
    }

    #[test]
    fn it_next_signed() {
        let tmp1 = 1 as SignedCounter;
        let tmp2 = 2 as SignedCounter;
        assert_eq!(next_signed(tmp1), tmp2);
    }

    #[test]
    fn it_prev_signed() {
        let tmp1 = 2 as SignedCounter;
        let tmp2 = 1 as SignedCounter;
        assert_eq!(prev_signed(tmp1), tmp2);
    }
}
