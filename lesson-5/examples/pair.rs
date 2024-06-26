use lesson_5::utils::pair::*;

fn main() {
    println!(" -- type Pair");
    println!("{:?}", default_pair());

    println!(" -- fn pair_scalar_sum");
    let a = (0, 3) as Pair;
    let b = (0, 3) as Pair;
    println!("{:?}", pair_scalar_sum(a, b));

    println!(" -- fn pair_vector_sum");
    let a = (0, 3) as Pair;
    let b = (0, 3) as Pair;
    println!("{:?}", pair_vector_sum(a, b));
}
