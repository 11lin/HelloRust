use ferris_says::say;
use std::io::{stdout, BufWriter};
use std::net::SocketAddr;
use socket2::{Socket,Domain,Type,SockAddr};

use rand::Rng;
use std::cmp::Ordering;
use std::io;
// Hello World
// fn main() {
//     println!("Hello, world!");

//     let stdout = stdout();
//     let message = String::from("Hello fellow Rustaceans!");
//     let width = message.chars().count();

//     let mut writer = BufWriter::new(stdout.lock());
//     say(message.as_bytes(), width, &mut writer).unwrap();    
// }

fn main() {
    println!("猜数字 1-6");
    let secret_number = rand::thread_rng().gen_range(1..7);
    loop {
        println!("请输入您猜的数字:");
        let mut guess = String::new();
    
        io::stdin().read_line(&mut guess).expect("输入错误");
        let guess:u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{} 不是正确的数字.",guess.trim());
                continue;
            },
        };
        println!("您猜的数字：{},机器人的数字：{}",guess,"secret_number");
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("小了！"),
            Ordering::Greater => println!("大了!"),
            Ordering::Equal => {
                println!("恭喜您猜对了!");
                break;
            },
        }
    }
}