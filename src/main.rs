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
