use lesson_5::utils::signed::*;

fn main() {
    println!(" -- fn default_signed_counter");
    println!("{:?}", default_signed_counter());

    println!(" -- fn next_signed");
    let tmp1 = 1 as SignedCounter;
    println!("{:?}", next_signed(tmp1));

    println!(" -- fn prev_signed");
    let tmp1 = 2 as SignedCounter;
    println!("{:?}", prev_signed(tmp1));
}
