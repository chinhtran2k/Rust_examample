use std::{collections::HashMap};
pub fn example_four(){
    let mut student_one = School{ student: HashMap::new()};
    student_one.add_student("Alice".to_string(),3);
    student_one.add_student("Bob".to_string(), 10);
    student_one.add_student("charlie".to_string(), 3);
    println!("điểm {:?}",student_one);
    student_one.check_score();
    student_one.check_student_alike_score(3)
}   
#[derive(Debug)]
struct School{
    student : HashMap<String, u32>
}
impl School {
    pub fn add_student(&mut self,name:String,score:u32){
        self.student.insert(name,score);
    }
    pub fn check_score(&mut self){
        let mut scores = vec![];
        for (key, value) in self.student.iter(){
            scores.push(value);
        }
        scores.sort();
        for item in 0..scores.len(){
            if scores[item] == scores[item+1]{
                scores.remove(item);
                break;
            }
        }
        println!("{:?}",scores);
        //scores.dedup();
        }
    pub fn check_student_alike_score(&mut self, grades: u32){
        let mut student_name = vec![];
        for (key, value) in self.student.iter(){
            if *value == grades{
                student_name.push(key.to_string())
            }
            }
        println!("{:?}",student_name)
    }
}
    