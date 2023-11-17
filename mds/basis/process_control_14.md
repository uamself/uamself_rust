# 流程控制

## 条件判断

`if`表达式在任何语言里面都很常见，其形式如下：

> if condition1 {
>
> *block1*
>
> } else if condition2 {
>
> * block2*
>
> } else {
>
> *block3*
>
> }

所有的*condition*都是一个bool型的表达式。Rust并不像C语言那样，非0值表示`true`，0值表示`false`。*condition*的花括号并不是必须的。

下面通过一道题来练习下：

求一个数对应的星期。将一个数对7取余，如果是0，则选择的是周日，以此类推，如果是6则选择的是周6。

示例代码如下：

```rust
    // 求一个数对应的星期。将一个数对7取余，如果是0，则选择的是周日，以此类推，如果是6则选择的是周6
    let week: u32 = 1;

    if week % 7 == 0 {
        println!("您选择的是周日！")
    } else if week % 7 == 1 {
        println!("您选择的是周一！")
    } else if week % 7 == 2 {
        println!("您选择的是周二！")
    } else if week % 7 == 3 {
        println!("您选择的是周三！")
    } else if week % 7 == 4 {
        println!("您选择的是周四！")
    } else if week % 7 == 5 {
        println!("您选择的是周五！")
    } else {
        println!("您选择的是周六！")
    }
```

## 循环

循环，顾名思义，表示重复执行某一段代码。

在Rust中，共有3中循环形式，分别是loop，while，for..in..循环。

控制循环的语句有两个：break和continue。

### break和continue

break和continue都用来循环控制。

break用于直接退出循环;

continue则表示跳出当前轮次的循环，不在执行continue后的代码，但是它仍然会再次执行调节判断，决定是否执行下次循环。

### loop循环

重复执行，永远不会结束，如果没有退出条件，那就是死循环了。使用起来比较简单。

其签名形式为：

> loop {
>
> // 要执行循环的代码
>
> }

示例代码如下：

```rust
    let mut count = 0;
    loop {
        count += 1;

        // 计数器为5则跳出循环
        if count == 5 {
            break;
        }
    }

    println!("count = {}", count);
```

### while循环

while后面跟一个表达式，如果表达式的值为`true`，则会执行循环。

```rust
    let mut count = 0;
    while count < 5 {
        count += 1;
    }

    println!("while循环 -> count = {}", count);
```

### for..in..循环

重复执行指定次数的循环。

```rust
    let mut count = 0;
    for i in 1..=5 {
        count = i
    }

    println!("for..in..循环 -> count = {}", count);
```

## match模式匹配

在Rust中，并没有其它语言的`switch`语句，取而代之的是`match`模式匹配。

`match`用来判断当前值是否匹配一系列模式中的某一个。

模式可以由字面量，变量，通配符来构成。与其它语言中的`switch`的类似，每一个模式即是一个分支。

PS：在Rust中，`match`模式匹配要求穷举所有的可能性，否则会导致程序报错。可以在一系列模式最后添加`_`通配符，表示没有指定的其它所有模式（相当于Java中的`default`）。

示例代码如下（将第一个示例转为match实现）：

```rust
    // 求一个数对应的星期。将一个数对7取余，如果是0，则选择的是周日，以此类推，如果是6则选择的是周6
    let week: u32 = 6;
    match week % 7 {
        0 => { println!("您选择的是周日！") }
        1 => { println!("您选择的是周一！") }
        2 => { println!("您选择的是周二！") }
        3 => { println!("您选择的是周三！") }
        4 => { println!("您选择的是周四！") }
        5 => { println!("您选择的是周五！") }
        6 => { println!("您选择的是周六！") }
        _ => { println!("未知！") }
    }
```




### if let 和 while let

在某些场合使用`match`可能会显得有些繁琐，因此在Rust中也提供了`if let`和`while let`在某些场景下替换`match`。

#### if let

*if let*的形式如下：

> if let pattern = expr {
>
> *block1*
>
> } else {
>
> *block2*
>
> }

使用`if let`进行匹配时要注意，`pattern`和`expr`直接是单`=`号。还是拿本章开始的那个例子来解释，假设我只是想对周五这一天做匹配妖气，而对其它没有任何要求，我们就可以使用`if let`进行改造。

```rust
let week:u32 = 5;
if let 5 = week % 7{

    println!("你选择的是周五");
}else{
    println!("你选择的是其他");
}
```



#### while let

*while let*的形式如下：

> while let pattern = expr {
>
> * block*
>
> }

现在有个要求需要逆向依次输出向量中的值。

使用loop和match实现：

```rust
    let mut vec = vec![1, 3, 5, 7, 9];
    loop {
        match vec.pop() {
            None => {
                break;
            }
            Some(value) => {
                print!("{} ", value);
            }
        }
    }
```


改造为while let:

```rust
    let mut vec = vec![1, 3, 5, 7, 9];
    while let Some(value) = vec.pop() {
        print!("{} ", value);
    }
```
