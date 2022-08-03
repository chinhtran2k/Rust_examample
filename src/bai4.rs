use std::collections::HashMap;
pub fn School(){
    let mut student_one = School{ student: HashMap::new()};
    student_one.add_student("Alice".to_string(),7);
    student_one.add_student("Bob".to_string(), 2);
    println!("điểm {:?}",student_one);
}   
#[derive(Debug)]
struct School{
    student : HashMap<String, u32>
}
impl School {
    fn add_student(mut self,name:String,score:u32){
        self.student.insert(name,score).unwrap();
    }
}