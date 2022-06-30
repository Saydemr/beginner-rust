// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

struct Person {
    age : i8,
    favorite_color : String,
    name : String
}

impl Person {
    fn print_info(&self) {
        println!("Name : {}\nAge : {}\nFavorite Color : {}", self.name, self.age, self.favorite_color)
    }
}

fn main() 
{
    let person_vector = vec![
        Person {age : 10, favorite_color : "red".to_owned(), name : "duygu".to_owned()},
        Person {age : 12, favorite_color : "black".to_owned(), name : "didem".to_owned()},
        Person {age : 14, favorite_color : "blue".to_owned(), name : "rüya".to_owned()}
    ];

    for person in person_vector {
        if person.age <= 10 {
            person.print_info();
        }
    }
}
