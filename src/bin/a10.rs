// Topic: Working with expressions
//
// Requirements:
// * Print "its big" if a variable is > 100
// * Print "its small" if a variable is <= 100
//
// Notes:
// * Use a boolean variable set to the result of
//   an if..else expression to store whether the value
//   is > 100 or <= 100
// * Use a function to print the messages
// * Use a match expression to determine which message
//   to print

fn print_var_info(lte_100 : bool)
{
    match lte_100 {
        false => println!("It's big!"),
        true => println!("It's small!")
    }
}

fn main() 
{
    let my_var : i32 = 100;
    let lte_100 = my_var <= 100;
    print_var_info(lte_100);
    
}
