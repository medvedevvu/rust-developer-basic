mod tests;

fn main() {
    println!("lesson-3 ");
}

fn double_int32(p: i32) -> i32 {
    p * 2
}

fn double_int64(p: i32) -> i64 {
    (p * 2) as i64
}

fn double_float32(p: f32) -> f32 {
    p * 2.0
}

fn double_float64(p: f32) -> f64 {
    (p * 2.0) as f64
}

fn int_plus_float_to_float(p1: i32, p2: f32) -> f64 {
    ((p1 as f32) + p2) as f64
}

fn int_plus_float_to_int(p1: i32, p2: f32) -> i64 {
    ((p2 as i32) + p1) as i64
}

fn tuple_sum(p: (i32, i32)) -> i32 {
    p.0 + p.1
}

fn array_sum(p: &[i32; 3]) -> i32 {
    p.iter().sum()
}