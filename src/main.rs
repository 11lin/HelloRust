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

fn guess_number() {
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

fn rock_paper_shotgun() {
    println!("开始石头剪刀布游戏!");

    fn cmp(a:u32,b:u32) -> Ordering {
        if (a == 1 && b == 3)||(a==2 && b== 1)||(a==3 && b== 2) {
            return Ordering::Less
        }else if (a == 1 && b == 2)||(a==2 && b== 3)||(a==3 && b== 1) {
            return Ordering::Greater
        }
        
        Ordering::Equal
    }

    fn get_char(c:u32) -> &'static str {
        match c {
            1 => "石头👊",
            2 => "剪刀✌️",
            3 => "布🖐",
            _ => "Err",
        }
    }
    loop {
        println!("1 石头👊");
        println!("2 剪刀✌️");
        println!("3 布🖐");
        
        let game_number = rand::thread_rng().gen_range(1..4);
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("输入错误");
        let guess:u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{} 不是正确的数字.",guess.trim());
                continue;
            },
        };

        println!("你出的:{},机器人出:{}",get_char(guess),get_char(game_number));
        match cmp(guess, game_number) {
            Ordering::Less => println!("你输了！"),
            Ordering::Equal => println!("不输不赢!"),
            Ordering::Greater => {
                println!("恭喜您赢了!");
                break;
            },
        }
    }
}

fn main() {
    print!("1 猜数字游戏\n2 石头剪头布\n");
    let mut game_type = String::new();
    io::stdin().read_line(&mut game_type).expect("输入错误");
    let start_type:u32 = match game_type.trim().parse() {
        Ok(t) => t,
        Err(_) => {
            println!("{} 输入游戏类型不对.",game_type.trim());
            0
        },
    };

    if start_type == 1 {
        guess_number();
    }else if start_type == 2 {
        rock_paper_shotgun();
    }
}