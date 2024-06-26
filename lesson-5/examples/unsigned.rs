use lesson_5::utils::unsigned::*;

fn main() {
    println!(" -- fn default_unsigned_counter");
    println!("{}", default_unsigned_counter());

    println!(" -- fn next_unsigned");
    let tmp1 = 1 as UnsignedCounter;
    println!("{}", next_unsigned(tmp1));
}
