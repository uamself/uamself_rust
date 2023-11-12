fn main(){
    let a: i32 = 5;
    println!("{} 转为浮点数 {}", a, a as f64);

    // 溢出
    //let b: u128 = u128::MAX;

    let b= 543;
    println!("{} 转为浮点数 {}", b, b as f32);
}