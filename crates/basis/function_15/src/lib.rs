// 函数声明别名
type Calc1 = fn(i32, i32) -> i32;

// 根据字符串获取函数(别名)
fn get_operation_alias(s: &str) -> Calc1 {
    if s == "add" {
        add
    } else {
        mul
    }
}

// 根据字符串获取函数
fn get_operation(s: &str) -> fn(i32, i32) -> i32 {
    if s == "add" {
        add
    } else {
        mul
    }
}

// 加法
fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

// 乘法
fn mul(a: i32, b: i32) -> i32 {
    return a * b;
}

fn main() {
    let a = 4;
    let b = 6;
    let add_result = get_operation_alias("add")(a, b);
    let mul_result = get_operation("mul")(a, b);

    dbg!(add_result);
    dbg!(mul_result);
}