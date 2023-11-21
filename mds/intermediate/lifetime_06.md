# 生命周期

## 简单案例

lifetime 这也是 Rust 中的重点。简单了解下 Rust 的生命期的概念以及简单使用。


我们先看一个小例子：

```rust
fn main() {
    let a;
    {
        let b = 1;
        a = &b;
    }
    println!("{}", *a);
}
```

这段代码是编译不通过的，来看下编译器给出的错误。

简单分析下：变量 `a` 是一个 `&i32` 引用类型，我们在内部代码块中初始化 `a`，但是当内部代码块执行结束后，变量 `b` 离开作用域被释放了，但是 `a` 没有被释放，这时 `a` 就会变成 `悬垂指针`，当然这在 Rust 中是绝对不允许的。

理论上来讲，其实所有的变量都存在生存期，**变量的生命期一定是包含引用的生存期**。

## 生命周期的使用

### 标注生命周期

只有引用类型才需要标注 lifetime。因此，以`&i32` 为例，标注生命期后变为 `&'a i32` ，在 `&` 后添加 `'a`，通常叫做`生命期 a`。a 可以被更换，其命名规则参考变量的命名规则。

* `&'a i32`  标注生命期 a 的共享引用
* `&'a mut i32`  标注生命期 a 的可变引用

### 函数/方法签名中的生命期标注

编译器通常会推断生命期，当然我们也可以标注生命期。通常我们写函数/方法时是下面的写法。

```rust
fn test(name: &str) -> &str {
    println!("{}", name);
    return name;
}
```

其实，这里是存在生命期标注的，如果编译器可以自动推断生命期时，则无需标注。上面的函数添加生命期标注后如下所示：

```rust
fn test_life<'_a>(name: &'_a str) -> &'_a str {
    println!("{}", name);
    return name;
}
```

在函数名后面，添加`<'a>`，如同泛型，在标注前先声明。然后再对每个参数或者返回值标注。

## 为什么存在生命期？

**生命期仅用于编译器的检查。并不会更改原有生命期的长短**。举个简单的例子，下面的代码是传入两个字符串，返回最长的那个字符串。

```rust
fn main() {
    let x = String::from("xxx");
    let y = "yyyy";

    let z = longest(x, y);

    println!("{}", z);
}

fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

如果我们直接编译，会提示错误。

```rust
error[E0106]: missing lifetime specifier
  --> src\main.rs:31:33
   |
31 | fn longest(x: &str, y: &str) -> &str {
   |               ----     ----     ^ expected named lifetime parameter
   |
   = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `x` or `y`
help: consider introducing a named lifetime parameter
   |
31 | fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
   |           ++++     ++          ++          ++

For more information about this error, try `rustc --explain E0106`.
error: could not compile `lifetime` due to previous error
```

错误提示告诉我们，缺少生命期标识符。在当编译时期，rust 并不知道我们返回的是 `x` 还是 `y`，因此不能确定返回的字符串的生命期。这个函数主体中， `if` 块返回的是 `x` 的引用，而 `else` 块返回的是 `y` 的引用。所以我们需要标注生命期，来告诉编译器.

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

标注传入参数的生命期都是 `a`，返回值的生命期也是 `a`，所以无论返回 `x` 还是 `y`，都是生命期 `a` 的 `&str`。因此：**生命期仅用于编译器的检查。**

## 总结

本章仅仅是简单了解下 lifetime，lifetime 是 Rust 中用来保证引用在使用时是有效的。

生命期并不会改变在方法和函数中返回引用时，如果返回的引用不指向其中一个参数，那么它必须指向在这个函数中创建的一个值，然而这将会产生悬垂引用。
