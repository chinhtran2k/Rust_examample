use std::io;
mod demo;
mod bai1;
mod bai2;
mod bai3;
mod bai4;
mod bai5;
fn main() {
    let mut example = String::new();
    println!("you like to see example ??");
    io::stdin().read_line(&mut example).unwrap();
    let example_number = example.trim().parse::<i32>().unwrap();
    if example_number == 1 {
        bai1::example_one();
    }
    else if example_number == 2 {
        bai2::example_two();
    }
    else if example_number == 3 {
        bai3::err_ownership();
    }
    else if example_number == 4 {
        bai4::example_four();
    }
    else if example_number == 5{
        bai5::Example_five();
    }
    else if example_number == 6{
        demo::demo_example();
    }
    else {
        println!("number is fail!!!")
    }
}