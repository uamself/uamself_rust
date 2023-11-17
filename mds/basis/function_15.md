# 函数

在写代码的时候，不可避免会产生很多重复的代码。这个时候可以将这些重复代码定义成一个函数。

## 函数定义

- 定义函数使用fn关键字，其后面紧跟函数名称，函数名称通常以蛇形命名法(snake\_case)。
- 函数如果有参数，必须为其指定类型。
- 如果函数存在返回值，也应该为其指定类型。
- 当然，函数的参数和返回值都不是必须的。函数的声明可以在`main`函数后面。下面是一个完整的函数表现形式。

> fn function\_name(parameter: T) -> N {
>
> }

下面通过一个示例来了解下函数。编写一个计算两个整数加法的函数。

```rust
fn main() {
    let a = 5;
    let b = 3;
    let c = add(a, b);
    println!("a + b = {}", c);
}

/// 计算两个数的加法
fn add(a: i32, b: i32) -> i32 {
    return a + b;
}
```

函数参数如果需要可变操作时，可以添加mut关键字，如：`mut a: i32`。函数如果存在多个参数，则需要通过逗号分隔。当函数需要返回值时，必须明确返回值类型，且在函数签名后面添加`->`符号，其后紧跟函数返回值类型。

PS：Rust中所有的函数都存在返回值，如果没有显示的明确函数的返回值，也会隐式的返回一个单元值()。

示例代码如下：

```rust
fn main() {
    let d = empty_no_return();
    dbg!(d);    // d = ()
}

/// 空函数，无返回值
fn empty_no_return() {

}
```


## 函数指针

高阶函数就是指以函数为参数或返回值的函数，是函数式编程语言的基础特性。将函数作为一种类型，既可以被调用，也可以作为参数，又可以作为返回值。实现这些操作的基础就是函数指针，函数指针使用fn()来指定。

顾名思义，函数指针就表示是指向函数的指针，它的值就是函数的地址啦。下面看一个例子：

```rust
fn say_hello() {
    println!("hello!")
}

fn main() {
    // 函数赋值，声明类型
    let say_hello_ptr: fn() = say_hello;
    say_hello_ptr();

    // 函数赋值，类型推断
    let other_say_hello_ptr= say_hello;
    other_say_hello_ptr();

    // 原函数调用
    say_hello()
}
```

我来解释下上面的代码吧。首先创建了一个say\_hello()函数。然后在main()函数中声明了两个函数变量分别是say\_hello\_ptr和other\_say\_hello\_ptr。然后分别输出。当然函数赋值时，支持类型推断。另外，虽然函数可以赋值给其它变量，但是原函数仍然可以被调用，且函数可以同时赋值给多个变量。

PS：函数赋值时，在等号右边只有函数名，不带有`()`。

## type定义别名

在Rust中，我们可以使用**type**关键字来为Rust的原始类型或者自定义类型定义别名。

示例代码如下

```rust
    type Int = i32;
    type Float = f32;
    type Double = f64;
    type Char = char;

    let a: Int = 3;
    let b: Float = 4.5;
    let c: Double = 134.6753453424234231;
    let d: Char = '我';

    dbg!(a);
    dbg!(b);
    dbg!(c);
    dbg!(d);
```

上面的代码可以看到，咱们把**i32**类型定义别名为**Int**类型，**f32**类型定义别名为**Float**类型，**f64**类型定义别名为**Double**类型，**char**类型定义别名为**Char**类型。

## 函数作为参数

当Rust的函数作为参数时，建议使用**type**关键字为函数指针类型定义别名。其目的是为了提升代码的可读性。下面我通过一个例子讲解将函数作为参数的例子。

```rust
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
    fn main() {
        let a = 5;
        let b = 3;
        let add_result = operation_alias(add, a, b);
        let mul_result = operation(mul, a, b);

        dbg!(add_result);
        dbg!(mul_result);
    }
```


