fn main(){
    let mut vec_push = vec![0; 5];
    vec_push.push(1);
    //dbg!(&vec_push);



    let mut vec_push = vec![0; 5];
    vec_push[3] = 1;
    //dbg!(&vec_push);

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


    let mut vec_default: Vec<i32> = Vec::new();
    println!("vec_default 的长度为 {}", vec_default.len());
    println!("vec_default 的容量为 {}", vec_default.capacity());
    vec_default.push(1);
    println!("vec_default 添加一个元素的长度为 {}", vec_default.len());
    println!("vec_default 添加一个元素的容量为 {}", vec_default.capacity());
}