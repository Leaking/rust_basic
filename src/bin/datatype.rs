fn main() {
    let a : u8 = 255;
    let b = a.overflowing_add(20);
    println!("{:?}", b);  // 19
    let b = a.wrapping_add(20);
    println!("{}", b);  // 19

    // NaN
    let x = (-42.0_f32).sqrt();
    if x.is_nan() {
        println!("未定义的数学行为")
    }

    // Range
    for i in 1..5 {
        println!("{}", i);
    } 
    for i in 1..=5 {
        println!("{}", i);
    }
    // 下面如果用双引号会编译异常
    for ch in 'a'..='q' {
        println!("{}", ch);
    }
}