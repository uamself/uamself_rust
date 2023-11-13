fn main(){
    //数组越界
    let f = [1, 3, 5, 7, 9];
    println!("f数组的长度是 {}", f.len());
    // 下面的代码会发生数据越界
    //println!("{}", f[5]);



    //元组
    let _tup: (i32, f64, bool) = (20, 3.14, false);

    // 一个元素的元组
    let tup_single: (i32, ) = (5, );
    // 下面两种方式不是一个元组
    // 他们等价于 let tup_wrong = 5;
    // let tup_wrong = (5);
    // let tup_wrong: (i32) = (5);
    dbg!(tup_single);

    // 空元组
    let empty_tup = ();
    dbg!(empty_tup);
}