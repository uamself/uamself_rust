# 向量(vector)

## 向量的定义

Vec是一种动态的可变数组，可以在运行时增长或者缩短数组的长度。其签名形式`Vec<T>`，T表示任意类型。`Vec<T>`叫做T类型的向量。向量的元素保存在堆上，这也是它可以动态增长或者缩短长度的原因。向量中的每个元素都分配有唯一的索引值，同数组一样，索引从0开始计数。


## 向量的创建

向量常见的创建方式有以下3种：

1、使用`Vec::new()`创建一个空的向量。容量为`0`，长度为`0`的向量在堆上是不占内存的。通常创建向量时使用`mut`修饰，因为向量需要动态增删元素。需要指定类型。

```rust
let mut vec_empty: Vec<i32> = Vec::new();
```

2、[推荐方法] 初始化时为向量指定容量。容量不等于长度，不要跟长度混淆。下面是容量为10的空向量（长度为0）。需要指定类型。

```rust
let mut vec_capacity :Vec<i32> = Vec::with_capacity(10);
```

3、[推荐方法] 使用"宏"创建向量。这种方式的创建方法类似于数组的语法。它也有3种创建方式。

* 创建空的向量。需要指定类型。

```
 let mut vec_marco: Vec<i32> = vec![];
```

* 创建带有默认元素的向量。支持类型推断。

```rust
let mut vec_marco = vec![1, 2, 3, 4, 5];
```

* 创建指定长度且初始化所有元素的向量。支持类型推断。

```
// 长度为5，元素初始化为0
let mut vec_marco = vec![0; 5]; 
```


## 添加元素

使用`push`方法在向量的尾部添加新元素。下面的代码是在向量后面追加了元素`1`。添加元素需要将向量使用`mut`关键字修饰。

```rust
let mut vec_push = vec![0; 5];
vec_push.push(1);
dbg!(&vec_push); 
```

```rust
 &vec_push = [
    0,
    0,
    0,
    0,
    0,
    1,
]

```



## 修改元素


修改元素，直接使用“变量名称[索引] = 要修改的值”即可重新为元素赋值。

下面的代码是将向量的第4个元素（索引值3的元素）的值修改为1。修改元素需要将向量使用`mut`关键字修饰。

```rust
let mut vec_push = vec![0; 5];
vec_push[3] = 1;
dbg!(&vec_push); 
```

```rust
 &vec_push = [
    0,
    0,
    0,
    1,
    0,
]

```



## 删除元素

Rust中有两种删除向量的元素的方式。删除元素需要将向量使用`mut`关键字修饰。

1、通过`pop`方法弹出队尾元素。调用`pop`方法会返回一个`Option`枚举类型。如果数组不为空，则会返回`Some(v)`，`v`是弹出的值。如果向量为空，则会返回`None`（关于`Option`这里了解即可）。

示例代码如下：

```rust
    let mut vec_pop = vec![1];
    let pop = vec_pop.pop();
    dbg!(pop);
    let pop = vec_pop.pop();
    dbg!(pop);
```

代码执行结果：

```
[src\main.rs:31] pop = Some(
    1,
)
[src\main.rs:33] pop = None
```

2、通过`remove`方法删除元素，需要传入将要删除元素的索引，并且返回删除的元素。这个操作会引发向量元素的移位，被删除元素的后面元素都会相应的左移一位。如果传入的索引大于向量的长度，则会产生程序错误。

示例代码如下：

```rust
    let mut vec_remove = vec!['w', 'o', 'r', 'l', 'd'];
    let remove_element = vec_remove.remove(3);
    dbg!(remove_element);
    // 索引越界，会发生错误
    // let remove_element = vec_remove.remove(5);
```

代码执行结果：

```
[src\main.rs:38] remove_element = 'l'

// 下面是越界的执行结果
thread 'main' panicked at 'removal index (is 5) should be < len (is 4)', library\alloc\src\vec\mod.rs:1347:13
stack backtrace:
```


## 访问元素

Rust中也存在两种方式去访问向量中的元素。

1、使用"向量名称[索引]"的方式访问指定元素，类似于修改元素的访问。如果传入的索引大于向量的长度，则会产生程序错误，常称作数组越界。

示例代码如下：

```rust
    let vec_find = vec![1, 2, 3];
    dbg!(vec_find[0]);
    dbg!(vec_find[1]);
    dbg!(vec_find[2]);  
```

代码执行结果：

```
[src\main.rs:44] vec_find[0] = 1
[src\main.rs:45] vec_find[1] = 2
[src\main.rs:46] vec_find[2] = 3
```

