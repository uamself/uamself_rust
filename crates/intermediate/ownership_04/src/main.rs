fn main() {
    // =================== 案例1 =======================
    // 基本数据类型
    let a = 3;
    let b = a;


    println!("a = {}, b = {}", a, b);


    // String类型
    let m = String::from("rust");
    // 让渡所有权
    let n = m;


    // m 变为无法访问，下面的代码将会报错
    // println!("m = {}, n = {}", m, n);


    println!("n = {}", n);

    // 变量遮蔽
    let m = 5;
    println!("m = {}", m);

    // ================== 案例2 =======================
    let a = vec![String::from("hello"), String::from("study"), String::from("rust")];
    let _b = a;
    // 下面的语句会报错
    // let c = a;

    // =================== 案例3 ======================
    let name = String::from("ZhangSan");
    print_name(name);// name的所有权已经转移到函数内部
    println!("{}", name);

    // =================== 案例4 ======================
    // get_name 返回值的所有权转移至 name
    let name = get_name();
    // 打印name
    println!("{}", name);
}

fn print_name(name: String) {
    println!("My name is {}", name);
}

fn get_name() -> String {
    // 1. 函数内创建 name 变量
    let name = String::from("LiSi");
    return String::from("My name is ") + name.as_str();
} // 2. 作用域结束，内存释放，name释放，将返回值的所有权转移至调用者
