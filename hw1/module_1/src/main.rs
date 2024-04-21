WeChat: cstutorcs
QQ: 749389476
Email: tutorcs@163.com
use solution::{is_leap_year, hello, plus_one, increase};

fn main() {
    println!("{}", hello());    
    println!("Is 1900 a leap year? {}", is_leap_year(1900));
    println!("Is 2020 a leap year? {}", is_leap_year(2020));

    let mut n: i32 = 9;
    println!("n = {}", n);
    plus_one(&mut n);
    println!("plus_one(9) = {}", n);

    let mut n: i32 = 9;
    println!("n = {}", n);
    increase(&mut n, 3);
    println!("increase(9, 3) = {}", n); 
}
