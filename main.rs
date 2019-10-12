#[derive(Debug)]
//use Std::io

//blue print of a sturct
struct Student {
    name : String,
    roll_num : String,
    course : String,
    dis_learning : bool,
    city : String,
    center : String,
    campus : String,
    quarter : String,
    day_time :String,
    batch : i16
}

//function returning instance of struct (associated function)
fn Build_Student (name : String,roll_num : String,dis_learning : bool)->Student{
    Student {
        name,
        roll_num,
        course : "AIOT".to_string(),
        dis_learning,
        city : "Karachi".to_string(),
        center : "Bahria Audutorium".to_string(),
        campus : "Karsaz".to_string(),
        quarter : "Q1".to_string(),
        day_time :"monday,12-3-2019".to_string(),
        batch : 2    }
}

fn main(){
    let student_01 = Student{
        name: String::from("osama qamar"),
        roll_num: String::from("iot04637"),
        dis_learning: false,
        course : "AIOT".to_string(),
        city : "Karachi".to_string(),
        center : "Bahria Audutorium".to_string(),
        campus : "Karsaz".to_string(),
        quarter : "Q1".to_string(),
        day_time :"monday,12-3-2019".to_string(),
        batch : 2
    };
    println!("{:#?}",student_01);




}
