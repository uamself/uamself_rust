# 泛型

## 泛型的定义

其实泛型我们已经见过它了，前面介绍的Vec\<T\>就是泛型的应用，T可以代指任何类型。在 Rust 中，\<T>代表泛型类型了。

**泛型是指在运行时指定数据类型的机制，使用泛型可以编写通用的代码，减少重复的工作量。**

## 泛型与向量

之前的文章，我们了解了向量。在向量声明时我们需要指定类型，代码示例如下：

```rust
fn main() {
    // 1、 泛型与向量
    let mut vec :Vec<&str> = vec![];
    vec.push("Hello");
    vec.push("Rust");

    println!("{:?}", vec);

    // 下面的语句会产生错误 //如果我们声明向量时标明的泛型类型为 &str ，那我们插入其它类型的元素就会报错。
    // vec.push(1);
}
```

因此，在向量中使用泛型类型，不仅提高了可读性，而且还会及时提醒咱们在写代码时的误操作。

## 泛型函数

在函数中，参数和返回值都可以是泛型类型，我们将带**有泛型类型参数的或者返回值的函数叫做泛型函数**。

下面通过一个 Demo 来讲解泛型函数。

> Question **请写一个函数，输入两个整数，返回第一个参数。**

```rust
fn print_generic<T>(a: T, _b: T) -> T {
    a
}

fn main() {
    let b = print_generic::<f32>(6.7, 4.5);
    println!("b = {}", b);

    // 输出结果
    // b = 6.7

    let c = print_generic::<&str>("hello", "rust");
    println!("c = {}", c);

    // 输出结果
    // c = hello
}

```

通过上面的代码，不仅完成了题目，而且还扩展成了字符串.

## 泛型枚举

枚举类型也可以泛型化。在 Rust 中常见的两个泛型枚举就是 Option<T> 和 Result<T, E> 了。这里主要先看下 Option<T> 官方的定义。

```rust
#[derive(Copy, PartialOrd, Eq, Ord, Debug, Hash)]
#[rustc_diagnostic_item = "Option"]
#[lang = "Option"]
#[stable(feature = "rust1", since = "1.0.0")]
pub enum Option<T> {
    /// No value.
    #[lang = "None"]
    #[stable(feature = "rust1", since = "1.0.0")]
    None,
    /// Some value of type `T`.
    #[lang = "Some"]
    #[stable(feature = "rust1", since = "1.0.0")]
    Some(#[stable(feature = "rust1", since = "1.0.0")] T),
}
```

Option<T> 表示一个可选的值。每个 Option 要么是 Some并包含一个值，要么是 None。通常我们使用它来赋初始值或者遇到错误的时候返回 None。

示例代码如下：

```rust
fn main() {
    let op = Some(5);
    println!("{:?}", op);

    // 除法
    let x1 = divider(5.0, 2.0);
    let x2 =  divider(5.0, 0.0);
    println!("{:?}", x1);
    println!("{:?}", x2);

    // 输出结果
    // Some(5)
    // Some(2.5)
    // None
}

/// 计算除法
fn divider(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        None
    } else {
        Some(a / b)
    }
}
```

在写代码时，我们可以直接使用 Some(T) 和 None 来表示 Option<T>。

另外，泛型枚举这里有个注意点：如果类型 T 是 Box 或其它智能指针类型， Rust 在内存中会省掉 Option<T>的标签字段。

例如：Option<Box\<f64>>在内存中只占一个字节存储。


## 泛型结构体

### 方法与函数

**方法**表示某个类型实例的行为，方法与函数的定义大同小异。

**在结构体中，方法必须定义在 ***impl*** 块中。**方法要求第一个参数必须**是** ****&self** 或者是 **&mut self****，这也是与函数的唯一区别。

**在方法中，使用 &self 读取实例中的数据，使用 &mut self 可以读写实例中的数据**。

### 在结构体中定义方法

