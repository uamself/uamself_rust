# Rust规范

## 程序入口

```rust
fn main() {

}
```

## main函数的参数和返回值

```rust
fn main() {
    for arg in std::env::args()
    {
        println!("{}", arg);
    }

    // 返回值，默认返回是0
    std::process::exit(0);
}

```


rust中有专门的函数对入参做处理——`std::env::args()`，他可以接收所有参数。

关于函数的返回值，可以使用`std::process::exit(0);`返回，其中的入参就是返回值了


## 命名规范

### 蛇形命名


`文件名`：例如：hello\_world.rs、main\_xxx.rs

`变量名`：例如：zhangsan\_name

`函数名`：例如：func\_name()

### 大驼峰命名


`结构体`：例如：struct ExampleStruct { name: String}

`enum类型`：例如：enum IpAddress {IPV4(u8,u8,u8,u8)}

### 其他


`关联常量`：全部大写，例如：NAME、AGE

`连接符`：`Cargo`默认把`连接符`“`-`”转换成`下划线`“`_`”

`语句`：跟C，Java语言等一样，每行语句结束都要添加`;`

## 标准输出


`print` 和 `println` 的区别就是多了一个`ln`。`print`是输出内容（不换行），`println`是输出内容并且换行。同理，`eprint`和`eprintln`是一样的。

`stdout` : 标准输出设备。默认是行缓冲的，它的输出数据会保存在一个`buffer`中，当换行的时候会输出到屏幕。如果程序转向输出到文件，它则会输出到文件里面。

`stderr` : 标准错误输出设备 。默认是无缓冲的，会直接输出数据。如果程序转向输出到文件，它则依然会输出到屏幕。


## dbg!

如果你想要调试输出，建议使用`dbg!`，这个命令是对`eprintln`的封装。通过它打印内容，输出内容会带文件名，行号等信息，可以很方便的程序调试。建议需要打印日志时使用`dbg!`来代替`println!`
