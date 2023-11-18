# trait

## trait定义

**trait** 是 Rust 中唯一的接口抽象方式，类似于 Java 和 C# 的 *interface*。**trait 可以定义多个抽象的方法（行为）来用于多个结构体共享，使得不同的结构体可以拥有相同的方法（行为）**。


## trait的实现

trait 是一种任何类型都可以选择支持或者不支持的特性，通常代表某一种行为或者能力。

> 示例：所有的动物基本上都会”说话“或者有一些其它类似的行为，但是它们的叫声又有区别，我们可以为把他们抽象出一个 trait 叫 **Animal**。

示例代码如下：

```rust
/// 动物 trait
trait Animal {
    // 动物的叫声
    fn make_sound(&self);
}
```


上面的代码就是定义了一个动物的 trait，定义 trait 很简单，我们只需要给它命名并且列出 trait 方法的类型签名即可。

那我们如何使用这个 trait 呢？实现这个 trait 的语法如下：

```rust
impl [TraitName] for [Type]
```

在 Rust 中，我们可以使用泛型为任何类型甚至是 **str** , **i32**,  **bool** 等内置类型添加扩展方法。

这里声明的 **Animal** 属于动物，我再定义几个动物类型的结构体。通常 trait 都是与 struct 一起使用。完整代码如下：

```rust
/// 动物 trait
trait Animal {
    // 动物的叫声
    fn make_sound(&self);
}

/// 狗
struct Dog {
    name: String,
}

/// 鱼
struct Cat {
    name: String,
}

/// 为 Dog 类型实现 Animal trait
impl Animal for Dog {

    // 打印狗的叫声
    fn make_sound(&self) {
        println!("汪汪~");
    }
}

/// 为 Cat 类型实现 Animal trait
impl Animal for Cat {
  
    // 打印猫的叫声
    fn make_sound(&self) {
        println!("喵喵~");
    }
}

fn main() {
    // 创建dog
    let dog = Dog { name: String::from("二哈") };
    // 创建cat
    let cat = Cat { name: String::from("美短") };

    // 运行方法
    dog.make_sound();
    cat.make_sound();
}

    // 运行结果:
    // 汪汪~
    // 喵喵~
```

上面代码注释都很明确，我就不多做解释了。使用 trait 的方法时候有两个注意事项：

* 定义 trait 方法的时候不要忘记第一个参数必须是 **&self**。有没有 **&self** 的区别，上一篇文章我已经解释了。
* 使用 trait 方法的时候，**trait 本身必须在当前作用域中，否则 trait 所有的方法都是隐藏的**。


## trait作为参数

**trait** 作为参数时一般使用 **impl trait** 的语法来表示参数类型。先上代码：

```rust
/// impl trait 作为参数
fn speak(animal: impl Animal) {
    animal.make_sound()
}

/// 为已有类型实现 trait （示例）
impl Animal for i32 {
    fn make_sound(&self) {
        println!("i32");
    }
}

fn main() {
    speak(dog);
    speak(cat);
  
    let a = 5;
    speak(a);
  
    // 运行结果：
    // 汪汪~
    // 喵喵~
    // i32
}
```

*speak* 函数表示可以接收实现了 **Animal trait** 的任何实例，包括已经存在的类型，上面代码以 **i32** 为例。

如果你想让 *speak* 函数接收任意类型，那你可以让那个类型实现   **Animal trait**。

问题又来了，如果我想在某一个函数里接收同时实现了多个 **trait** 的参数应该如何操作呢？继续往下看：

```rust
/// 测试多trait
trait Test {
    fn test(&self);
}

/// 为Dog实现Test
impl Test for Dog {
    fn test(&self) {
        println!("这是一个Test方法");
    }
}

/// 打印同时实现 Test 和 Animal Trait的方法
fn printMulti(p: impl Test + Animal) {
    p.make_sound();
    p.test();
}

fn main() {
    // 多 trait同时实现
    printMulti(dog);
  
    // 下面的代码错误
    // printMulti(cat);
  
    // 运行结果:
    // 汪汪~
	// 这是一个Test方法
}
```

