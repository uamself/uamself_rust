# 迭代器

## 迭代器的定义

迭代器模式是将遍历数据集合的行为抽象为单独的迭代对象，在遍历集合时把集合中所有元素按顺序传递给处理逻辑。**Iterator**是一个**trait**。

```rust
trait Iterator {

    type Item;

    fn next(&mut self) -> Option<Self::Item>

    // 其它方法

}
```

上面是一个简化的迭代器**trait**，**Item**是迭代器迭代过程中产生值的类型。**next**方法返回值是一个**Option**，如果返回**Some(v)**，则**v**表示迭代器的下一个值，如果返回**None**，则迭代器将会终止。

我们可以通过集合的**iter()**方法得到**Iterator**，接收迭代器的变量必须是可变的。

迭代器内的其它方法暂时忽略。**trait**将会在后续章节介绍。

示例代码如下：

```rust
    let vec = vec![3,4,5];
    let mut iter = vec.iter();
    dbg!(iter.next());
    dbg!(iter.next());
    dbg!(iter.next());
    dbg!(iter.next());
```

```rust
[src\main.rs:6] iter.next() = Some(3,)
[src\main.rs:7] iter.next() = Some(4,)
[src\main.rs:8] iter.next() = Some(5,)
[src\main.rs:9] iter.next() = None
```

## 迭代器与for循环

迭代器也可以配合for循环来使用。迭代器与for连用比较简单，直接上代码。

示例代码如下：

```rust
    let vec_for = vec![1, 2, 3, 4, 5];

    for i in vec_for.iter() {
        print!("{} ", i);
    }
```


## 迭代器与消费器

- sum 求和消费器

  它可以去迭代器内的元素进行累加。示例代码如下：

  ```rust
      let vec_consumer = vec![2, 4, 6, 8, 10];
      let sum_result: i32 = vec_consumer.iter().sum();
      dbg!(sum_result);
  ```
- any 条件消费器

  它可以判断迭代器内的元素是否存在满足某个条件的元素。返回值是布尔类型。示例代码如下：

  ```rust
      let vec_consumer = vec![2, 4, 6, 8, 10];
      let any_result = vec_consumer.iter().any(|x| *x % 2 != 0);
      dbg!(any_result);
  ```
- collect 收集消费器

  它可以将迭代器转换成指定的容器类型。下面示例代码如下：

  ```rust
  let collect_result: Vec<i32> = vec_consumer.iter().map(|x| x - 1).collect();
  dbg!(collect_result);
  ```

## 迭代器与适配器

在**Iterator trait**中还有一系列方法叫做适配器，这些方法还支持链式调用。

常见的适配器方法有**map、take、filter、rev、zip**等。下面是详细介绍。

### map

**map**方法可以让每个元素调用闭包内的方法，最后再将元素收集起来，生成一个新的容器。常与**collect**方法连用。而在上面已经介绍过了，这里就不再赘述了。

### take

生成一个仅迭代原迭代器中前n个元素的新迭代器。常用于变量指定数量元素的场景。生成的是一个**Take**结构体，包含了原迭代器和长度。有关结构体的知识下节会介绍。

示例代码如下：

```rust
    let vec_adapter = vec![1, 3, 5, 7, 9];
    let take_result = vec_adapter.iter().take(3);
    dbg!(take_result);
```

代码运行结果

```rust
take_result = Take {

    iter: Iter(
        [
            1,
            3,
            5,
            7,
            9,
        ],
    ),
    n: 3,

}
```


### filter

对迭代器中每个元素调用闭包生成一个过滤元素的新迭代器。

闭包返回值一定是布尔类型。

如果是**true**则当前元素放入迭代器内，反之当前元素将被忽略。

最后需要通过**collect**方法收集成新的迭代器。

示例代码如下:

```rust
let vec_adapter = vec![1, 3, 5, 7, 9]; //3 5 7 9 11
let filter_result: Vec<i32> = vec_adapter.iter().map(|x| *x + 2).filter(|x| *x % 3 == 0).collect();
dbg!(filter_result);
```

代码运行结果如下:

```rust
filter_result = [

    3,
    9,

]


```

### rev

将迭代器逆转后生成新的迭代器。返回值是**Rev**结构体，当遍历该结构体时会从后往前遍历。

示例代码

```rust
    let vec_adapter = vec![1, 3, 5, 7, 9];
    let rev_result = vec_adapter.iter().rev();

    dbg!(&rev_result);
    for i in rev_result {
        print!("{} ", i);
    }

```

代码运行结果

```rust
&rev_result = Rev {

    iter: Iter(
        [
            1,
            3,
            5,
            7,
            9,
        ],
    ),

}

9 7 5 3 1 
```

### zip

将两个迭代器压缩成一个新的迭代器。

实际上是将将两个迭代器同时迭代并返回一个元组，第一个元素来自第一个迭代器，第二个元素来自第二个迭代器。

示例代码如下:

```rust
    let vec1 = vec![3, 5, 7];
    let vec2 = vec![2, 4, 6];
    // (3,2) (5,4) (7,6)
    let vec_zip: Vec<i32> = vec1.iter().zip(vec2.iter()).map(|x| { x.0 + x.1 }).collect();
    dbg!(vec_zip);
```

运行结果:

```rust
vec_zip = [
    5,
    9,
    13,

]
```

## 小结

本节讲了一些迭代器的基础用法，常用的迭代器方法一般也就是**any、map、filter、collect**。有关迭代器的内容其实还有很多，本节课仅仅是简单介绍了下迭代器。关于迭代器更高级的用法我将在进阶章节里继续讲解。
