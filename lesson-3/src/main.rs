mod tests;

fn main() {
    println!("lesson-3 ");
}

fn double_int32(p: i32) -> (i32, bool) {
    let (a,b) = p.overflowing_mul(2);
    if b {
        (0i32, true)
    } else {
        (a, false)
    }
}

fn double_int64(p: i32) -> (i64, bool) {
    let tmp:i64 = p as i64;
    let (a,b) = tmp.overflowing_mul(2);
    if b {
        (0i64, true)
    } else {
        (a, false)
    }
}

fn double_float32(p: f32) -> f32 {
    p * 2.0
}

fn double_float64(p: f32) -> f64 {
    (p  as f64 ) * 2.0
}

fn int_plus_float_to_float(p1: i32, p2: f32) -> f64 {
    (p1 as f64) + (p2 as f64)
}

fn int_plus_float_to_int(p1: i32, p2: f32) -> (i64, bool) {
    let (a, b) = (p1 as i64).overflowing_add(p2 as i64);
    if b {
        (0i64, true)
    } else {
        (a , false)
    }
}

fn tuple_sum(p: (i32, i32)) -> (i32, bool) {
    let (a, b) = p.0.overflowing_add(p.1);
    if b {
        (0i32, true)
    } else {
        (a , false)
    }
}

fn array_sum(p: &[i32; 3]) -> (i32, bool) {
    let  mut summ :i32 = 0;
    let  mut b:bool;
    for item in p.iter() {
        (summ, b) = summ.overflowing_add(*item);
        if b {
            return  (0i32, b);
        }
    }
    (summ, false)
}