上面的代码又定义了一个 *Test* 的 **trait**，我们也为 *Dog*  实现了这个 *Test*。最后定义了一个 *printMulti* 函数打印内容。我们不能为这个函数传参 *cat*，因为它没有实现 *Test*。


## 泛型与trait

### 定义泛型trait


泛型它又来了，泛型和 **trait** 又会发生什么样的火花呢？先看下面的代码：

```rust
// 定义一个泛型trait
trait MyPrint<T> {
    // 输出传递的参数
    fn print(&self, x: T) -> T;
}


// 测试结构体
struct Test;


// 为Test实现MyPrint
impl MyPrint<i32> for Test {
    // 返回值
    fn print(&self, x: i32) -> i32 {
        return x;
    }
}


fn main() {
    let test = Test;
    // 直接输出结果
    println!("{}", test.print(3));
}
```

**MyPrint** 是一个泛型 trait，在 trait 名称后面添加 \<T> 符号则标明该 trait 是一个泛型的 trait。我们声明了一个结构体 Test，为其实现了 MyPrint<i32> 的trait。最后，我们就可以调用 `print` 直接输输出 i32 类型的数据了 。像类似 MyPrint\<i32> 、 MyPrint\<String> 、 MyPrint\<u8>等等这都不是同一类型的。


### 泛型约束

我们还可以使用 trait 对泛型进行约束，trait 约束与泛型参数声明在一起，这种方式一般用于复杂的开发场景。示例代码如下：

```rust
fn trait_demo<T: TraitOne + TraitTwo + TraitOther>(param: T){
    // code...
}
```

这里的 **T** 表示该参数**同时**实现了 **TraitOne, TraitTwo, TraitOther** 三个 trait。

如果存在多个泛型，我们也可以对多个泛型进行约束。示例如下：

```rust
fn multi_fun<T: TraitOne, E: TraitTwo + TraitOther>(param1: T, param2: E) {
    // code...
}
```

在遇到更加复杂的开发场景时，可能存在泛型存在多个 trait 约束的场景，为了避免可读性差和“头重脚轻”的问题，我们可以通过 **where** 关键字来进行优化。可以将约束写在函数签名后面——>  **where** + 约束条件。

以 **multi\_fun** 为例，转为 **where** 关键字的写法如下：

```rust
fn multi_fun_where<T,E>(param1: T, param2: E) where T: TraitOne, E: TraitTwo + TraitOther {
    // code...
}
```


## Supertraits


Rust 中没有 “继承”的概念，但是我们可以定义一个 trait 为另一个 trait 的超集。在某些文章中又叫做"子 trait" 或者"继承"。

```rust
// Supertraits
trait Animal {
    fn speak(&self);
}

trait Dog: Animal {
    // 狗还会跳
    fn jump(&self);
}

struct SmallDog;

// 为 SmallDog 实现 Dog 的同时，也必须实现 Animal
impl Animal for SmallDog {
    fn speak(&self) {


    }
}

impl Dog for SmallDog {
    fn jump(&self) {
    }
}
```

我们再为 **SmallDog** 实现 **Dog** 的同时，也要为其实现 **Animal**。实现的顺序是无所谓的，但必须要实现，否则会产生错误。其实这点类似于 Java 中的接口继承。


## 常用Derive的trait

在介绍常用 **trait** 前，我们先了解下 *Derive*， *Derive* 我们常翻译为“派生”。在 Rust 中，有 `#[derive]` 这样一个属性，通过这个属性，编译器能够提供某些 trait 的基本实现。当然如果在实际开发中需要更复杂的行为，这些 trait 也可以手动实现。

## 

### Debug

源码：

```rust
#[doc(alias = "{:?}")]
#[rustc_diagnostic_item = "Debug"]
#[rustc_trivial_field_reads]
pub trait Debug {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result;
}
```

Debug 是可以与 `derive` 属性一起使用的。官方提供了默认 `{:?}` 的实现。在结构体的文章中，我们曾用过这个属性。相对于结构体等一些类型的实例，我们是无法直接通过 `println("{:?}")`或者 `dbg()` 打印它们的。这时我们可以为其类型加上 `#[derive(Debug)]` 属性。

