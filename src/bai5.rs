use std::fmt;

pub fn Example_five(){
    for number in fibonacci_number(){
        println!("{}",number);
    }
}
#[derive(Debug)]
struct Fibonacci{
    a : u32,
    b: u32,
}

impl Iterator for Fibonacci {
    type Item = u32;
    fn next(&mut self) -> Option<u32>{
        //checked_mul , saturating_add
        let mut res = self.a.checked_add(self.b)? ;
        self.b =self.a;
        self.a = res;
        Some(res)
    }
}
fn fibonacci_number() -> Fibonacci{
    Fibonacci { a:1 ,b:0}
}

struct  StrDisplayable<'a>(Vec<&'a str>);

impl<'a> fmt::Display for StrDisplayable<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        for  v in &self.0{
            write!(f,"\n{}",v)?;
        }
        Ok(())
    }
}