2、使用`get`方法访问元素，传入的参数同样是索引，但是其返回的是`Option`枚举类型，如果索引越界，不会产生错误，则会返回`None`。（关于`Option`这里了解即可）。

示例代码如下：

```rust
    let vec_get = vec![1, 2, 3];
    dbg!(vec_get.get(0));
    dbg!(vec_get.get(1));
    dbg!(vec_get.get(2));
    // 下面代码不会产生错误，正常执行
    dbg!(vec_get.get(3));
```

代码执行结果：

```
[src\main.rs:52] vec_get.get(0) = Some(
    1,
)
[src\main.rs:53] vec_get.get(1) = Some(
    2,
)
[src\main.rs:54] vec_get.get(2) = Some(
    3,
)
[src\main.rs:56] vec_get.get(3) = None 
```


## 向量容量的变化

### 向量容量增长策略

向量的容量（Capacity）是指为存储元素所分配的空间。向量的长度（Length）如果大于其当前的容量，则会发生重新分配空间的操作，这个过程比较耗时。假设当前数组的容量为5，如果向量内的元素小于等于5个，则其容量不会增加，如果元素超过5个，则向量的容量就会重新分配，在原有的基础上乘以2（目前2是向量的增长因子），最后容量会变成10。因此尽可能的在初始化时为其指定合适的容量。

可以通过`len()`方法获取向量的长度，`capacity()`方法获取向量的容量，容量变化的示例代码如下：

```rust
    let mut vec_capacity :Vec<i32> = Vec::with_capacity(5);
    // 动态增长
    println!("vec_capacity 填充元素前的长度为 {}", vec_capacity.len());
    println!("vec_capacity 填充元素前的容量为 {}", vec_capacity.capacity());
    // 关于循环这里了解即可，这里的意思是，填充0,1,2,3,4五个元素
    for i in 0..5 {
        vec_capacity.push(i);
    }
    println!("vec_capacity 填充5个元素后的长度为 {}", vec_capacity.len());
    println!("vec_capacity 填充5个元素后的容量为 {}", vec_capacity.capacity());

    // 填充第6个元素
    vec_capacity.push(5);

    println!("vec_capacity 填充6个元素后的长度为 {}", vec_capacity.len());
    println!("vec_capacity 填充6个元素后的容量为 {}", vec_capacity.capacity());
```

代码执行结果：

```
vec_capacity 填充元素前的长度为 0
vec_capacity 填充元素前的容量为 5
vec_capacity 填充5个元素后的长度为 5
vec_capacity 填充5个元素后的容量为 5
vec_capacity 填充6个元素后的长度为 6
vec_capacity 填充6个元素后的容量为 10
```

### 首次填充元素的默认容量

咱们再来看另外一个问题，假设创建一个容量为0的向量，然后向其中添加元素，这时容量会变成多少呢？

示例代码如下：

```rust
    let mut vec_default: Vec<i32> = Vec::new();
    println!("vec_default 的长度为 {}", vec_default.len());
    println!("vec_default 的容量为 {}", vec_default.capacity());
    vec_default.push(1);
    println!("vec_default 添加一个元素的长度为 {}", vec_default.len());
    println!("vec_default 添加一个元素的容量为 {}", vec_default.capacity());
```

代码执行结果：

```
vec_default 的长度为 0
vec_default 的容量为 0
vec_default 添加一个元素的长度为 1
vec_default 添加一个元素的容量为 4
```

从上面程序的运行结果，可以得知，首次创建空容量的向量，向其中添加一个元素后，则会将容量变调整为`4`。之后的容量调整则遵循增长因子为2的规则。

## 数组与向量的区别

* 原生数组（Array）的签名是`[T;N]`，而向量（Vector）的签名是`Vec<T>`
* 原生数组（Array）的大小是在编译时确定的常量，也是类型自身的一部分。其大小不更改，不能向数组中增加或者缩短数组的长度。向量（Vector）是一种可以动态分配增长或者缩减的数组，在运行时才会确定向量的长度。
* 原生数组（Array）的元素可以在栈上存储，而向量（Vector）的元素只能在堆上分配。


## 小结

其实Vec<T>包好了3个值：分别是：对分配在对上用于保存元素的缓冲区的引用，该缓冲区可以存储元素的个数（Capacity），当前实际存储的元素个数（Length）。如果提前知道向量的容量，推荐使用`Vec::with_capacity`。现在知道了向量是由三部分组成，那么它在内存中的模型是什么样的呢？关于向量和数组该如何遍历？排序？其它高级用法？这一系列的问题将会在后续章节一一介绍，这节课仅仅是简单介绍了向量这种数据类型。
