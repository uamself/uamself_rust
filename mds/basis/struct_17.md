# 结构体(struct)

## 结构体定义

Rust中有三种结构体类型：

- 命名字段（name-field）结构体、
- 类元组（tuple-like）结构体，
- 类基元（unit-like）结构体。

我们常见的也就是前两种，类基元结构体虽不常见，但却很有用哦。

结构体使用关键字`struct`声明，后面紧接着结构体的名称。

```rust
struct Student{}
```

结构体的命名遵循**驼峰命名法**。


## 命名字段结构体（Name-Field）

命名字段结构体的每一部分数据，被称作字段(Field).

结构体中以`name:T`格式定义字段。`T`表示数据类型，`name`表示字段的名称。

其字段名遵循变量的命名规则。结构体的字段必须声明其类型，**不支持自动类型推断**，每个字段需要用英文逗号分隔开。

```rust
    // 某游戏账号结构体
    struct Account {
        // 账号id i32
        id: u32,
        // 账号状态 是否是正常状态 true:正常 false:异常
        status: bool,
        // 账号类型 'n'是普通用户 's'是高级用户
        acc_type: char,
    }
```

### 实例化命名字段结构体

结构体实例化也需要使用`let`关键字，声明了一个结构体，可以认为它是我们自定义的一种数据类型。下面的代码就是实例化一个结构体。

```rust
// 不可变实例
let account = Account{
    id : 1,
    status : false,
    acc_type: 's',
};
```


```rust
// 可变实例
let mut account = Account{
    id : 1,
    status : false,
    acc_type: 's',
};
```


还有再介绍下另外的两种语法糖。下面两种情况可以算作是扩展，可以了解下。

#### 外部变量名与结构体属性相同

当前外部存在变量名称与结构体内的属性名称相同时，可简写。

如下面代码中的 `id` 这个字段，我们想把外部的 `id` 赋值给结构体内的 `id` 属性，可以将 `id: id` 简写成 `id`。

```rust
    let id = 1;
    let mut account = Account {
        id, // 这里的 id 等同于 id: id,
        status: false,
        acc_type: 's',
    };
```



#### 使用其它实例的字段实例化结构体

可能存在某种场景，结构体 `S` 的多个示例 `A`, `B`, `C` 等它们存在相同的字段。那么可以采用这种方式简化实例化结构体的操作。

我们以 `Account` 为例，每当实例化一个结构体时，仅仅是 `id` 不同，其余字段相同。可以看到下面的代码中使用**..M**(M表示某结构体的实例)表示**对除某字段外未外显示设置值的字段以M实例对应字段的值来赋值**。

```rust
    let id = 1;

    let mut account = Account {
        id, // 这里的 id 等同于 id: id,
        status: false,
        acc_type: 's',
    };

    let mut account2 = Account {
        id: 3,
        ..account // 其余字段使用account实例对应的字段
    };

    let mut account3 = Account {
        id: 4,
        ..account // 其余字段使用account实例对应的字段
    };
```


### 修改和访问命名字段结构体

修改和访问结构体都是用 "**实例名 . 字段名**"的形式。

访问:

```rust
println!("某游戏账号的 id 是 {}, 当前的用户状态: {}, 用户类型为 {}", account.id, account.status, account.acc_type);// [运行结果]// 某游戏账号的 id 是 1, 当前的用户状态: false, 用户类型为 s
```

修改:

```rust
    account.id = 99;
    account.status = true;
    account.acc_type = 'n';
    println!("[修改后]某游戏账号的 id 是 {}, 当前的用户状态: {}, 用户类型为 {}", account.id, account.status, account.acc_type);// [运行结果]// [修改后]某游戏账号的 id 是 99, 当前的用户状态: true, 用户类型为 n
```


### 公有和私有

Rust中的结构体与其它类型一样，默认是私有的，只在声明它的模块中可见。

要想让结构体对外部模块可见，需要在其定义之前加上**pub**关键字。

当然，结构体的字段默认也是私有的，即使结构体声明为共有，其字段仍可以私有。代码如下：

```rust
// 结构体公有 字段私有
pub struct Circle {
    r: i32
}

// 结构体公有 字段公有
pub struct Circle {
    pub r: i32  
}
```


## 类元组（Tuple-Like）结构体

类元组结构体，因为它类似于元组，网络上有些文章叫"元组结构体"。

类元组结构体的值称为**元素（Element）**，其创建方式和访问方式与元组基本一致。直接上代码。

```rust
    // 声明类元组结构体
    struct Point(i32, i32);
    // 创建类元组结构体
    let mut point = Point(1, 1);
    // 修改值
    point.0 = 10;
    // 访问值
    println!("Point{{x = {}, y = {}}}", point.0, point.1); //Point{x = 10, y = 1}
```

另外，类元组结构体中的元素也可以声明为公有元素。如下：

```rust
pub struct Point(pub i32, pub i32);
```


## 类基元（unit-like）结构体

这种结构体个人建议了解即可，但是在某些情况下也是有用的，后面的文章遇到会继续讨论。

**类基元（unit-like）结构体是一种没有任何元素的结构体**。

```rust
    // 声明
    struct UnitStruct;
    // 创建
    let us = UnitStruct;
```

**类基元结构体的值不占内存，与基元类型相似（）。Rust并不会把类基元结构体的值保存到内存里，更不会生成操作它们的代码。这种类型只有一个值。**


## 结构体布局(todo)

重点来了，我们开始讨论Rust中结构体的布局。以下面的结构体为例：

```rust
    struct Salary {
        // 表示月薪
        monthly: Vec<u32>,
        // 表示奖金
        bonus: u32,
    }
    // 我的薪资每个月，10,000元RMB，共12个月
    // 另外我的年终奖是 66,666元
    let mut my_salary = Salary {
        monthly: vec![10_000; 12],
        bonus: 66_666,
    };
```
