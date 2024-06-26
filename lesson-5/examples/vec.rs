use lesson_5::utils::vec::*;

fn main() {
    println!(" -- fn default_vec3()");
    println!("{:#?}", default_vec3());

    println!(" -- fn vec3_vector_sum()");
    let a = [1; 3] as Vec3;
    let b = [1; 3] as Vec3;
    println!("{:#?}", vec3_vector_sum(a, b));

    println!(" -- fn vec3_scalar_sum()");
    let a = [1; 3] as Vec3;
    let b = [1; 3] as Vec3;
    println!("{:#?}", vec3_scalar_sum(a, b));
}