首先，使用**type**关键字为函数指针类型**fn(i32,i32)->i32**起一个别名为**Calc**。然后又定义了一个函数**operation\_alias**，它有三个参数，其中第一个参数为**Calc**类型，第二个和第三个参数是**i32**类型。紧接着又定义了一个函数**operation**，它也有三个参数，与**operation\_alias**一样。不同的是，**operation**的第一个参数没有使用别名，而是使用了原始的函数指针类型，在可读性上可能比较差。最后按照**fn(i32,i32)->i32**形式，分别定义了**add**和**mul**函数来计算加法和乘法。

**main**函数中，分别调用**operation\_alias**和**operation**方法，计算**a**和**b**的值。

## 函数作为返回值

当Rust的函数作为返回值时，建议使用**type**关键字为函数指针类型定义别名。其目的是为了提升代码的可读性。下面我通过一个例子讲解将函数作为返回值的例子。

```rust
// 函数声明别名
type Calc = fn(i32, i32) -> i32;

// 根据字符串获取函数(别名)
fn get_operation_alias(s: &str) -> Calc {
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
```

代码解释：

首先，使用**type**关键字为函数指针类型**fn(i32,i32)->i32**起一个别名为**Calc**。

然后又定义了一个函数**get\_operation\_alias**，它只有一个参数，根据参数返回指定的函数。紧接着又定义了一个函数**get\_operation**，它也只有一个参数，与**get\_operation\_alias**一样。不同的是，**get\_operation**的返回值使用了原始的函数指针类型，在可读性上可能比较差。最后按照**fn(i32,i32)->i32**形式，分别定义了**add**和**mul**函数来计算加法和乘法。

**main**函数中，分别调用**get\_operation\_alias**和**get\_operation**方法，计算**a**和**b**的值。


## 函数小结

本节介绍了Rust高阶函数的两种形式——函数作为参数和函数作为返回值。当函数作为参数或者作为返回值时，建议使用**type**类型定义别名。


## 闭包

### 闭包定义

闭包（Closure）通常是指词法闭包，是一个持有外部环境变量的函数。

**外部环境**是指闭包定义时所在的词法作用域。

外部环境变量，在函数式编程范式中也被称为**自由变量**，是指并不是在闭包内定义的变量。

**将自由变量和自身绑定的函数**就是闭包。闭包通常有参数列表（由两条竖线包裹）和表达式组成，语法形式如下。

**| parameters |** expression

示例代码如下：

```rust
    let add = |a, b| a + b;
    // let add: fn(i32, i32) -> i32 = |a, b| a + b;
    let result = add(1, 2);
    // let result = add(1.2, 3.4);
    dbg!(result); // 3
```

解释下上面的代码，

第一行代码声明了一个函数类型，其使用闭包语法创建的。

我们没有为其指定类型，它会根据上下文推断为`fn(i32, i32) -> i32`类型，相当于第二行注释。

再看第4行注释的代码，如果将它打开则会报错，因为前面已经将其推断为`fn(i32, i32) -> i32`类型，不再接收其它类型。

从上面的代码来看，又感觉闭包是简化版的函数。


### 从函数到闭包

下面用一个例子来看下从函数到闭包经过了哪些变化。

首先我们先定义一个函数

```rust
fn add(a:i32, b:i32) -> i32 {a + b}
```

转成闭包，使用**let**关键字声明，括号变成管道符号。最后添加分号结束。

```rust
let add = |a:i32, b:i32|->i32 {a + b};
```

简化，去除参数类型的声明。闭包并不像函数那样严格要求注明函数类型和返回值类型，去除括号。

```rust
let add = |a, b| {a + b};
```

继续简化，

```rust
let add = |a, b| a + b;
```


### 捕获变量

在闭包中还存在一个特点，它可以捕获和使用其被定义时所在的作用域中的变量。

示例代码如下：

```rust
    let k = 8;

    let add_var = |a| a + k;

    let result = add_var(10);
    dbg!(result);   //18
```


### 闭包小结

总结下闭包的特点。

* 闭包不严格要求注明参数和返回值类型
* 不注明类型的闭包，若多次调用但传递不同类型，则会发生错误
* 闭包可以捕获和使用其所在作用域中的变量
* 闭包通常适用于相对较小的场景上下文
* 闭包性能要快于函数指针
