
#[allow(dead_code)]
#[derive(PartialEq, PartialOrd)]  enum Access {
    Admin,
    Manager,
    User,
    Guest
}

fn main()
{
    let my_value = Access::User;
    if my_value > Access::Manager {
        println!("It works!");
    }
}