```rust
struct Person {
    // 名字
    name: String,
    // 年龄
    age: u8,
}

impl Person {
    // 方法
    fn get_age(&self) -> u8 {
        return self.age;
    }

    fn set_age(&mut self, age: u8) {
        self.age = age;
    }

    fn get_name(&self) -> &str {
        return self.name.as_str();
    }

    fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    //关联函数
    fn to_string(person: Person) -> String {
        return format!("Person {{ name: {}, age: {} }}", person.get_name(), person.get_age());
    }
}

fn main() {
    let mut person = Person {
        name: String::from("test"),
        age: 8,
    };

    println!("修改前：name: {}, age: {}", person.get_name(), person.get_age());

    person.set_name("张三");
    person.set_age(18);

    println!("修改后：name: {}, age: {}", person.get_name(), person.get_age());

    // 通过关联函数输出
    println!("{}", Person::to_string(person))

    // 输出结果：
    // 修改前：name: test, age: 8
    // 修改后：name: 张三, age: 18
    // Person { name: 张三, age: 18 }
}
```


首先我们定义一个结构体 \_Person\_，我们通过 **impl **为其添加方法。其中，所有方法的第一个参数都是 **&self** 或者是 **&mut self**。在 impl 块中，我们定义了 *get\_age，set\_age，get\_name，set\_name* 四个方法。再看 main 函数块内，我们可以通过“**实例名.方法名**”的语法来调用方法。**调用方法时，不需要传递 self 参数，self 参数将由编译器自动传递。**另外，我们还在结构体内定义了一个关联函数 to\_string 来打印内容。我们通过 “**结构体名::函数名**”的语法来调用结构体内的关联函数。


### 泛型结构体

结构体也可以是泛型的，换句话讲，我们可以使用泛型写一个通用的结构体模板。

前面介绍的 Vec 向量就是一个泛型结构体类型，这种结构体可以插入任何数据类型，我们下面以自定义一个"队列"来介绍泛型结构体，先上代码。

```rust
/// 定义一个队列
struct Queue<T> {
    // 通过向量存储数据
    data: Vec<T>,
    // 队列的长度
    size: u32,
}

///  定义队列的方法
impl<T> Queue<T> {
    /// 创建一个队列
    /// 这是一个关联函数
    pub fn new() -> Queue<T> {
        Queue {
            data: vec![],
            size: 0,
        }
    }

    /// 向队列中插入一个值
    /// 这是一个方法
    pub fn offer(&mut self, value: T) {
        self.data.push(value);
        self.size += 1;
    }

    /// 弹出队列的最顶部的元素
    /// 这是一个方法
    pub fn poll(&mut self) {
        if self.size > 0 {
            self.data.remove(0);
            self.size -= 1;
        }
    }
}

fn main() {
    let mut queue: Queue<i32> = Queue::new();
    queue.offer(4);
    queue.offer(7);
    queue.offer(10);
    println!("队列的长度：{}", queue.size);
    queue.poll();
    queue.poll();
    queue.poll();
    queue.offer(10);
    println!("队列的长度：{}", queue.size);

    // 输出结果：
    // 队列的长度：3
    // 队列的长度：1
}


```


首先定义一个队列结构体 **Queue**，定义结构体时，需要在结构体名称后面加上泛型的标签。

我们借助向量来存储数据，再额外维护一个队列的长度 size 变量，简单的队列就定义完成。

接着定义队列的方法，这里与普通的结构体方法略有不同。

在impl 块中，我们分别实现了创建一个队列的关联函数（**new**）和数据入队（**offer**），数据出队两个方法（**poll**）。


## 小结

泛型是 Rust 类型系统中重要的概念，它可以减少我们的代码重复率，编写出干净简洁更为抽象和通用的代码。

泛型不仅可以用在向量、函数、枚举，结构体中,还可以用于集合，trait，方法中，后面会一一介绍。
