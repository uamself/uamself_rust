fn say_hello() {
    println!("hello!")
}

fn main() {
    // 函数赋值，声明类型
    let say_hello_ptr: fn() = say_hello;
    say_hello_ptr();

    // 函数赋值，类型推断
    let other_say_hello_ptr = say_hello;
    other_say_hello_ptr();

    // 原函数调用
    say_hello();

    // ==================================
    // 函数声明别名
    type Calc = fn(i32, i32) -> i32;
    // 操作运算(别名)
    fn operation_alias(calc: Calc, a: i32, b: i32) -> i32 {
        return calc(a, b);
    }
    // 操作运算
    fn operation(calc: fn(i32, i32) -> i32, a: i32, b: i32) -> i32 {
        return calc(a, b);
    }
    // 加法
    fn add(a: i32, b: i32) -> i32 {
        return a + b;
    }
    // 乘法
    fn mul(a: i32, b: i32) -> i32 {
        return a * b;
    }

    let a = 5;
    let b = 3;
    let add_result = operation_alias(add, a, b);
    let mul_result = operation(mul, a, b);

    dbg!(add_result);
    dbg!(mul_result);
}
