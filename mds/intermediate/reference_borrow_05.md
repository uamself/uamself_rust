# 引用和借用


## 引用（Reference）与借用（Borrow）

当我们在使用某些方法或者函数时，并不想把所有权转移到方法或者函数内部，Rust 则提供了一个新的概念——**借用**。借用的意思就是每次要执行某个方法或者函数，我把变量先借给你用一下，等你执行结束，你还要还回来。有句俗话不是说好借好还，再借不难吗。另外，变量在借用的时候，起码要显而易见吧，让编译器和读者都要明白，所以我们就使用 `&` 操作符。简单来讲，**“引用”表示在代码层面可见的语法，而“借用”则表示代码在这个过程中的整个行为流程**。


## 可变引用和共享引用

再来了解两个概念——共享引用和可变引用。

* 共享引用（Shared Reference）：直译为共享引用，**只能读取不能修改**。很多文章也称其为“不可变引用”。我们可以为一个值创建多个共享引用。如 `&T` 就叫做 T 的共享引用。

```rust
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
```

* 可变引用（Mutable Reference）：**可以读取且可以修改**。共享应用使用操作符 `&mut` 。如 `&mut T` 就叫做 T 的可变引用。当某个值存在可变引用时，你将不能拥有该值的任何其它引用。

```rust
fn main() {
    let mut a = String::from("rust");
    let b = &mut a;
    b.push_str(" is so easy!");
    println!("{}", b);
}
// 运行结果
// rust is so easy!
```

【重要】：**共享引用在编译时执行多读（Multiple Readers）的检查规则，可变引用在编译时执行单写（Single Writer）的检查规则。**

* 如果某个值存在共享引用，该值将被**锁定**，无法被修改，即便是该值的所有者也将禁止修改它。
* 如果某个值存在可变引用，则该引用将拥有**排他读写权**。在这个**可变引用的存续期间**，对应值的所有者都将无法使用。

## 解引用

既然存在引用，那必然需要相反的操作——解引用。在 Rust 中，引用是通过 `&` 来**显式**创建的，那么解引用也需要**显式**使用 `*` 操作符。

### 显示解引用

```rust
fn main() {
    let x = 888;
    let y = &x;
    println!(" x = {}, y = {}", x, *y);
}
// 运行结果
//  x = 888, y = 888
```

### 隐式解引用

当使用 `.` 操作符来调用一些方法时，Rust 会对齐进行**隐式解引用**。除此之外，必须使用 `&` 和 `*` 来追随引用。

```rust
fn main() {
    let mut a = String::from("rust");
    (&mut a).push_str(" is so easy!");
    println!("{}", a);
}
// 运行结果
// rust is so easy!
```


## println!() 和 dbg!()

```rust

fn main() {
    let mut m = String::from("rust");
    // 打印 m 和 n(没有解引用)
    println!("m = {}", m);
    // m 追加字符串(没有失去所有权)
    m.push_str(" is easy.");
    // 打印 m
    println!("{}", m);
}


// 运行结果
// m = rust
// rust is easy.
```

cargo expand 展开的代码如下

```rust

#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
fn main() {
    let mut m = String::from("rust");
    {
        ::std::io::_print(
            ::core::fmt::Arguments::new_v1(
                &["m = ", "\n"],
                // 传递了 &m
                &[::core::fmt::ArgumentV1::new_display(&m)],
            ),
        );
    };
    m.push_str(" is easy.");
    {
        ::std::io::_print(
            ::core::fmt::Arguments::new_v1(
                &["", "\n"],
                // 传递了 &m
                &[::core::fmt::ArgumentV1::new_display(&m)],
            ),
        );
    };

```

通过展开后的代码，可以看到 `println!()` 传递的是引用的值，所以它不会失去所有权了。但是如果你使用 `dbg!` 来打印变量的时候将会失去所有权，因为 `dbg!` 并没有获取参数的引用值，大家可以自行测试。

## 引用作为方法/函数参数

当我们在传递非 `Copy` 标记的类型的参数时，往往会失去所有权。如果我们在执行某个函数或者方法时，不想失去所有权，我们可以定义引用参数传给函数/方法。

```rust
fn main() {
    let mut a = String::from("hello");
    print_string(&a);

    // 向 a 中追加字符串
    a.push_str(" rust!");
    print_string(&a);

    println!("println! 打印--- {}", a);
}

// 打印一个字符串
fn print_string(s: &String) {
    println!("print_string 打印--- {}", s);
}
```

我们创建了一个 `print_string` 函数，接收的参数是引用类型的。然后声明字符串变量 `a`, 将 `a` 的引用传入该函数。即使在后面再次使用 `a` ，它也不会失去所有权。**当引用离开作用域时，将会归还所有权**。

另外，值得注意的一点是 **当进行解引用时，将会重新获得所有权**。

```rust
fn main() {
    let aa = String::from("xxx");
    operate_string(&aa);
}

fn operate_string(aa: &String) {
    // 下面的代码是错误的
    // let x = *aa;
}
```

“当一个函数/方法接收引用时，我们在函数/方法内对参数解引用。” 假设这种做法是可行的，`aa` 的所有权被转移给 `x`， 那么函数/方法执行结束后，`x`销毁失去所有权，外部的 `aa` 将指向未知的位置，`aa` 将会变成**野指针**。所以此做法是被禁止的，编译将会发生错误。


## 引用作为函数/方法返回值

```rust
fn main() {
    let mut b = String::from("hello world");
    let c = get_self(&mut b);
    println!("c = {}", c);
}

fn get_self(s: &String) -> &String {
    return s;
}

// 运行结果
// c = hello world
```
