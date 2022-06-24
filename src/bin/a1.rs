// Topic: Functions
//
// Program requirements:
// * Displays your first and last name
//
// Notes:
// * Use a function to display your first name
// * Use a function to display your last name
// * Use the println macro to display messages to the terminal

fn main() 
{
    let fname = "asdas";
    let lastname = "vazavaz";
    display_first_name(fname.to_string());
    display_last_name(lastname.to_string());
}

fn display_first_name(first_name: String)
{
    println!("{:?}", first_name)
}


fn display_last_name(last_name: String)
{
    println!("{:?}", last_name)
}
