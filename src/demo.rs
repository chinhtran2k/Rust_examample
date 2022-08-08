use core::panic;
use std::{fs::File, io::ErrorKind};

use rand::Error;

pub fn demo_example(){
    let file = File::open("D:/sách/hello.txt");

    let f = match file {
        Ok(f) => f,
        Err(error) => match error.kind(){
            ErrorKind::NotFound => match File::create("D:/sách/hello.txt"){
                Ok(new_file) => new_file,
                Err(error) => panic!("Error at creating new file {:?}",error)
            },
            other_error => panic!("{:?}",other_error)          
        }
    };

    let ff = File::open("D:/sách/hello.txt").unwrap_or_else(|error|{
        if error.kind() == ErrorKind::NotFound{
            File::create("D:/sách/hello.txt").unwrap_or_else(|error|{
                panic!("problem crating the file : {:?}",error );
            })
        }
        else{
            panic!("problem openning the file: {:?}",error);
        }
    });

    let p1 = Point::default();
    p1.x = 100;
    let p2 = p1;
    // let mut s = String::new();
    // match f.read_to_string(&mut s){
    //     Ok(_) => Ok(s),
    //     Err => Err(e)
    // }
    // let student_a = Student{
    //     name : "tran trong chinh".to_string(),
    //     age : 22,
    //     class : "VMO".to_string()
    // };
    // student_a.get_student();

    // let r;
    // let x =5;
    // r = &x; 
    // println!("{}",r);

    // let String1 = String::from("abcd");
    // let String2 = "xyz";
    // let result = longest( &String1, String2);
    // println!("{}",result);

    // let counter = Counter {x :0};
    // let x =counter.next();
       // print!("{}",x);
    

}

mod  module_inline{
    pub struct A{}
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
struct Point {
    x : u32,
    y : u32,
}

// fn read_file_normal_short() -> Result<String, Error>{
//     let path = "hello.txt";
//     let f = File::open(path)?;
//     let mut s = String::new();
//     f.read_to_string(&mut s)?;
// }

// struct Container(i32 , i32);
// trait  Contains <A, B>{
//     fn contains (&self,_: &A , _:&B)->bool;
//     fn first(&self) -> i32;
//     fn last(&self) -> i32;
// }

// impl Contains<i32, i32> for Container{
//     fn contains(&self ,x: &i32 ,y: &i32)-> bool{
//         (&self.0 == x) && (&self.1 == y)
//     }
//     fn first(&self) -> i32 {
//         self.0
//     }
//     fn last(&self) -> i32 {
//         self.1
//     }
// }

// use std::ops::AddAssign;
// pub trait Iterator<T> {
//     fn next(&mut self) -> T;
// }
// struct  Counter<T> {
//     x : T,
//     y :T 
// }
// impl<T:AddAssign + Copy + Clone> Iterator<T> for Counter<T> {
//     fn next(&mut self) -> T{
//     self.x += self.y;
//     self.x
//     }
// }


// fn longest<'a>(x: &'a str , y: &'a str)-> &'a str {
//     if x.len() > y.len(){
//         x
//     }
//     else{
//         y
//     }
// }
// struct Student{
//     name : String,
//     age : u8,
//     class : String,
// }

// impl Student{
//     fn  get_student(self){
//         println!("Name: {} ,age: {}, class: {}.",self.name,self.age,self.class)
//     }
// } 
// pub use std::ops::Add;