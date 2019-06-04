// sec6 基本構文
// 入力された年がうるう年が判断するプログラム
use std::io;
use std::io::Write;

fn main() {
    // 行コメント
    /* ブロック/* コメント */ */
    let mut year = String::new();
    print!("please input a year to check if it is a leap year: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut year).unwrap();
    let year = year.trim().parse::<u32>().unwrap();

    if is_leap_year(year) {
        println!("{} is a leap year.", year);
    } else {
        println!("{} is not a leap year.", year);
    }
    // 6-5
    let circle1 = Circle::small_circle();
    println!("Circle diameter: {}", circle1.diameter());
    let a = "aaaaaaee";
    let mut b = "aefeae";
    //a = "fefea";
    b = "ccccc";
    // 6-6-3 スコープ
    // スコープは目新しいことない
    // 6-6-4 シャドウイング
    // できるよ、というはなし
    let a = 1;
    let a = 1;
    // 6-6-5 定数とスタティック変数
    const SECRET: i32 = 111;
    static STTC:i32 = 222;
    // 6-7 演算子
    //let ans3 = 70 + 8.9; // error
    let ans3 = 70.0 + 8.9;
    // 6-8 分岐
    let a = 11;
    if a % 2 == 0 {
        println!("{} is an even number", a);
    } else {
        println!("odd");
    }
    let even_or_odd = if a % 2 == 0 { "even" } else { "odd" };
    println!("{}", even_or_odd);
    // 6-8-2 match
    let val = 100;
    match val {
        1 => println!("One"),
        100 => println!("100"),
        _ => println!("something else"),
    }
    let mm = match val {
        1 => "One",
        100 => "100",
        _ => "something else",
    };
    let unknown = Some("Apple");
    let string = match unknown {
        Some(something) => String::from("Hi, ") + something,
        None => String::from("Nothing"),
    };
    let ten = 10;
    let ten_ref = &ten;
    match ten_ref {
        number => assert_eq!(&10, number),
    }
    match ten_ref {
        &number => assert_eq!(10, number),
    }
    let string = Some("aaaaaaaaaaaaaaaaaaa");
    let m = match string {
        Some(s) if s.len() > 10 => "Long String",
        Some(_) => "String",
        None => "Nothing"
    };
    // 6-8-3 if let式
    // swift の if let にいっしょかな
    // 6-9 繰り返し
    // 6-0-1 loop
    let mut c = 0;
    loop {
        println!("LOOOOOP");
        if c > 10 {
            break;
        }
        c += 1;
    }
    c = 0;
    let ten = loop {
        if c == 0 {
            break c;
        }
        c += 1;
    };
    // 6-9-2 while
    // 6-9-3 while let わからん
    let mut c = Some(0);
    // conterがSome(i)の場合だけループってこと？
    while let Some(i) = c {
        if i == 10 {
            c = None;
        } else {
            println!("{}" i);
            c = Some(i + 1);
        }
    }
    // 6-9-4 for 
    let vec  = vec!["cyan", "m", "Y"];
    for v in vec.iter() {
        println!("{} " , v);
    }
    

    
}

fn is_leap_year(year:u32) -> bool {
    year % 4 == 0 && !(year % 100 == 0 && year % 400 != 0)
}

struct Circle {
    radius: u32,
}
impl Circle {
    fn diameter(&self) -> u32 {
        self.radius * 2
    }
    fn small_circle() -> Circle {
        Circle{radius:1}
    }
}
