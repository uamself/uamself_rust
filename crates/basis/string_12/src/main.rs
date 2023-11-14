fn main(){
    let _a = String::from("this is book");
    let _b = String::new();

    let _c: Vec<i32> = Vec::new();


    // 换行
    let hello4 = "My name is Betty, 18 years old. I like play piano very much and was awarded
    of a numbers of prizes for that.";
    println!("hello4 = {}", hello4);

    // 忽略换行符
    let hello5 = "My name is Betty, 18 years old. I like play piano very much and was awarded \
    of a numbers of prizes for that.";
    println!("hello5 = {}", hello5);



    // 测试转义
    let raw_str = r"D:\study_rust\013\string";
    println!("raw_str = {}", raw_str);

    // 测试引号
    let raw_str_ref = r##"测试引号"英文引号",英文引号会原样输出！！"##;
    println!("raw_str_ref = {}", raw_str_ref);


    // 字节字符串
    let byte_str = b"a byte string!";
    println!("byte_str = {:?}", byte_str);

    // 原始字节字符串
    let raw_byte_str = br#"it is a "raw byte string"."#;
    println!("raw_str_ref = {:?}", raw_byte_str);

    let rust_str = "rust";
    let rust_string = String::from(rust_str);
    println!("rust_str 字面量指向的地址 {:?}", rust_str.as_ptr());
    println!("rust_string 指向的地址 {:?}", &rust_string.as_ptr());


    let s1 = "rust_to_string";
    let _s2 = s1.to_string();






    // --------------------------------------------
    let mut hello = String::with_capacity(15);

    hello.push('h');
    hello.push('e');
    hello.push('l');
    hello.push('l');
    hello.push('o');

    println!("hello 字符串的内容 -> {}", hello);

    println!("hello 堆上的指针 -> {:p}", hello.as_ptr());
    println!("hello 的容量 -> {}", hello.capacity());
    println!("hello 的字节长度 -> {}", hello.len());

    println!("hello 栈上的指针 -> {:p}", &hello);
}