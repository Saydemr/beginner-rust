// Topic: User input
//
// Requirements:
// * Verify user input against pre-defined keywords
// * The keywords represent possible power options for a computer:
//   * Off
//   * Sleep
//   * Reboot
//   * Shutdown
//   * Hibernate
// * If the user enters one of the keywords, a message should be printed to
//   the console indicating which action will be taken
//   * Example: If the user types in "shutdown" a message should display such
//     as "shutting down"
// * If the keyword entered does not exist, an appropriate error message
//   should be displayed
//
// Notes:
// * Use an enum to store the possible power states
// * Use a function with a match expression to print out the power messages
//   * The function should accept the enum as an input
// * Use a match expression to convert the user input into the power state enum
// * The program should be case-insensitive (the user should be able to type
//   Reboot, reboot, REBOOT, etc.)
use std::io;

enum State {
    Off,
    Sleep,
    Reboot,
    Shutdown,
    Hibernate
}

impl State {
    fn new(state : &str) -> Option<State> {
        let state = state.trim().to_lowercase();
        match state.as_str() {
            "off"       => Some(State::Off),
            "sleep"     => Some(State::Sleep),
            "reboot"    => Some(State::Reboot),
            "shutdown"  => Some(State::Shutdown),
            "hibernate" => Some(State::Hibernate),
            _           => None
        }
    }
}

fn get_input() -> io::Result<String>
{
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim().to_owned())
}

fn print(state : &State) 
{
    use State::*;
    match state {
        Off       => println!("Turned off"),
        Sleep     => println!("Putting to sleep"),
        Reboot    => println!("Rebooting"),
        Shutdown  => println!("Shutting down"),
        Hibernate => println!("Hibernating"),
    }
}

fn main()
{
    let user_input = get_input();
    let state = State::new(&user_input);
    if state != None {
        match &state {
            Option::Status => println!("{:?}", status),
            _              => println!("Yeaak"),
        }
    }
}
