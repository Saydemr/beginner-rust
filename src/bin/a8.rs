// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

enum Flavor {
    Yummy,
    Salty,
    Sour
}

struct Drink {
    flavor: Flavor,
    ounce : i32
}

fn print_drink(drink : Drink)
{
    println!("Ounce of the drink is : {:?}", drink.ounce);
    match drink.flavor {
        Flavor::Yummy => println!("Flavor is Yummy"),
        Flavor::Salty => println!("Flavor is Salty"),
        Flavor::Sour => println!("Flavor is Sour")
    }
}

fn main() 
{
    let my_drink = Drink {
        flavor : Flavor::Yummy,
        ounce  : 15
    };
    print_drink(my_drink);
}
