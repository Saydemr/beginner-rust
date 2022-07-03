// Topic: Result & the question mark operator
//
// Requirements:
// * Determine if an employee can access a building using a digital keycard
// * Employees that can access the building are:
//   * Maintenance crews
//   * Marketing department employees
//   * Managers
// * Other employees that work at the company are:
//   * Line supervisors
//   * Kitchen staff
//   * Assembly technicians
// * Ensure that terminated employees cannot access the building
//   regardless of their position
//
// Notes:
// * Use an enum to represent all types of employees
// * Use a struct to store the employee type and whether they are
//   still employed
// * Use a function that returns a Result to determine if the employee
//   may enter the building
// * Print whether the employee may access the building
//   * Must use a function that utilizes the question mark operator to do this

#![allow(dead_code)]

enum Position {
    Maintenance,
    Marketing,
    Manager,
    Line,
    Kitchen,
    Technician,
}

enum Status {
    Active,
    Terminated
}

struct Employee {
    employee_type : Position,
    is_employed : Status
}


fn can_access(employee: &Employee) -> Result<(), String>
{
    match employee.is_employed {
        Status::Terminated => return Err("Not a current employee".to_owned()),
        _                  => ()
    }

    match employee.employee_type {
        Position::Maintenance => return Ok(()),
        Position::Marketing   => return Ok(()),
        Position::Manager     => return Ok(()),
        _                     => return Err("Invalid position".to_owned())
    }
}


fn try_access(employee : &Employee) -> Result<(), String>
{
    let _access = can_access(&employee)?;
    println!("Access granted.");
    Ok(())
}

fn main() 
{
    let employee = Employee { employee_type : Position::Kitchen, is_employed : Status::Active };
    match try_access(&employee) {
        Err(e) => println!("Cannot enter the building : {:?}", e),
        _      => () 
    }
}
