// 1 задание
pub fn get_mut_ref(cort: &mut (i32, i32), first_or_second: bool) -> &mut i32 {
    let (first, second) = cort;
    if first_or_second {
        first
    } else {
        second
    }
}

// 2 задание
pub fn get_slice_item_ref(slc: &mut [u32], n: usize) -> &mut u32 {
    &mut slc[n]
}

// 3 задание
pub fn get_slice_item_back_ref(slc: &mut [u32], n: usize) -> &mut u32 {
    &mut slc[slc.len() - n]
}

// 4 задание
pub fn get_two_slices(slc: &[u32], n: usize) -> (&[u32], &[u32]) {
    (&slc[..=n - 1], &slc[n..])
}

// 5 задание
pub fn get_four_slices(slc: &[u32]) -> (&[u32], &[u32], &[u32], &[u32]) {
    let step = slc.len() / 4;
    (
        &slc[..step],
        &slc[step..step * 2],
        &slc[step * 2..step * 3],
        &slc[step * 3..],
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cortege_test() {
        let mut data = (1, 2);
        let em1: i32 = 1;
        assert_eq!(get_mut_ref(&mut data, true), &em1);

        let em2: i32 = 2;
        assert_eq!(get_mut_ref(&mut data, false), &em2);

        let em3: i32 = 3;
        data.0 = 3;
        assert_eq!(get_mut_ref(&mut data, true), &em3);

        let em4: i32 = 4;
        data.1 = 4;
        assert_eq!(get_mut_ref(&mut data, false), &em4);
    }

    #[test]
    fn get_slice_item_ref_test_1() {
        let mut sl1 = [1u32, 2u32, 3u32, 4u32];
        assert_eq!(get_slice_item_ref(&mut sl1[..], 2), &mut 3u32);
    }

    #[test]
    fn get_slice_item_ref_test_2() {
        let mut sl1 = [1u32, 2u32, 3u32];
        let point_to_2 = get_slice_item_ref(&mut sl1[..], 1);
        *point_to_2 = 4u32;
        assert_eq!(&sl1[..], &[1u32, 4u32, 3u32][..]);
    }

    #[test]
    fn get_slice_item_back_ref_test_1() {
        let mut sl1 = [1u32, 2u32, 3u32, 4u32];
        assert_eq!(get_slice_item_back_ref(&mut sl1[..], 1), &mut 4u32);
    }

    #[test]
    fn get_slice_item_back_ref_test_2() {
        let mut sl1 = [1u32, 2u32, 3u32];
        let point_to_2 = get_slice_item_back_ref(&mut sl1[..], 1);
        *point_to_2 = 4u32;
        assert_eq!(&sl1[..], &[1u32, 2u32, 4u32][..]);
    }

    #[test]
    fn get_two_slices_test() {
        let sl = [1u32, 2u32, 3u32, 4u32, 5u32, 6u32];
        let (sl1, sl2) = get_two_slices(&sl[..], 3);
        assert_eq!(sl1, [1u32, 2u32, 3u32]);
        assert_eq!(sl2, [4u32, 5u32, 6u32]);
    }
    #[test]
    fn get_four_slices_test() {
        let sl = [1u32, 2u32, 3u32, 4u32, 5u32, 6u32, 7u32, 8u32, 9u32, 10u32];
        let (sl1, sl2, sl3, sl4) = get_four_slices(&sl[..]);
        assert_eq!(sl1, [1u32, 2u32]);
        assert_eq!(sl2, [3u32, 4u32]);
        assert_eq!(sl3, [5u32, 6u32]);
        assert_eq!(sl4, [7u32, 8u32, 9u32, 10u32]);
    }
}