示例代码：

```
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    // 1、Debug
    let rec1 = Rectangle {
        width: 3,
        height: 5,
    };
    println!("{:?}", rec1);

    let rec2 = Rectangle {
        width: 6,
        height: 4,
    };
    dbg!(rec2);
}

```

如果我们在打印时，不加 `#[derive(Debug)]` 属性。则编译器会提示错误`the trait Debug is not implemented for Rectangle`，并且建议添加 `#[derive(Debug)]`。


### Eq 和 PartialEq

`Eq` 是 **Equal** 的缩写，即这两个是判断是否相等的 `trait`。他们同样可以与 `derive` 连用。

源码：

```rust
pub trait PartialEq<Rhs: ?Sized = Self> {
    fn eq(&self, other: &Rhs) -> bool;

    fn ne(&self, other: &Rhs) -> bool {
        !self.eq(other)
    }
}

pub trait Eq: PartialEq<Self> {
    fn assert_receiver_is_total_eq(&self) {}
}
```

如果我们想比较某个类型的两个值 a 和 b是否相等，那么我们就必须为类型实现 `PartialEq Trait`。可以看到 `Eq` 和 `PartialEq` 是父子关系。要实现 `Eq` 必须在实现 `PartialEq`的基础上实现。 `Eq` 和 `PartialEq`来自于抽象代数中的等价关系和局部等价关系。两个都满足了对称性（即 `a == b` 可以推出 `b == a`）和传递性（即`a == b` 和 `b == c` 可以推出 `a == c`）。**`Eq` 还需要满足自反性（即 a == a）**。在 Rust 中，浮点数类型两个 `NAN` 是不相等的。Rust 只为其实现了`PartialEq`。下面是官方源码（来自`cmp.rs`）:

```rust
partial_eq_impl! {
        bool char usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64
}
eq_impl! { () bool char usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
```

我们自己实现 `PartialEq` 来判断两个三角形是否相似。

```rust
// 三角形
struct Triangle {
    a: f64,
    b: f64,
    c: f64,
}

// 自己实现 PartialEq
// 判断三角形相似
impl PartialEq for Triangle {
    fn eq(&self, other: &Self) -> bool {
        let x = self.a / other.a;
        let y = self.b / other.b;
        let z = self.c / other.c;
        return x == y && x == z && y == z;
    }

    fn ne(&self, other: &Self) -> bool {
        return !eq(self, other);
    }
}

fn main() {
    // 相似三角形
    let tri1 = Triangle { a: 3.0, b: 4.0, c: 5.0 };
    let tri2 = Triangle { a: 6.0, b: 8.0, c: 10.0 };
    println!("tri1 和 tri2 相似 ? {}", tri1 == tri2);
  
    // 运行结果
    // tri1 和 tri2 相似 ? true
}
```

当然也可以通过 derive 来默认实现结构体的比较。

```
#[derive(PartialEq)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rec1 = Rectangle {
        width: 3,
        height: 5,
    };

    let rec2 = Rectangle {
        width: 3,
        height: 5,
    };
    println!("rec1 == rec2 ? {}", rec1 == rec2);
	// 运行结果
    // rec1 == rec2 ? true
}
```


### Ord 和 PartialOrd


`Ord` 是 **Order** 的缩写，即这两个是判断是全序关系的 `trait`。他们同样可以与 `derive` 连用。全序关系是指集合内的任何一对元素都是相互可以比较的。`Ord` 和 `PartialOrd` 的使用方法同  `Eq` 和 `PartialEq` 类似。

但是有两个依赖要求，`PartialOrd` 必须要求类型实现  `PartialEq` 和 `Ord` 必须要求类型实现 `PartialOrd`  和  `Eq` 。

`Ord` 中还提供了 max 和 min 方法，更加方便进行比较。

源码：

