use std::io;
pub fn example_one(){
    let str = "tran trong chinh";
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut i = 0;
    for idx in str.chars(){
        if input.trim() == idx.to_string() {
            i = i +1;
        }
    }
    println!("so lan {} xuat hien trong {} la {} lan",input,str,i);
}