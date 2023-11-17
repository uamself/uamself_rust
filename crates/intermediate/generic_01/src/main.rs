// fn print_generic<T>(a: T, _b: T) -> T {
//     a
// }
//
// fn main() {
//     let b = print_generic::<f32>(6.7, 4.5);
//     println!("b = {}", b);
//
//     // 输出结果
//     // b = 6.7
//
//     let c = print_generic::<&str>("hello", "rust");
//     println!("c = {}", c);
//
//     // 输出结果
//     // c = hello
// }
//
//



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

