pub fn demo_example(){
    let student_a = Student{
        name : "tran trong chinh".to_string(),
        age : 22,
        class : "VMO".to_string()
    };
    student_a.get_student();
}
struct Student{
    name : String,
    age : u8,
    class : String,
}

impl Student{
    fn  get_student(self){
        println!("Name: {} ,age: {}, class: {}.",self.name,self.age,self.class)
    }
}