```rust
pub trait PartialOrd<Rhs: ?Sized = Self>: PartialEq<Rhs> {

    fn partial_cmp(&self, other: &Rhs) -> Option<Ordering>;


    fn lt(&self, other: &Rhs) -> bool {
        matches!(self.partial_cmp(other), Some(Less))
    }

    fn le(&self, other: &Rhs) -> bool {
        !matches!(self.partial_cmp(other), None | Some(Greater))
    }

    fn gt(&self, other: &Rhs) -> bool {
        matches!(self.partial_cmp(other), Some(Greater))
    }

    fn ge(&self, other: &Rhs) -> bool {
        matches!(self.partial_cmp(other), Some(Greater | Equal))
    }
}

pub trait Ord: Eq + PartialOrd<Self> {

    fn cmp(&self, other: &Self) -> Ordering;

    fn max(self, other: Self) -> Self
    where
        Self: Sized,
    {
        max_by(self, other, Ord::cmp)
    }


    fn min(self, other: Self) -> Self
    where
        Self: Sized,
    {
        min_by(self, other, Ord::cmp)
    }

    fn clamp(self, min: Self, max: Self) -> Self
    where
        Self: Sized,
    {
        assert!(min <= max);
        if self < min {
            min
        } else if self > max {
            max
        } else {
            self
        }
    }
}
```

通过`derive` 为结构体类型添加可以比较的属性。示例代码：

```rust
#[derive(PartialEq)]
#[derive(PartialOrd)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rec1 = Rectangle {
        width: 1,
        height: 5,
    };

    let rec2 = Rectangle {
        width: 3,
        height: 5,
    };
    println!("rec1 > rec2 ? {}", rec1 > rec2);
}
```

自定义 `PartialOrd` 的代码我就不贴了。 自定义 `PartialOrd` 时，需要实现 `partial_cmp` 方法。示例：有一种物体比较大小取决于价格，它的价格越低就越大。

```
// 货物
#[derive(PartialEq)]
struct Good {
    price: f64,
}

impl PartialOrd for Good {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // 要求价格低的反而大
        return if self.price < other.price {
            Some(Ordering::Greater)
        } else if self.price > other.price {
            Some(Less)
        } else {
            Some(Ordering::Equal)
        };
    }
}

fn main() {
        let good1 = Good {
        price: 1.0
    };

    let good2 = Good {
        price: 2.0
    };
    println!("good1 > good1 ? {}", good1 > good2);
}
// 运行结果
// good1 > good2 ? true
```

`partial_cmp` 方法返回值是 `Option<Ordering>` 类型，这里需要注意下（有关`Option`类型相关的知识点请前往 Rust 中级教程 第1课）。`Option` 类型包含一个 `Ordering` 枚举。如果返回 `Greater` 表示当前值比相比较的另一个值大，`Less` 表示当前值比相比较的另一个值小，`Equal` 则表示相等。

`Ordering` 官方源码（来自`cmp.rs`）：

```rust
pub enum Ordering {
    /// An ordering where a compared value is less than another.
    #[stable(feature = "rust1", since = "1.0.0")]
    Less = -1,
    /// An ordering where a compared value is equal to another.
    #[stable(feature = "rust1", since = "1.0.0")]
    Equal = 0,
    /// An ordering where a compared value is greater than another.
    #[stable(feature = "rust1", since = "1.0.0")]
    Greater = 1,
}
```




## Default trait

前置知识 Sized 和 ?Sized

`Sized` 和 `UnSized` 这是一种标记 trait (marker trait)，他没有方法或者关联类型。

Rust 为其适用的所有类型都自动实现了这个 trait，**任何人都不能自己实现**。当然它也不可以同 `derive` 一起使用。

`Sized` 标识固定大小类型，即他们的值在内存中的大小都是相同的。比如每个 f64 类型占8个字节，i32 类型占4个字节等等。

`?Sized` 表示非固定大小类型，即他们的值大小并不固定，比如前面文章讲到过的字符串切片类型 `&str`和 `[T]` 类型的数组。说白了，非固定大小的类型基本都是占两个字节的**胖指针**或者是 **trait object**（有关trait object 的概念暂时不会介绍）。

在我们声明泛型结构体时的时候，通常是 `struct<T>` 来声明。然而 Rust 则会理解为 `struct<T: Sized>`。这是因为非固定大小的类型具有非常大的局限性，因此大多数的泛型都被限制使用 `Sized` 类型。如果你不想限制 `T`，则需要显示的表示出来。

