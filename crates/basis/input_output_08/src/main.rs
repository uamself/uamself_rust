fn main(){
    let a = 31;
    let b = [1, 3, 5, 7, 9];

    println!("二进制 {:b}", a);
    println!("八进制 {:o}", a);
    println!("十六进制(小写) {:x}", a);
    println!("十六进制(大写) {:X}", a);
    println!("科学计数(小写) {:e}", 100000_f32);
    println!("科学计数(大写) {:E}", 100000_f32);
    println!("打印Debug {:?}", b);
    println!("输出标点 {:+}", 5);

    println!("前置符二进制 {:#b}", a);
    println!("前置符八进制 {:#o}", a);
    println!("前置符十六进制(小写) {:#x}", a);
    println!("前置符十六进制(大写) {:#X}", a);
    println!("带换行和缩进的Debug打印 {:#?}", b);

    println!("使用大于号右对齐 {:>6}{:>6}{:>6}", 1, 2, 3);
    println!("使用小于号左对齐 {:<6}{:<6}{:<6}", 1, 2, 3);
    println!("省略大于号右对齐 {:6}{:6}{:6}", 1, 2, 3);
    println!("居中对齐 {:^6}{:^6}{:^6}", 1, 2, 3);
    println!("填充任意字符居中对齐 {:-^7}{:*^6}{:1^6}", 1, 2, 3);

    println!("二进制8位补零 {:08b}", a);
    println!("八进制8位补零 {:08o}", a);
    println!("十六进制16位补零 {:016b}", a);

    println!("{:1$}", "a", 6);
    println!("{1:0$}", 6, "a");
    println!("{:width$}", "a", width = 6);

    println!("小数保留位数 {:.3} ", 0.01);
    println!("小数保留位数 {1:.0$} ", 3, 0.01);
    println!("{}小数保留3位数 {:.*} --- 保留4位数 {:.*} ", 0.01, 3, 0.01, 4, 0.10);

    // -----------------{{}}--------------------
    println!("左边的括号  {{");
    println!("右边的括号  }}");
    println!("全括号  {{}}");

    // --------------------stdin-----------------
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("read_line error!");
        println!("你输入的内容是 : {}", input);
        if input.contains("end") { break;  }
    }

}