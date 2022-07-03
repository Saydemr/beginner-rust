// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

struct Student {
    name : String,
    locker_id : Option<i32>
}

impl Student {
    fn print(&self)
    {
        println!("Name : {:?}", self.name);
        match self.locker_id {
            Some(number) => println!("Locker number : {:?}", number),
            None   => println!("No locker assignment.")
        }
    }
}


fn main() 
{
    let student1 = Student { 
        name : "devrim".to_owned(),
        locker_id : Some(321)
    };

    student1.print();

    let student2 = Student { 
        name : "dua".to_owned(),
        locker_id : None
    };

    student2.print();
}