```rust
/// 显式标明
struct MySized<T: ?Sized> {
    value: T,
}
```

有关这 Sized trait 暂时作为了解即可。



`Default` 是 Rust 中提供默认值类型的一个 trait，通常作用于 struct 上。该类型可以同 derive 一起使用，任何人都不能自己。

源码（default.rs）：

```rust
pub trait Default: Sized {
    fn default() -> Self;
}
```

源码很简单，就只有一个 *default* 函数当在结构体上使用 `#[derive(Default)]` 时，Rust 将会自动为其实现一些基本类型参数的默认值。示例代码如下：

```rust
#[derive(Default, Debug)]
struct Test1 {
    a: i32,
    b: f64,
    c: bool,
    d: char,
    e: (),
    f: String,
}

fn main() {
    let test = Test1::default();
    println!("{:#?}", test);
    // 运行结果：
    // Test1 {
    //     a: 0,
    //     b: 0.0,
    //     c: false,
    //     d: '\0',
    //     e: (),
    //     f: "",
    // }
}
```

其实参数的默认值我们也可以从源码中找到：

```
default_impl! { (), (), "Returns the default value of `()`" }
default_impl! { bool, false, "Returns the default value of `false`" }
default_impl! { char, '\x00', "Returns the default value of `\\x00`" }

default_impl! { usize, 0, "Returns the default value of `0`" }
default_impl! { u8, 0, "Returns the default value of `0`" }
default_impl! { u16, 0, "Returns the default value of `0`" }
default_impl! { u32, 0, "Returns the default value of `0`" }
default_impl! { u64, 0, "Returns the default value of `0`" }
default_impl! { u128, 0, "Returns the default value of `0`" }

default_impl! { isize, 0, "Returns the default value of `0`" }
default_impl! { i8, 0, "Returns the default value of `0`" }
default_impl! { i16, 0, "Returns the default value of `0`" }
default_impl! { i32, 0, "Returns the default value of `0`" }
default_impl! { i64, 0, "Returns the default value of `0`" }
default_impl! { i128, 0, "Returns the default value of `0`" }

default_impl! { f32, 0.0f32, "Returns the default value of `0.0`" }
default_impl! { f64, 0.0f64, "Returns the default value of `0.0`" }
```

使用 `#[derive(Default)]` 标记的结构体，其参数也必须都实现 `Default` trait。另外，我们也可以自己实现 `Default` trait。示例代码如下：

```rust
#[derive(Debug)]
struct Test2 {
    a: i32,
    b: f64,
    c: bool,
}

/// 默认实现 Default
impl Default for Test2 {
    fn default() -> Self {
        return Self {
            a: 10,
            b: 20.0,
            c: true,
        };
    }
}

fn main() {
    let test2 = Test2::default();
    println!("{:#?}", test2);
    // 运行结果
    // Test2 {
    //     a: 10,
    //     b: 20.0,
    //     c: true,
    // }
}
```

编译器做了什么？

看到这里，大家有没有好奇，我们添加 `#[derive]` 后，编译器到底做了什么呢？其实，我们可以通过 `cargo-expand` 看到。以`Debug` 为例。我们将下面的代码通过命令展开。

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rec = Rectangle {
        width: 3,
        height: 5,
    };
    println!("{:?}", rec);
}
```

首次使用请先通过以下命令安装。

```
cargo install cargo-expand
```

**下面的命令必须使用 Rust nightly 版本**

```
 cargo rustc --profile=check -- -Zunpretty=expanded
```

展开后的代码如下：

```
#![feature(prelude_import)]                                                                                                                                                           
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
struct Rectangle {
    width: u32,
    height: u32,
}
#[automatically_derived]
impl ::core::fmt::Debug for Rectangle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(f, "Rectangle",
            "width", &&self.width, "height", &&self.height)
    }
}

fn main() {
    let rec = Rectangle { width: 3, height: 5 };
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(&["", "\n"],
                &[::core::fmt::ArgumentV1::new_debug(&rec)]));
    };
}

```

很清楚的可以看到，编译器为 `Rectangle` 结构体实现了 `Debug` trait。
