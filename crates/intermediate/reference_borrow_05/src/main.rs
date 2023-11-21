fn main() {
    // 1.共享引用
    // String类型
    let x = String::from("888");
    // 引用：&x 表示 对x的共享引用
    let y = &x;
    println!("x = {}, y = {}", x, y);
}
// 运行结果
//  x = 888, y = 888