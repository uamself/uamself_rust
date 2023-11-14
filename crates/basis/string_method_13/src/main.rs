fn main() {
    // ==================== 追加 ===================
    let mut string = String::from("hello ");
    println!("原有字符串为{}", string);

    string.push('r');
    println!("追加字符r push() -> {}", string);

    string.push_str("ust!");
    println!("追加字符串 push_str() -> {}", string);

    //=================== 插入 ====================
    let mut insert_string = String::from("hello rust!");

    insert_string.insert(5, ',');
    println!("插入字符 insert() -> {}", insert_string);

    insert_string.insert_str(6, " I like");
    println!("插入字符串 insert_str() -> {}", insert_string);

    // ==========1、使用+或者+=连接字符串 ==============
    let string_append = String::from("hello ");
    let string_rust = String::from("rust");
    // &string_rust会自动解引用为&str
    let result = string_append + &string_rust;
    let mut result = result + "!";
    result += "!!!";

    println!("连接字符串 + -> {}", result);

    // ========= 2、使用format!连接字符串 ================
    let s1 = "hello_";
    let s2 = String::from("_rust");
    let s = format!("{} {}!", s1, s2);
    println!("{}", s);

    // ===========  replace =============
    let string_replace = "I like rust. Learning rust is my favorite!";
    let new_string_replace = string_replace.replace("rust", "RUST");
    println!("{}", new_string_replace);

    // ===========  replacen =============
    let string_replacen = "I like rust. Learning rust is my favorite!";
    let new_string_replacen = string_replacen.replacen("rust", "RUST", 1);
    println!("{}", new_string_replacen);

    // =============== replace_range =================
    let mut string_replace_range = String::from("I like rust!");
    string_replace_range.replace_range(7..8, "R");
    println!("{}", string_replace_range);


    // ================== pop ======================
    let mut string_pop = String::from("rust pop 中文!");
    let p1 = string_pop.pop();
    let p2 = string_pop.pop();
    dbg!(p1);
    dbg!(p2);
    dbg!(string_pop);
    // ================== remove ======================
    let word = "中";
    let ch = "1";
    println!("word 占 {} 个字节", std::mem::size_of_val(word));
    println!("ch 占 {} 个字节", std::mem::size_of_val(ch));

    let mut string_remove = String::from("测试remove方法");
    println!("string_remove 占 {} 个字节", std::mem::size_of_val(string_remove.as_str()));
    // 删除第一个汉字
    string_remove.remove(0);
    // 下面代码会发生错误
    // string_remove.remove(1);
    // 直接删除第二个汉字
    // string_remove.remove(3);
    dbg!(string_remove);

    // ================== truncate  ======================
    let mut string_truncate = String::from("测试truncate");
    string_truncate.truncate(3);
    dbg!(string_truncate);
    // ================== clear ======================
    let mut string_clear = String::from("string clear");
    string_clear.clear();
    dbg!(string_clear);
    println!("end")
}