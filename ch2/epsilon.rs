fn main() {
    let result: f32 = 0.1+0.1;
    let desired: f32 = 0.2;

    let absolute_difference = (desired - result).abs();

    assert!(absolute_difference <= f32::EPSILON);
    println!("{}",absolute_difference);

    let result64: f64 = 0.1+0.1+0.1;
    let desired64: f64 = 0.3;

    let absolute_difference64 = (desired64-result64).abs();
    println!("{}",absolute_difference64<=f64::EPSILON);
    println!("diff: {}, epsilon: {}", absolute_difference64, f64::EPSILON);

    // Operations that produce mathematically undefined results, such as taking the square root of a negative number (-42.0.sqrt()), present particular problems. Floating- point types include “not a number” values (represented in Rust syntax as NAN values) to handle these cases.
    // NAN values poison other numbers. Almost all operations interacting with NAN return NAN. Another thing to be mindful of is that, by definition, NAN values are never equal. This small program will always crash:
    let x = (-42.0_f32).sqrt();
    // assert_eq!(x,x);

    // defensive
    assert!(x.is_nan());

    let y: f32 = 1.0/0.0;
    assert!(!y.is_finite());
}
