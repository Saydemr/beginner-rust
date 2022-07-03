// Topic: Result
//
// Requirements:
// * Determine if a customer is able to make a restricted purchase
// * Restricted purchases require that the age of the customer
//   is at least 21
//
// Notes:
// * Use a struct to store at least the age of a customer
// * Use a function to determine if a customer can make a restricted purchase
// * Return a result from the function
// * The Err variant should detail the reason why they cannot make a purchase

#[derive(Debug)]
struct Customer {
    age : i8
}

fn try_purchase(customer : &Customer) -> Result<(), String>
{
    if customer.age < 21 {
        return Err("User cannot purchase restricted goods".to_owned());
    }
    Ok(())
}

fn main() 
{
    let customer1 = Customer { age : 17 };
    let result = try_purchase(&customer1);
    println!("{:?}",result);
}
