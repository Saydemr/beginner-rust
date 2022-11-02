// Project 1: Interactive bill manager
//
// User stories:
// * L1: I want to add bills, including the name and amount owed.
// * L1: I want to view existing bills.
// * L2: I want to remove bills.
// * L3: I want to edit existing bills.
// * L3: I want to go back if I change my mind.
//
// Tips:
// * Use the loop keyword to create an interactive menu.
// * Each menu choice should be it's own function, so you can work on the
//   the functionality for that menu in isolation.
// * A vector is the easiest way to store the bills at level 1, but a
//   hashmap will be easier to work with at levels 2 and 3.
// * Create a function just for retrieving user input, and reuse it
//   throughout your program.
// * Create your program starting at level 1. Once finished, advance to the
//   next level.
use std::collections::HashMap;

struct Bill {
    name: String,
    amount: f32,
}

struct Bills {
    bills: HashMap<String, f32>,
}

impl Bills {
    fn new() -> Self {
        Self { bills: HashMap::new() }
    }

    fn add(&mut self, bill: Bill) {
        self.bills.insert(bill.name, bill.amount);
    }

    fn view(&self) {
        for (name, amount) in &self.bills {
            println!("{}: ${}", name, amount);
        }
    }

    fn remove(&mut self, name: String) -> bool {
        self.bills.remove(&name).is_some()
    }
}

fn add_bills_menu(bills: &mut Bills) {
    println!("Enter the name of the bill:");
    let name = get_input();

    println!("Enter the amount of the bill:");
    let amount = get_input().parse::<f32>().unwrap();

    bills.add(Bill { name, amount });
    println!("Bill added!");
}

fn view_bills_menu(bills: &Bills) {
    bills.view();
}

fn remove_bills_menu(bills: &mut Bills) {
    println!("Enter the name of the bill to remove:");
    let name = get_input();

    let check = bills.remove(name);
    if check {
        println!("Bill removed!");
    } else {
        println!("Bill not found!");
    }
}

fn edit_bills_menu(bills: &mut Bills) {
    println!("Enter the name of the bill to edit:");
    let name = get_input();

    println!("Enter the new amount of the bill:");
    let amount = get_input().parse::<f32>().unwrap();

    let mut checky = bills.remove(name).is_some();
    if checky {
        bills.add(Bill { name, amount });
        println!("Bill edited!");
    } else {
        println!("Bill not found!");
    }
}


fn get_input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_owned()
}

fn main_menu() {
    fn show() {
        println!("
        1. Add bill
        2. View bills
        3. Remove bill
        4. Edit bill
        5. Exit
        ");
    }

    let mut bills = Bills::new();

    loop {
        show();
        let input = get_input();
        match input.as_str() {
            "1" => add_bills_menu(&mut bills),
            "2" => view_bills_menu(&bills),
            "3" => remove_bills_menu(&mut bills),
            "4" => edit_bills_menu(&mut bills),
            "5" => break,
            _ => println!("Invalid input"),
        }
    }
}

fn main()
{
    main_menu();
}
