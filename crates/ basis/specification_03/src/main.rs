fn main() {
    for arg in std::env::args()
    {
        println!("{}", arg);
    }





    print!("print");  //行缓冲
    println!("println");


    eprint!("我是 eprint!");
    eprintln!("我是 eprintln!");

    dbg!("end");


    // 返回值，默认返回是0
    std::process::exit(0);
}