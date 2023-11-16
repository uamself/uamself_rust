# 枚举

## 枚举的定义

枚举类型是一种自定义的数据类型，使用enum关键字 + 自定义的命名 + 枚举值来定义。

我们通过使用 **枚举名 :: 枚举值** 来访问枚举的值。

使用场景：当一个参数可能存在多个取值范围时，我们定义成枚举类型来控制变量的取值范围，从而防止输入非法值。

示例代码如下：

```rust
// 枚举示例代码
// 声明一个星期的枚举类型，只有7天，周一到周日。
enum Week {
    Mon,
    Tue,
    Wed,
    Thu,
    Fri,
    Sat,
    Sun,
}
```

## C 式枚举

上面的示例代码，就是Rust中最简单的枚举类型了，同 **C 语言** 一模一样。在内存中，C 式枚举的值存储为整数，默认从0开始，后面的值依次 +1。如果想更改某个枚举的值，需要告诉它用什么整数。示例代码如下：

```rust
// 为了可以格式化打印枚举值，需要加入下面这行代码，具体有关 trait 的知识将在后面介绍。
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
    println!("{:?}", Week::Wed);
    // 打印枚举的值
    println!("{}", Week::Wed as i32);
    println!("{}", Week::Mon as i32);
    // 由于Thu 赋值维 300, 则后面的值依次+1
    println!("{}", Week::Fri as i32);
}
// 输出结果:
// Wed
// 2
// 0
// 301
```


## 包含数据的枚举

在 Rust 中，枚举的值存在 3 种变体，分别对应前两篇文章介绍的 3 种结构体。

**没有数据的变体对应类基元结构体，元组变体的写法和作用同元组结构体一样，结构体变体则存在花括号和命名字段。**

**在一个枚举里可以同时存在3 种变体。**

```rust
#[derive(Debug)]
enum Color {
    // 定义色值，参数分别表示16进制颜色代码，R，G，B
    White(String, u8, u8, u8),
    Red,
    Black { code: String, r: u8, g: u8, b: u8 },
}



fn main() {
    let white = Color::White(String::from("#FFFFFF"), 255, 255, 255);
    let red = Color::Red;
    let black = Color::Black { 
                    code: String::from("#000000"), 
                    r: 0, 
                    g: 0, 
                    b: 0
                 };
    println!("{:?}", white);
    println!("{:?}", red);
    println!("{:?}", black);   

    // 输出结果:
    // White("#FFFFFF", 255, 255, 255)
    // Red
    // Black { code: "#000000", r: 0, g: 0, b: 0 }

}
```

## 枚举内存布局(todo)
