# 切片引用(slice)

## 切片定义

官方定义：Slice是对向量或者数组中部分元素序列的引用。

其签名形式为&[T]和&mut [T]，分别叫做T类型的共享切片和T类型的可修改切片。

通俗易懂点说，*Slice*就是表示数组或者向量的一个范围。

*Slice*从严格意义上讲，应该叫做对切片的引用。由于提到切片大都是指对它的引用，所以习惯上就把“切片引用”省略为“切片”了。


示例代码如下：

```rust
    let vec = vec![1, 3, 5, 7, 9];

    let array = [0, 2, 4, 6, 8];

    let vec_slice: &[i32] = &vec;
    let array_slice: &[i32] = &array;

    dbg!(vec_slice);
    dbg!(array_slice);
```

代码执行结果：

```rust
[src\main.rs:8] vec_slice = [

    1,
    3,
    5,
    7,
    9,
]
[src\main.rs:9] array_slice = [
    0,
    2,
    4,
    6,
    8,
]
```

## 在切片中使用范围

在切片中可以使用范围来切割数组或者向量，签名为：&数组或向量名称[范围]。切片中的范围分为以下3种：

1、前闭后开。形如：`[a..b]`。表示从a到b的范围，包含a不包含b。

2、0到指定位置。形如：`[..b]`。表示从0到b的位置，不包含b。

3、指定位置到结束。形如：`[a..]`。表示从a到结束，包含a。

4、全部。形如：`[..]`。表示从0到结束的全部。等同于直接引用。

示例代码如下：

```rust
    let vec = vec![1, 3, 5, 7, 9];

    let array = [0, 2, 4, 6, 8];

    let vec1 = &vec[1..3];
    let vec2 = &vec[..2];
    let vec3 = &vec[3..];
    let vec4 = &vec[..];

    println!("vec1 => vec中下标1到下标3的 元素 {:#?}", vec1);
    println!("vec2 => vec中下标0到下标2的 元素 {:#?}", vec2);
    println!("vec3 => vec中下标3到结束的 元素 {:#?}", vec3);
    println!("vec4 => vec中下标0到结束的 元素 {:#?}", vec4);
    // 相同
    assert_eq!(&vec[..], &vec);

    let array1 = &array[1..3];
    let array2 = &array[..2];
    let array3 = &array[3..];
    let array4 = &array[..];

    println!("array1 => array中下标1到下标3的 元素 {:#?}", array1);
    println!("array2 => array中下标0到下标2的 元素 {:#?}", array2);
    println!("array3 => array中下标3到结束的 元素 {:#?}", array3);
    println!("array4 => array中下标3到结束的 元素 {:#?}", array4);
    // 相同
    assert_eq!(&array[..], &array);
```


代码运行结果：

```rust
vec1 => vec中下标1到下标3的 元素 [
    3,
    5,
]
vec2 => vec中下标0到下标2的 元素 [
    1,
    3,
]
vec3 => vec中下标3到结束的 元素 [
    7,
    9,
]
vec4 => vec中下标0到结束的 元素 [
    1,
    3,
    5,
    7,
    9,
]
array1 => array中下标1到下标3的 元素 [
    2,
    4,
]
array2 => array中下标0到下标2的 元素 [
    0,
    2,
]
array3 => array中下标3到结束的 元素 [
    6,
    8,
]
array4 => array中下标3到结束的 元素 [
    0,
    2,
    4,
    6,
    8,
]

```

`assert_eq!`也是一个宏，这是一个断言，断言参数里面的两个值相等。如果不相等则会抛出错误。

## 切片元素的访问


切片的访问与数组和向量类似，同样也会坚持切片的索引是否有效，如果超出长度则会出现错误。

示例代码如下：

```rust
    let vec = [1, 3, 5, 7, 9];

    let vec_s = &vec[1..4];
    dbg!(vec_s[2]);
```

代码运行结果：

```rust
[src\main.rs:47] vec_s[2] = 7
```

## 切片元素的修改


想要修改切片中的元素，切片引用的原数组或者向量必须是可变的，也就是必须使用`mut`关键字修饰，且切片类型为可修改切片&mut [T]。切片的值一旦被修改，切片引用的原数组或者向量的值同样被修改。因为切片指向的值和原数组或者向量的值是同一块内存区域。

示例代码如下：

```rust
    // 必须mut修饰
    let mut vec = [1, 3, 5, 7, 9];
    // 声明为可修改的切片
    let vec_m = &mut vec[1..4];
    vec_m[2] = 10;

    dbg!(vec_m);
    dbg!(vec);
```

## 切片的常用方法


1、`len()` ——> 获取切片的长度。

2、`is_empty()`——> 判断切片是否为空，返回值为布尔型。

```rust
    let vec = [1, 3, 5, 7, 9];

    let vec_s = &vec[0..0];

    println!("切片 vec_s 的长度是{} ", vec_s.len());
    println!("切片 vec_s 是空吗？{} ", vec_s.is_empty());
```

```rust
切片 vec_s 的长度是0 

切片 vec_s 是空吗？true 
```
