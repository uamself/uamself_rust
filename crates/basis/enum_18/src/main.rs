#[derive(Debug)]
enum Week {
    Mon,
    Tue,
    Wed,
    Thu = 300,
    Fri,
    Sat,
    Sun,
}
fn main() {
    // 打印枚举类型
    println!("{:?}", Week::Wed); //Wed
    // 打印枚举的值
    println!("{}", Week::Wed as i32); //2
    println!("{}", Week::Mon as i32); //0
    // 由于Thu 赋值维 300, 则后面的值依次+1
    println!("{}", Week::Fri as i32); //301
}